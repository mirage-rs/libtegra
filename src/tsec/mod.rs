//! Driver for interfacing with the Tegra Security Co-Processor (TSEC).
//!
//! # Description
//!
//! The TSEC is a dedicated unit powered by a NVIDIA Falcon microprocessor with
//! crypto extensions. Its purpose is to leverage certain cryptographic tasks
//! done by firmwares signed by NVIDIA into a secure space that cannot be taken
//! over by the host system.
//!
//! It has three operating modes:
//!
//! - No Secure Mode (NS): Every piece of microcode that is not cryptographically
//! signed by NVIDIA will be executed in this mode. It prevents you from accessing
//! certain registers and may disable physical memory access from code.
//!
//! - Light Secure Mode (LS): In this mode, the Falcon has more privileges than in
//! No Secure Mode, but fewer than in Heavy Secure Mode. This mode leaks some of
//! the internal state to ease up debugging and can only be enabled from Heavy
//! Secure Mode microcode.
//!
//! - Heavy Secure Mode (HS): This mode can be entered by uploading signed microcode
//! and grants the full range of privileges to the microcode. This state essentially
//! turns the Falcon into a black box that doesn't expose any of its inner workings
//! to, for example, the host system.
//!
//! # Firmware
//!
//! [envytools] have proven to be valuable tools when it comes to working with
//! various ISAs used by NVIDIA, including the Falcon processor.
//!
//! The following examples will use this reference firmware:
//!
//! ```asm
//! mov $r15 0xB0B0B0B0;
//! mov $r9 0x1100;
//! iowr I[$r9] $r15;
//! exit;
//! ```
//!
//! It can be assembled with the following command, assuming
//! the code is stored in a file called `faucon.asm`:
//!
//! ```shell
//! envyas -m falcon -V fuc5 -F crypt faucon.asm -i -o faucon.bin
//! ```
//!
//! # Firmware alignment
//!
//! Firmware blobs that should be booted on the [`Tsec`] are supposed to
//! be aligned to the boundary denoted by [`FIRMWARE_ALIGNMENT`]. This is
//! implied by the Falcon code segment, which consists of 0x100 byte pages.
//! An approach to getting this correct could be:
//!
//! ```
//! use libtegra::tsec::{Firmware, FIRMWARE_ALIGNMENT};
//!
//! /// The firmware blob.
//! static FW: Firmware<u8, 13> = Firmware::new([
//!     0xDF, 0xB0, 0xB0, 0xB0, 0xB0,   // mov $r15 0xB0B0B0B0;
//!     0x49, 0x00, 0x11,               // mov $r9 0x1100;
//!     0xF6, 0x9F, 0x00,               // iowr I[$r9] $r15;
//!     0xF8, 0x02,                     // exit;
//! ]);
//!
//! assert_eq!(&*FW as *const u8 as usize % FIRMWARE_ALIGNMENT, 0);
//! ```
//!
//! # Executing code
//!
//! Using our firmware blob and the static variable `FW` from above,
//! we can load the code onto the TSEC and try to execute it:
//!
//! ```no_run
//! use libtegra::tsec::{Firmware, Tsec};
//!
//! static FW: Firmware<u8, 13> = Firmware::new([
//!     0xDF, 0xB0, 0xB0, 0xB0, 0xB0,   // mov $r15 0xB0B0B0B0;
//!     0x49, 0x0, 0x11,                // mov $r9 0x1100;
//!     0xF6, 0x9F, 0x0,                // iowr I[$r9] $r15;
//!     0xF8, 0x2,                      // exit;
//! ]);
//!
//! // Load our firmware onto the TSEC.
//! Tsec::A.load_firmware(&*FW).unwrap();
//!
//! // Boot it up!
//! let mut mailbox0 = 0;
//! let mut mailbox1 = 0;
//! unsafe {
//!     Tsec::A.boot(0, &mut mailbox0, &mut mailbox1).unwrap();
//!     assert_eq!(mailbox1, 0xB0B0B0B0);
//! }
//! ```
//!
//! [NVIDIA Falcon]: https://envytools.readthedocs.io/en/latest/hw/falcon/index.html
//! [envytools]: https://github.com/envytools/envytools
//! [`Tsec`]: struct.Tsec.html
//! [`FIRMWARE_ALIGNMENT`]: constant.FIRMWARE_ALIGNMENT.html

mod registers;

use core::ops::{Deref, DerefMut};

use enum_primitive::FromPrimitive;

use crate::car::Clock;
use crate::kfuse;
use crate::timer::get_milliseconds;
pub use crate::tsec::registers::*;

/// The alignment bits for TSEC firmware blobs.
pub const FIRMWARE_ALIGN_BITS: usize = 8;
/// The alignment a TSEC firmware blob is expected to have.
pub const FIRMWARE_ALIGNMENT: usize = 1 << FIRMWARE_ALIGN_BITS;

/// A helper structure to align arrays containing Falcon machine code to the expected
/// 0x100 bytes memory alignment for DMA transfers into the code segment.
#[repr(align(256))]
#[derive(Clone, Copy)]
pub struct Firmware<T, const N: usize>([T; N]);

