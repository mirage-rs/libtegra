//! Implementation of the ARM Generic Interrupt Controller v2.

use core::fmt;
use core::marker::PhantomData;

/// An interrupt request for the GIC.
pub type Irq = IrqNumber<{ Gic::NUM_IRQS }>;

/// A generic callback that acts as an interrupt handler.
pub type IrqHandler = fn(usize) -> Result<(), &'static str>;

/// Representation of the Generic Interrupt Controller.
pub struct Gic {
    // Pointer to the GIC CPU Interface registers.
    gicc: *const gicc::Registers,
    // Pointer to the GIC Distributor registers.
    gicd: *const gicd::Registers,

    // A table of IRQ descriptors corresponding to their IRQ numbers.
    handlers: [Option<IrqDescriptor>; Self::MAX_IRQS],
}

impl Gic {
    /// The maximum number of allowed IRQs to be handled by this driver.
    const MAX_IRQS: usize = 300; // Normally 1019, but reserve some space.
    const NUM_IRQS: usize = Self::MAX_IRQS + 1;

    /// Gets an instance of the GIC through the base addresses to the CPU Interface
    /// and Distributor registers.
    ///
    /// # Safety
    ///
    /// The caller is responsible for providing correct physical addresses pointing
    /// to the GICC and GICD devices. Otherwise MMIO access hits uncontrolled addresses
    /// and undefined behavior will be the result.
    pub const unsafe fn get(gicc_base: usize, gicd_base: usize) -> Self {
        Gic {
            gicc: gicc_base as *const _,
            gicd: gicd_base as *const _,
            handlers: [None; Self::MAX_IRQS],
        }
    }

    /// Registers an interrupt descriptor that corresponds to the given IRQ number.
    ///
    /// This overrides previously configured IRQ descriptors for the interrupt,
    /// if any.
    pub fn register_descriptor(&mut self, irq: Irq, descriptor: IrqDescriptor) {
        let irq_num = irq.into_inner();
        assert_ne!(irq_num, 0);

        self.handlers[irq_num - 1] = Some(descriptor);
    }

    /// Dispatches the interrupt handler corresponding to the given IRQ.
    ///
    /// This function should be directly called from the CPU's IRQ exception
    /// vector. A reference to [`IrqContext`] must be passed to this method to
    /// ensure that this is the case.
    ///
    /// Any potential errors when no interrupt handler is actually registered or
    /// when the handler itself encountered an error will be consumed into the
    /// `Result` returned by this function.
    pub fn dispatch_irq<'ctx>(
        &'ctx self,
        irq: Irq,
        _ic: &IrqContext<'ctx>,
    ) -> Result<(), &'static str> {
        let irq_num = irq.into_inner();
        assert_ne!(irq_num, 0);

        if let Some(descriptor) = self.handlers[irq_num - 1] {
            (descriptor.handler)(irq_num)
        } else {
            Err("No handler registered for the given IRQ")
        }
    }
}

/// Interrupt context token.
///
/// An instance of this type indicates that the current core is executing in IRQ
/// context. This means that execution is currently inside an interrupt vector or
/// a subroutine call of it.
// Stolen from https://github.com/rust-embedded/bare-metal/blob/master/src/lib.rs#L16
#[derive(Clone, Copy, Debug)]
pub struct IrqContext<'ctx> {
    _0: PhantomData<&'ctx ()>,
}

impl<'ctx> IrqContext<'ctx> {
    /// Creates a new IRQ context token.
    ///
    /// # Safety
    ///
    /// This must only be called when the current core is in IRQ context and must not
    /// live beyond the end of it. Further, the lifetime `'ctx` of the returned instance
    /// is unconstrained. User code must not be able to influence the lifetime picked
    /// for this type, since that might cause it to be inferred to `'static`.
    #[inline(always)]
    pub unsafe fn new() -> Self {
        IrqContext { _0: PhantomData }
    }
}

/// Describes an IRQ with a corresponding handler function.
#[derive(Clone, Copy, Debug)]
pub struct IrqDescriptor {
    /// A descriptive name for the interrupt in question.
    pub name: &'static str,
    /// A callback that should be invoked for handling this interrupt accordingly.
    pub handler: IrqHandler,
}

/// A wrapper for IRQ numbers that enforces the number to be in a valid range.
#[derive(Clone, Copy, Debug)]
pub struct IrqNumber<const MAX: usize>(usize);

impl<const MAX: usize> IrqNumber<{ MAX }> {
    /// Creates a new IRQ from a given number and validates the number.
    ///
    /// # Panics
    ///
    /// This method panics if the given IRQ number is higher than allowed by
    /// the driver.
    pub const fn new(irq: usize) -> Self {
        if irq >= MAX {
            panic!("IRQ number greater than allowed");
        }

        IrqNumber(irq)
    }

