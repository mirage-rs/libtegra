//! Driver for interfacing with the Tegra Security Co-Processor (TSEC).
//!
//! # Description
//!
//! The TSEC is a dedicated unit powered by a NVIDIA Falcon
//! microprocessor with crypto extensions.
//!
//! ## Falcon Processor
//!
//! The [NVIDIA Falcon] microprocessor is an MCU with very limited code
//! and data space, which is why many features in general are not
//! available.
//!
//! See [this article] for a more in-depth description of the Falcon
//! LLVM backend and thus the Falcon environment along with its limitations
//! and features.
//!
//! Within the TSEC, it has special crypto extensions, which makes it
//! usable for execution of critical code within a secure environment.
//!
//! ## Firmware
//!
//! [envytools] have proven to be valuable tools when it comes to working
//! with various ISAs used by NVIDIA, including the Falcon processor.
//!
//! The following examples will use this reference firmware:
//!
//! ```asm
//! mov $r15 0xB0B0B0B0;
//! mov $r12 0x0;
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
//! be aligned to the boundary denoted by [`FIRMWARE_ALIGNMENT`].
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
//! static FAUCON: Firmware<[u8; 15]> = Firmware::new([
//!     0xDF, 0xB0, 0xB0, 0xB0, 0xB0,   // mov $r15 0xB0B0B0B0;
//!     0xC, 0x0,                       // mov $r12 0x0;
//!     0x49, 0x0, 0x11,                // mov $r9 0x1100;
//!     0xF6, 0x9F, 0x0,                // iowr I[$r9] $r15;
//!     0xF8, 0x2,                      // exit;
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
//! /// The global instance of the TSEC.
//! const TSEC: Tsec = Tsec::new();
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
//! // The arguments for the Mailbox configuration.
//! let mut argument0 = 0;
//! let mut argument1 = 0;
//!
//! // Load our Faucon firmware onto the TSEC.
//! TSEC.load_firmware(&FAUCON.value).unwrap();
//!
//! // Boot it up...
//! unsafe {
//!     TSEC.boot_firmware(0, &mut argument0, &mut argument1).unwrap();
//! }
//!
//! // ...and finally check the Mailbox parameters!
//! assert_eq!(argument0, 0x0);
//! assert_eq!(argument1, 0xB0B0B0B0);
//! ```
//!
//! [NVIDIA Falcon]: https://envytools.readthedocs.io/en/latest/hw/falcon/index.html
//! [this article]: https://0x04.net/~mwk/Falcon.html
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
        Trap0,
        Trap1,
        Trap2,
        Trap3,
        InvalidOpcode,
        AuthenticationEntry,
        PageMiss,
        PageMultipleMiss,
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
    /// Counter where execution has halted and a [`FalconExceptionClause`] which
    /// provides additional details.
    ///
    /// [`FalconExceptionClause`]: enum.FalconExceptionClause.html
    Exception(u32, FalconExceptionClause),
}

/// Representation of the Tegra Security Co-Processor.
pub struct Tsec;

impl Tsec {
    /// Creates a new instance of the TSEC.
    ///
    /// NOTE: Please refrain from calling this method multiple times.
    /// It is advised to create a single, global instance
    /// of the [`Tsec`] and stick to it.
    ///
    /// [`Tsec`]: struct.Tsec.html
    pub const fn new() -> Self {
        Tsec
    }

    /// Waits for the DMA engine to enter idle state.
    fn dma_wait_idle(&self) -> Result<(), FalconError> {
        let register_base = unsafe { &*REGISTERS };

        let timeout = get_milliseconds() + 10_000;

        while (register_base.FALCON_DMATRFCMD.get() & (1 << 1)) == 0 {
            if get_milliseconds() > timeout {
                return Err(FalconError::DmaTimeout);
            }
        }

        Ok(())
    }

    /// Attempts to load a TSEC firmware blob onto the Falcon processor.
    fn try_load_firmware(&self, firmware: &[u8]) -> Result<(), FalconError> {
        let register_base = unsafe { &*REGISTERS };

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
    /// TSEC can always fail, especially for malformed or misaligned blobs.
    ///
    /// [`Tsec::load_firmware`]: struct.Tsec.html#method.load_firmware
    pub unsafe fn boot_firmware(
        &self,
        boot_vector: u32,
        arg0: &mut u32,
        arg1: &mut u32,
    ) -> Result<(), FalconError> {
        let register_base = &*REGISTERS;
        let mut res;

        // Configure Falcon and start the CPU.
        register_base.FALCON_MAILBOX1.set(*arg1);
        register_base.FALCON_MAILBOX0.set(*arg0);
        //register_base.FALCON_MAILBOX0.set(*arg1);
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