impl<T, const N: usize> Firmware<T, { N }> {
    /// Aligns the given firmware buffer to the required Falcon code page alignment.
    pub const fn new(firmware: [T; N]) -> Self {
        Firmware(firmware)
    }

    /// Returns the inner data.
    pub fn into_inner(self) -> [T; N] {
        self.0
    }
}

impl<T, const N: usize> Deref for Firmware<T, { N }> {
    type Target = [T; N];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T, const N: usize> DerefMut for Firmware<T, { N }> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<T, const N: usize> From<[T; N]> for Firmware<T, { N }> {
    fn from(t: [T; N]) -> Self {
        Firmware::new(t)
    }
}

enum_from_primitive! {
    /// Enumeration of potential Falcon processor exception clauses
    /// that may occur during code execution on the TSEC.
    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    #[repr(u8)]
    pub enum FalconExceptionClause {
        /// The software trap 0 that may be triggered by the TRAP instruction.
        Trap0 = 0,
        /// The software trap 1 that may be triggered by the TRAP instruction.
        Trap1 = 1,
        /// The software trap 2 that may be triggered by the TRAP instruction.
        Trap2 = 2,
        /// The software trap 3 that may be triggered by the TRAP instruction.
        Trap3 = 3,
        /// A trap that is triggered when the Falcon encounters an opcode it
        /// cannot decode.
        InvalidOpcode = 8,
        /// A so-called Secure Fault was indicated by jumping to a secure page,
        /// but the MAC verification for that page failed, which halted the MCU.
        AuthenticationEntry = 9,
        /// A page fault occurred because the TLB failed to provide mappings for
        /// a virtual address on lookup.
        PageMiss = 10,
        /// A page fault occurred because the TLB contained multiple mappings for
        /// a single virtual address on lookup.
        PageMultipleMiss = 11,
        /// A breakpoint, which was set through the integrated hardware debugging
        /// interface, was hit during code execution.
        BreakpointHit = 15,
    }
}

impl From<u8> for FalconExceptionClause {
    fn from(exception_clause: u8) -> FalconExceptionClause {
        match FalconExceptionClause::from_u8(exception_clause) {
            Some(result) => result,
            None => panic!("Got unexpected exception clause: {}", exception_clause),
        }
    }
}

assert_eq_size!(FalconExceptionClause, u8);

/// Falcon processor exceptions that may occur when interacting with it from the
/// host system or may be caused by running code.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FalconError {
    /// The DMA engine timed out.
    DmaTimeout,
    /// The firmware blob is misaligned.
    FirmwareMisaligned,
    /// A Falcon exception that occurred during execution, consisting of the Program
    /// Counter where execution stopped and a [`FalconExceptionClause`] which
    /// provides additional context.
    ///
    /// [`FalconExceptionClause`]: enum.FalconExceptionClause.html
    Exception(u32, FalconExceptionClause),
}

/// Representation of the Tegra Security Co-Processor.
#[derive(Debug)]
pub struct Tsec {
    // A pointer to the TSEC register block in memory.
    registers: *const Registers,
}

// Definitions of known TSEC instances.
impl Tsec {
    /// Representation of the TSEC-A instance.
    pub const A: Self = Tsec {
        registers: TSEC_A_REGISTERS,
    };

    /// Representation of the TSEC-B instance.
    pub const B: Self = Tsec {
        registers: TSEC_B_REGISTERS,
    };
}

impl Tsec {
    /// Initializes the TSEC for use.
    ///
    /// NOTE: This method must be called once before the TSEC is usable.
    /// Otherwise, the SoC will hang itself whenever the device is accessed.
    pub fn init(&self) {
        let tsec = unsafe { &*self.registers };

        // Enable the device clocks that are required by the TSEC.
        Clock::HOST1X.enable();
        Clock::TSEC.enable();
        Clock::TSECB.enable();
        Clock::SOR_SAFE.enable();
        Clock::SOR0.enable();
        Clock::SOR1.enable();
        Clock::KFUSE.enable();

        // Ensure that KFUSE is ready (since TSEC sources the KFUSE key from it).
        kfuse::wait_until_ready().unwrap();

        // Configure the Falcon processor.
        tsec.FALCON_DMACTL.set(0);
        tsec.FALCON_IRQMSET.set(0xFFF2);
        tsec.FALCON_IRQDEST.set(0xFFF0);
        tsec.FALCON_ITFEN.set(3);
    }

    /// Shuts the TSEC down and makes it inaccessible.
    pub fn finalize(&self) {
        // Disable all device clocks for TSEC.
        Clock::KFUSE.disable();
        Clock::SOR1.disable();
        Clock::SOR0.disable();
        Clock::SOR_SAFE.disable();
        Clock::TSECB.disable();
        Clock::TSEC.disable();
        Clock::HOST1X.disable();
    }

    fn dma_wait_idle(&self) -> Result<(), FalconError> {
        let tsec = unsafe { &*self.registers };

        let timeout = get_milliseconds() + 10_000;

        while (tsec.FALCON_DMATRFCMD.get() & (1 << 1)) == 0 {
            if get_milliseconds() > timeout {
                return Err(FalconError::DmaTimeout);
            }
        }

        Ok(())
    }