    /// Returns the wrapped IRQ number.
    pub fn into_inner(self) -> usize {
        self.0
    }
}

impl<const MAX: usize> fmt::Display for IrqNumber<{ MAX }> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// Representation of the CPU Interface registers for the GIC.
///
/// Access to these registers is banked so that each CPU core sees their own instance
/// of the GICC when accessing the same registers. It is used to acknowledge and handle
/// pending IRQs.
pub mod gicc {
    use register::{mmio::*, register_bitfields, register_structs};

    register_bitfields! {
        u32,

        /// Bitfields of the `GICC_CTLR` register.
        pub GICC_CTLR [
            EOImode OFFSET(9) NUMBITS(1) [],

            IRQBypDisGrp1 OFFSET(8) NUMBITS(1) [],

            FIQBypDisGrp1 OFFSET(7) NUMBITS(1) [],

            IRQBypDisGrp0 OFFSET(6) NUMBITS(1) [],

            FIQBypDisGrp0 OFFSET(5) NUMBITS(1) [],

            CBPR OFFSET(4) NUMBITS(1) [],

            FIQEn OFFSET(3) NUMBITS(1) [],

            AckCtl OFFSET(2) NUMBITS(1) [],

            EnableGrp1 OFFSET(1) NUMBITS(1) [],

            EnableGrp0 OFFSET(0) NUMBITS(1) []
        ],

        /// Bitfields of the `GICC_PMR` register.
        pub GICC_PMR [
            Priority OFFSET(0) NUMBITS(8) []
        ],

        /// Bitfields of the `GICC_BPR` register.
        pub GICC_BPR [
            BinaryPoint OFFSET(0) NUMBITS(3) []
        ],

        /// Bitfields of the `GICC_IAR` register.
        pub GICC_IAR [
            CPUID OFFSET(10) NUMBITS(3) [],

            InterruptID OFFSET(0) NUMBITS(10) []
        ],

        /// Bitfields of the `GICC_HPPIR` register.
        pub GICC_HPPIR [
            CPUID OFFSET(10) NUMBITS(3) [],

            PENDINTID OFFSET(0) NUMBITS(10) []
        ],

        /// Bitfields of the `GICC_AIAR` register.
        pub GICC_AIAR [
            CPUID OFFSET(10) NUMBITS(3) [],

            InterruptID OFFSET(0) NUMBITS(10) []
        ],

        /// Bitfields of the `GICC_AHPPIR` register.
        pub GICC_AHPPIR [
            CPUID OFFSET(10) NUMBITS(3) [],

            PENDINTID OFFSET(0) NUMBITS(10) []
        ],

        /// Bitfields of the `GICC_IIDR` register.
        pub GICC_IIDR [
            ProductID OFFSET(20) NUMBITS(12) [],

            ArchVersion OFFSET(16) NUMBITS(4) [],

            Revision OFFSET(12) NUMBITS(4) [],

            Implementer OFFSET(0) NUMBITS(12) []
        ]
    }

    register_structs! {
        /// Representation of the GIC CPU Interface registers.
        #[allow(non_snake_case)]
        pub Registers {
            (0x0000 => pub GICC_CTLR: ReadWrite<u32, GICC_CTLR::Register>),
            (0x0004 => pub GICC_PMR: ReadWrite<u32, GICC_PMR::Register>),
            (0x0008 => pub GICC_BPR: ReadWrite<u32, GICC_BPR::Register>),
            (0x000C => pub GICC_IAR: ReadOnly<u32, GICC_IAR::Register>),
            (0x0010 => pub GICC_EOIR: WriteOnly<u32>),
            (0x0014 => pub GICC_RPR: ReadOnly<u32>),
            (0x0018 => pub GICC_HPPIR: ReadOnly<u32, GICC_HPPIR::Register>),
            (0x001C => pub GICC_ABPR: ReadWrite<u32>),
            (0x0020 => pub GICC_AIAR: ReadOnly<u32, GICC_AIAR::Register>),
            (0x0024 => pub GICC_AEOIR: WriteOnly<u32>),
            (0x0028 => pub GICC_AHPPIR: ReadOnly<u32, GICC_AHPPIR::Register>),
            (0x002C => pub GICC_STATUSR: ReadOnly<u32>),
            (0x0030 => _reserved0),
            (0x00D0 => pub GICC_APR: [ReadWrite<u32>; 0x4]),
            (0x00E0 => pub GICC_NSAPR: [ReadWrite<u32>; 0x4]),
            (0x00F0 => _reserved1: [ReadWrite<u32>; 0x3]),
            (0x00FC => pub GICC_IIDR: ReadOnly<u32, GICC_IIDR::Register>),
            (0x0100 => _reserved2),
            (0x1000 => pub GICC_DIR: WriteOnly<u32>),
            (0x1004 => _reserved3),
            (0x2000 => @END),
        }
    }

