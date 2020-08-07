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
//! ## Firmware
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
//! ## Firmware Alignment
//!
//! Firmware blobs that should be booted on the [`Tsec`] are supposed to
//! be aligned to the boundary denoted by [`FIRMWARE_ALIGNMENT`]. This is
//! implied by the Falcon code segment, which consists of 0x100 byte pages.
//! An approach to getting this correct could be:
//!
//! ```
//! use libtegra::tsec::FIRMWARE_ALIGNMENT;
//!
//! /// A helper that wraps a TSEC firmware blob to ensure correct alignment.
//! #[repr(align(256))]
//! struct Firmware<T: Sized> {
//!     pub value: T,
//! }
//!
//! impl<T: Sized> Firmware<T> {
//!     pub const fn new(value: T) -> Self {
//!         Firmware { value }
//!     }
//! }
//!
//! /// The firmware blob.
//! static FAUCON: Firmware<[u8; 13]> = Firmware::new([
//!     0xDF, 0xB0, 0xB0, 0xB0, 0xB0,   // mov $r15 0xB0B0B0B0;
//!     0x49, 0x00, 0x11,               // mov $r9 0x1100;
//!     0xF6, 0x9F, 0x00,               // iowr I[$r9] $r15;
//!     0xF8, 0x02,                     // exit;
//! ]);
//!
//! assert_eq!(FAUCON.value.as_ptr() as usize % FIRMWARE_ALIGNMENT, 0);
//! ```
//!
//! ## Code Execution
//!
//! Using our firmware blob and the static variable `FAUCON` from above,
//! we can load the code onto the TSEC and try execute it:
//!
//! ```no_run
//! use libtegra::tsec::Tsec;
//!
//! /// A helper that wraps around a TSEC firmware blob to ensure correct alignment.
//! #[repr(align(256))]
//! struct Firmware<T: Sized> {
//!     pub value: T,
//! }
//!
//! impl<T: Sized> Firmware<T> {
//!     pub const fn new(value: T) -> Self {
//!         Firmware { value }
//!     }
//! }
//!
//! /// The firmware blob.
//! static FAUCON: Firmware<[u8; 15]> = Firmware::new([
//!     0xDF, 0xB0, 0xB0, 0xB0, 0xB0,   // mov $r15 0xB0B0B0B0;
//!     0xC, 0x0,                       // mov $r12 0x0;
//!     0x49, 0x0, 0x11,                // mov $r9 0x1100;
//!     0xF6, 0x9F, 0x0,                // iowr I[$r9] $r15;
//!     0xF8, 0x2,                      // exit;
//! ]);
//!
//! // Load our Faucon firmware onto the TSEC.
//! Tsec::A.load_firmware(&FAUCON.value).unwrap();
//!
//! // Boot it up!
//! unsafe {
//!     Tsec::A.boot_firmware(0).unwrap();
//! }
//! ```
//!
//! [NVIDIA Falcon]: https://envytools.readthedocs.io/en/latest/hw/falcon/index.html
//! [envytools]: https://github.com/envytools/envytools
//! [`Tsec`]: struct.Tsec.html
//! [`FIRMWARE_ALIGNMENT`]: constant.FIRMWARE_ALIGNMENT.html

use enum_primitive::FromPrimitive;

use crate::{car::Clock, kfuse, timer::get_milliseconds};
pub use registers::*;

mod registers;

/// The alignment bits for TSEC firmware blobs.
pub const FIRMWARE_ALIGN_BITS: usize = 8;
/// The alignment a TSEC firmware blob is expected to have.
pub const FIRMWARE_ALIGNMENT: usize = 1 << FIRMWARE_ALIGN_BITS;