    /// Loads Falcon microcode into the processor memory.
    ///
    /// This method utilizes the Falcon DMA engine to load the given firmware
    /// into the code segment, starting from physical and virtual address `0`
    /// and must be separately executed using [`Tsec::boot`].
    ///
    /// NOTE: The firmware buffer is expected to be [aligned] correctly
    /// to the boundaries of 0x100 byte pages in order to be uploaded.
    ///
    /// [aligned]: constant.FIRMWARE_ALIGNMENT.html
    /// [`Tsec::boot`]: #method.boot
    pub fn load_firmware(&self, firmware: &[u8]) -> Result<(), FalconError> {
        let tsec = unsafe { &*self.registers };

        // Check if the firmware is being aligned correctly.
        let firmware_address = firmware.as_ptr() as usize;
        if (firmware_address % FIRMWARE_ALIGNMENT) != 0 {
            return Err(FalconError::FirmwareMisaligned);
        }

        // Make sure the DMA engine is in idle state.
        self.dma_wait_idle()?;

        // Load in the memory base address of the firmware buffer.
        tsec.FALCON_DMATRFBASE
            .set((firmware_address >> FIRMWARE_ALIGN_BITS) as u32);

        // Configure the DMA engine to transfer the firmware buffer into the Falcon IMEM.
        for (index, _) in firmware.chunks(FIRMWARE_ALIGNMENT).enumerate() {
            let base = (index * FIRMWARE_ALIGNMENT) as u32;
            let offset = base;

            tsec.FALCON_DMATRFMOFFS.set(offset);
            tsec.FALCON_DMATRFFBOFFS.set(base);
            tsec.FALCON_DMATRFCMD.set(1 << 4);

            self.dma_wait_idle()?;
        }

        Ok(())
    }

    /// Boots the Falcon from the specified boot vector.
    ///
    /// The firmware must have been loaded into the Falcon in advance, either by
    /// calling [`Tsec::load_firmware`] or doing the necessary transfers manually.
    /// The boot vector then specifies from where code should be executed, most code
    /// blobs presumably expect `0` to be passed, and the CPU boots up.
    ///
    /// There is also support for both shared mailboxes which act as scratch registers
    /// to share data between the Falcon and the host system. Through the respective
    /// arguments, mailboxes can be filled with supplied values and at the end of
    /// execution, the variables will be overridden with the final state of the TSEC
    /// mailboxes (e.g. to check result codes on the host processor).
    ///
    /// # Safety
    ///
    /// This method is considered unsafe because code execution on the TSEC can fail
    /// for malformed or misaligned blobs or through code fucking up internal state.
    ///
    /// [`Tsec::load_firmware`]: #method.load_firmware
    pub unsafe fn boot(
        &self,
        boot_vector: u32,
        mailbox0: &mut u32,
        mailbox1: &mut u32,
    ) -> Result<(), FalconError> {
        let tsec = &*self.registers;
        let mut res;

        // Configure Falcon and start the CPU.
        tsec.FALCON_MAILBOX0.set(*mailbox0);
        tsec.FALCON_MAILBOX1.set(*mailbox1);
        tsec.FALCON_BOOTVEC.set(boot_vector);
        tsec.FALCON_CPUCTL.set(1 << 1);

        // Wait for the DMA engine to enter idle state.
        res = self.dma_wait_idle();
        if res.is_ok() {
            // Wait for the CPU to be halted.
            while tsec.FALCON_CPUCTL.get() != (1 << 4) {}

            // Check if the CPU has crashed.
            let exception_info = tsec.FALCON_EXCI.get();
            if exception_info != 0 {
                // Gather exception details.
                let pc = exception_info & 0xFFFFF;
                let exception = FalconExceptionClause::from(((exception_info >> 20) & 0xF) as u8);

                res = Err(FalconError::Exception(pc, exception));
            }
        }

        *mailbox0 = tsec.FALCON_MAILBOX0.get();
        *mailbox1 = tsec.FALCON_MAILBOX1.get();

        res
    }

    /// Dumps the DMEM of the Falcon engine into the supplied buffer.
    ///
    /// This function is useful for debugging purposes and to examine how firmware
    /// interacts with the data memory.
    ///
    /// The DMEM size of TSEC is hardcoded to `0x4000` bytes. For other Falcon
    /// engines, the size of DMEM in words should be determined by reading
    /// `(FALCON_HWCFG >> 9 & 0x1F) << 6`.
    ///
    /// NOTE: This is only usable while TSEC is in No Secure mode context.
    pub fn dump_dmem(&self, output: &mut [u32; 0x1000]) {
        let tsec = unsafe { &*self.registers };

        // Configure a full dump of DMEM with auto-incrementing addresses.
        tsec.FALCON_DMEMC0.set((0 << 2) | (1 << 25));

        // Read all words of the DMEM into the output buffer.
        for i in output.iter_mut() {
            *i = tsec.FALCON_DMEMD0.get();
        }
    }
}