    assert_eq_size!(Registers, [u8; 0x2000]);
}

/// Representation of the Distributor registers for the GIC.
///
/// The GICD is used to configure and route IRQs to one or more CPU cores subsequently.
/// Unlike for the GICC, access to these registers is only banked for a few registers
/// and other than that, all CPU cores see the same instance of the Distributor.
pub mod gicd {
    use register::{mmio::*, register_bitfields, register_structs};

    // TODO: Add missing bitfields.

    register_bitfields! {
        u32,

        /// Bitfields of the `GICD_CTLR` register.
        pub GICD_CTLR [
            EnableGrp1 OFFSET(1) NUMBITS(1) [],

            EnableGrp0 OFFSET(0) NUMBITS(1) []
        ],

        /// Bitfields of the `GICD_TYPER` register.
        pub GICD_TYPER [
            LSPI OFFSET(11) NUMBITS(5) [],

            SecurityExtn OFFSET(10) NUMBITS(1) [],

            CPUNumber OFFSET(5) NUMBITS(3) [],

            ITLinesNumber OFFSET(0) NUMBITS(5) []
        ]
    }

    register_structs! {
        /// Representation of the GIC Distributor registers.
        #[allow(non_snake_case)]
        pub Registers {
            (0x0000 => pub GICD_CTLR: ReadWrite<u32, GICD_CTLR::Register>),
            (0x0004 => pub GICD_TYPER: ReadOnly<u32, GICD_TYPER::Register>),
            (0x0008 => pub GICD_IIDR: ReadOnly<u32>),
            (0x000C => _reserved0),
            (0x0080 => pub GICD_IGROUP: [ReadWrite<u32>; 0xF]),
            (0x00BC => _reserved1),
            (0x0100 => pub GICD_ISENABLER: [ReadWrite<u32>; 0xF]),
            (0x013C => _reserved2),
            (0x0180 => pub GICD_ICENABLER: [ReadWrite<u32>; 0xF]),
            (0x01BC => _reserved3),
            (0x0200 => pub GICD_ISPENDR: [ReadWrite<u32>; 0xF]),
            (0x023C => _reserved4),
            (0x0280 => pub GICD_ICPENDR: [ReadWrite<u32>; 0xF]),
            (0x02BC => _reserved5),
            (0x0300 => pub GICD_ISACTIVER: [ReadWrite<u32>; 0xF]),
            (0x033C => _reserved6),
            (0x0380 => pub GICD_ICACTIVER: [ReadWrite<u32>; 0xF]),
            (0x03BC => _reserved7),
            (0x0400 => pub GICD_IPRIORITYR: [ReadWrite<u32>; 0x7F]),
            (0x05FC => _reserved8),
            (0x0800 => pub GICD_ITARGETSR: [ReadWrite<u32>; 0x7F]),
            (0x09FC => _reserved9),
            (0x0C00 => pub GICD_ICFGR: [ReadWrite<u32>; 0x1F]),
            (0x0C7C => _reserved10),
            (0x0D00 => pub GICD_PPISR: ReadOnly<u32>),
            (0x0D04 => pub GICD_SPISR: [ReadOnly<u32>; 0xE]),
            (0x0D3C => _reserved11),
            (0x0F00 => pub GICD_SGIR: WriteOnly<u32>),
            (0x0F04 => _reserved12),
            (0x0F10 => pub GICD_CPENDSGIR: [ReadWrite<u32>; 0x3]),
            (0x0F1C => _reserved13),
            (0x0F20 => pub GICD_SPENDSGIR: [ReadWrite<u32>; 0x3]),
            (0x0F2C => _reserved14),
            (0x0FD0 => pub GICD_PIDR4: ReadOnly<u32>),
            (0x0FD4 => pub GICD_PIDR5: ReadOnly<u32>),
            (0x0FD8 => pub GICD_PIDR6: ReadOnly<u32>),
            (0x0FDC => pub GICD_PIDR7: ReadOnly<u32>),
            (0x0FE0 => pub GICD_PIDR0: ReadOnly<u32>),
            (0x0FE4 => pub GICD_PIDR1: ReadOnly<u32>),
            (0x0FE8 => pub GICD_PIDR2: ReadOnly<u32>),
            (0x0FEC => pub GICD_PIDR3: ReadOnly<u32>),
            (0x0FF0 => pub GICD_CIDR0: ReadOnly<u32>),
            (0x0FF4 => pub GICD_CIDR1: ReadOnly<u32>),
            (0x0FF8 => pub GICD_CIDR2: ReadOnly<u32>),
            (0x0FFC => pub GICD_CIDR3: ReadOnly<u32>),
            (0x1000 => @END),
        }
    }

    assert_eq_size!(Registers, [u8; 0x1000]);
}