enum_from_primitive! {
    /// Enumeration of potential Falcon processor exception clauses
    /// that may occur during code execution on the TSEC.
    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    #[repr(u8)]
    pub enum FalconExceptionClause {
        /// The software trap 0 that may be triggered by the TRAP instruction.
        Trap0,
        /// The software trap 1 that may be triggered by the TRAP instruction.
        Trap1,
        /// The software trap 2 that may be triggered by the TRAP instruction.
        Trap2,
        /// The software trap 3 that may be triggered by the TRAP instruction.
        Trap3,
        /// A trap that is triggered when the Falcon encounters an opcode it
        /// cannot decode.
        InvalidOpcode,
        /// A so-called Secure Fault was indicated by jumping to a secure page,
        /// but the MAC verification for that page failed, which halted the MCU.
        AuthenticationEntry,
        /// A page fault occurred because the TLB failed to provide mappings for
        /// a virtual address on lookup.
        PageMiss,
        /// A page fault occurred because the TLB contained multiple mappings for
        /// a single virtual address on lookup.
        PageMultipleMiss,
        /// A breakpoint, which was set through the integrated hardware debugging
        /// interface, was hit during code execution.
        BreakpointHit,
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

/// Enumeration of potential Falcon processor exceptions
/// that may occur when interacting with the TSEC.
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
    /// The internal TSEC MMIO register interface.
    registers: *const Registers,
}

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
    fn dma_wait_idle(&self) -> Result<(), FalconError> {
        let register_base = unsafe { &*self.registers };

        let timeout = get_milliseconds() + 10_000;

        while (register_base.FALCON_DMATRFCMD.get() & (1 << 1)) == 0 {
            if get_milliseconds() > timeout {
                return Err(FalconError::DmaTimeout);
            }
        }

        Ok(())
    }

    fn try_load_firmware(&self, firmware: &[u8]) -> Result<(), FalconError> {
        let register_base = unsafe { &*self.registers };

        // Check if the firmware is being aligned correctly.
        let firmware_address = firmware.as_ptr() as usize;
        if (firmware_address % FIRMWARE_ALIGNMENT) != 0 {
            return Err(FalconError::FirmwareMisaligned);
        }

        // Ensure that KFUSE is ready.
        kfuse::wait_until_ready().unwrap();

        // Configure the Falcon processor.
        register_base.FALCON_DMACTL.set(0);
        register_base.FALCON_IRQMSET.set(0xFFF2);
        register_base.FALCON_IRQDEST.set(0xFFF0);
        register_base.FALCON_ITFEN.set(3);

        // Make sure the DMA block is in idle state.
        self.dma_wait_idle()?;

        // Load in the memory address of the firmware buffer.
        register_base
            .FALCON_DMATRFBASE
            .set((firmware_address >> FIRMWARE_ALIGN_BITS) as u32);

        // Configure DMA to transfer the physical firmware buffer to the Falcon processor.
        for (index, _) in firmware.chunks(FIRMWARE_ALIGNMENT).enumerate() {
            let base = (index * FIRMWARE_ALIGNMENT) as u32;
            let offset = base;

            register_base.FALCON_DMATRFMOFFS.set(offset);
            register_base.FALCON_DMATRFFBOFFS.set(base);
            register_base.FALCON_DMATRFCMD.set(1 << 4);

            self.dma_wait_idle()?;
        }

        Ok(())
    }

    /// Loads a TSEC firmware buffer onto the Falcon processor.
    ///
    /// NOTE: The memory buffer is expected to be [aligned] correctly.
    ///
    /// [aligned]: constant.FIRMWARE_ALIGNMENT.html
    pub fn load_firmware(&self, firmware: &[u8]) -> Result<(), FalconError> {
        // Enable the device clocks that are required by the TSEC.
        Clock::HOST1X.enable();
        Clock::TSEC.enable();
        Clock::TSECB.enable();
        Clock::SOR_SAFE.enable();
        Clock::SOR0.enable();
        Clock::SOR1.enable();
        Clock::KFUSE.enable();

        // Attempt to load the firmware.
        match self.try_load_firmware(firmware) {
            Ok(()) => Ok(()),
            Err(err) => {
                // The operation has failed, the clocks aren't needed anymore.
                Clock::KFUSE.disable();
                Clock::SOR1.disable();
                Clock::SOR0.disable();
                Clock::SOR_SAFE.disable();
                Clock::TSECB.disable();
                Clock::TSEC.disable();
                Clock::HOST1X.disable();

                Err(err)
            }
        }
    }

    /// Boots a TSEC firmware blob at a specified boot vector start address.
    ///
    /// NOTE: The firmware has to be loaded in through [`Tsec::load_firmware`]
    /// in advance, otherwise this method will fail.
    ///
    /// UNSAFE: This method is considered unsafe because code execution on the
    /// TSEC can fail for malformed or misaligned blobs.
    ///
    /// [`Tsec::load_firmware`]: struct.Tsec.html#method.load_firmware
    pub unsafe fn boot_firmware(&self, boot_vector: u32) -> Result<(), FalconError> {
        let register_base = &*self.registers;
        let mut res;

        // Configure Falcon and start the CPU.
        register_base.FALCON_MAILBOX1.set(0);
        register_base.FALCON_MAILBOX0.set(1);
        register_base.FALCON_BOOTVEC.set(boot_vector);
        register_base.FALCON_CPUCTL.set(2);

        // Wait for the DMA engine to enter idle state.
        res = self.dma_wait_idle();
        if res.is_ok() {
            // Wait for the CPU to be halted.
            while register_base.FALCON_CPUCTL.get() != (1 << 4) {}

            // Check if the CPU has crashed.
            let exception_info = register_base.FALCON_EXCI.get();
            if exception_info != 0 {
                // Gather exception details.
                let pc = exception_info & 0x80000;
                let exception = FalconExceptionClause::from((exception_info >> 20) as u8 & 0xF);

                res = Err(FalconError::Exception(pc, exception));
            }
        }

        res
    }
}
