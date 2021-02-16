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
    const MAX_IRQS: usize = 224; // Normally 1019, but reserve some space.
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

    /// Initializes and enables the GIC device for interrupt delivery.
    ///
    /// NOTE: This method must be called once before the GIC is usable.
    pub fn init(&self) {
        let gicc = unsafe { &*self.gicc };
        let gicd = unsafe { &*self.gicd };

        // Ensure that the `Gic` instance points to a supported device.
        let gic_version = (gicd.GICD_PIDR2.get() >> 0x4) & 0xF;
        if gic_version != 2 {
            panic!("GIC version is incompatible with GICv2");
        }

        // Enable the Distributor.
        gicd.GICD_CTLR.write(gicd::GICD_CTLR::EnableGrp0::SET);

        // Boot and configure the CPU Interface to accept IRQs of all priorities.
        gicc.GICC_PMR.write(gicc::GICC_PMR::Priority.val(0xFF));
        gicc.GICC_CTLR.write(gicc::GICC_CTLR::EnableGrp0::SET);
    }

    /// Gets the number of IRQs implemented in hardware.
    pub fn get_num_irqs(&mut self) -> usize {
        let gicd = unsafe { &*self.gicd };

        let lines_num = gicd.GICD_TYPER.read(gicd::GICD_TYPER::ITLinesNumber) as usize;
        (lines_num + 1) * 32
    }

    /// Extracts the IRQ with the highest priority from pending interrupts.
    ///
    /// This function should be directly called from the CPU's IRQ exception
    /// vector. A reference to [`IrqContext`] must be passed to this method to
    /// ensure that this is the case.
    ///
    /// [`IrqContext`]: struct.IrqContext.html
    pub fn get_pending_irq<'ctx>(&'ctx self, _ic: &IrqContext<'ctx>) -> Irq {
        let gicc = unsafe { &*self.gicc };

        let num = gicc.GICC_IAR.read(gicc::GICC_IAR::InterruptID);
        Irq::new(num as usize)
    }

    /// Completes the handling of a currently active IRQ by signaling an EOI
    /// (End of Interrupt).
    ///
    /// This function should be directly called from the CPU's IRQ exception
    /// vector. A reference to [`IrqContext`] must be passed to this method to
    /// ensure that this is the case.
    ///
    /// [`IrqContext`]: struct.IrqContext.html
    pub fn mark_irq_completed<'ctx>(&'ctx self, irq: Irq, _ic: &IrqContext<'ctx>) {
        let gicc = unsafe { &*self.gicc };

        let irq_num = irq.into_inner() as u32;
        gicc.GICC_EOIR.write(gicc::GICC_EOIR::EOIINTID.val(irq_num));
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
    ///
    /// [`IrqContext`]: struct.IrqContext.html
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

    /// Sets the enabled state for a specific IRQ.
    ///
    /// By marking an interrupt as disabled, it will not be dispatched by the GIC
    /// anymore until it is re-enabled again and vice versa. Can be used to bypass
    /// specific IRQs either permanently or temporarily.
    pub fn set_irq_enable(&self, irq: Irq, enable: bool) {
        let gicd = unsafe { &*self.gicd };

        // Find the index of the `ISENABLER[i]` register corresponding to the
        // IRQ number and determine bit to set or clear in it.
        let irq_num = irq.into_inner();
        let enable_reg_index = irq_num >> 5;
        let enable_bit = (enable as u32) << (irq_num % 32);

        // Set the bit in the corresponding ISENABLER register.
        let isenabler = &gicd.GICD_ISENABLER[enable_reg_index];
        isenabler.set(isenabler.get() | enable_bit);
    }

    /// Configures the interrupt triggering mode for the given SPI.
    pub fn set_spi_mode(&self, irq: Irq, mode: IrqMode) {
        let gicd = unsafe { &*self.gicd };

        let irq_num = irq.into_inner();
        assert!(irq_num >= 32);

        // Find the index of the `ICFGR[i]` register corresponding to the IRQ
        // number and determine the trigger configuration value to write.
        let icfgr_reg_index = irq_num >> 4;
        let icfgr_value = ((mode as u32) << 1) << (irq_num % 16) * 2;

        // Set the bits in the corresponding ICFGR register.
        let icfgr = &gicd.GICD_ICFGR[icfgr_reg_index];
        icfgr.set(icfgr.get() | icfgr_value);
    }

    /// Sets a given IRQ to a pending state in the GIC.
    pub fn set_irq_pending(&self, irq: Irq) {
        let gicd = unsafe { &*self.gicd };

        // Find the index of the `ISPENDR[i]` register corresponding to the
        // IRQ number and determine the bit to set in it.
        let irq_num = irq.into_inner();
        let ispendr_reg_index = irq_num >> 5;
        let ispendr_bit = 1 << (irq_num % 32);

        // Set the bit in the corresponding ISPENDR register.
        let ispendr = &gicd.GICD_ISPENDR[ispendr_reg_index];
        ispendr.set(ispendr.get() | ispendr_bit);
    }

    /// Sets the delivery priority for a given IRQ.
    ///
    /// The lower the value, the higher the priority. `0` is the highest
    /// interrupt delivery priority supported by the GIC.
    pub fn set_irq_priority(&self, irq: Irq, priority: u32) {
        let gicd = unsafe { &*self.gicd };

        // Find the index of the `IPRIORITYR[i]` register corresponding to the
        // IRQ number to determine the priority configuration value to write.
        let irq_num = irq.into_inner();
        let ipriorityr_reg_index = irq_num >> 2;
        let ipriorityr_reg_bit = (irq_num % 4) * 8;

        // Calculate the mask to apply to the register value.
        let mask = 0xFF << ipriorityr_reg_bit;

        // Write the value in the corresponding IPRIORITYR register.
        let ipriorityr = &gicd.GICD_IPRIORITYR[ipriorityr_reg_index];
        let value = ipriorityr.get() & !mask;
        ipriorityr.set(value | ((priority << ipriorityr_reg_bit) & mask));
    }

    /// Assigns a new group to a given IRQ.
    ///
    /// On boot or reset, all SPIs are in group 0, making them secure interrupt.
    /// Putting individual SPIs into group 1 effectively makes them secure interrupts.
    pub fn set_irq_group(&self, irq: Irq, group: u32) {
        let gicd = unsafe { &*self.gicd };

        // Find the index of the `IGROUPR[i]` register corresponding to the IRQ
        // number to determine the group configuration value to write.
        let irq_num = irq.into_inner();
        let igroupr_reg_index = irq_num >> 5;
        let igroupr_reg_bit = irq_num % 32;

        // Calculate the mask to apply to the register value.
        let mask = 1 << igroupr_reg_bit;

        // Write the value in the corresponding IGROUPR register.
        let igroupr = &gicd.GICD_IGROUPR[igroupr_reg_index];
        let value = igroupr.get() & !mask;
        igroupr.set(value | ((group << igroupr_reg_bit) & mask));
    }

    /// Configures the target CPU core to route the given IRQs to.
    pub fn set_spi_target_cpu(&self, irq: Irq, cpu: u32) {
        let gicd = unsafe { &*self.gicd };

        // Find the index of the `ITARGETSR[i]` register corresponding to the
        // IRQ number to determine the configuration value to write.
        let irq_num = irq.into_inner();
        let itargetsr_reg_index = irq_num >> 2;
        let itargetsr_reg_bit = (irq_num % 4) * 8;

        // Calculate the mask to apply to the register value.
        let mask = 0xFF << itargetsr_reg_bit;

        // Write the value to the corresponding ITARGETSR register.
        let itargetsr = &gicd.GICD_ITARGETSR[itargetsr_reg_index];
        let value = itargetsr.get() & !mask;
        itargetsr.set(value | ((cpu << itargetsr_reg_bit) & mask));
    }
}

/// Enum for selecting in which mode to trigger interrupts on.
#[derive(Clone, Copy, Debug)]
#[repr(u32)]
pub enum IrqMode {
    /// A level-triggered interrupt request.
    Level = 0,
    /// An edge-triggered interrupt request.
    Edge = 1,
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

        /// Bitfields of the `GICC_EOIR` register.
        pub GICC_EOIR [
            EOIINTID OFFSET(0) NUMBITS(10) []
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
            (0x0010 => pub GICC_EOIR: WriteOnly<u32, GICC_EOIR::Register>),
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
        ],

        /// Bitfields of the `GICD_ITARGETSR` register.
        pub GICD_ITARGETSR [
            Offset3 OFFSET(24) NUMBITS(8) [],

            Offset2 OFFSET(16) NUMBITS(8) [],

            Offset1 OFFSET(8) NUMBITS(8) [],

            Offset0 OFFSET(0) NUMBITS(8) []
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
            (0x0080 => pub GICD_IGROUPR: [ReadWrite<u32>; 0x20]),
            (0x0100 => pub GICD_ISENABLER: [ReadWrite<u32>; 0x20]),
            (0x0180 => pub GICD_ICENABLER: [ReadWrite<u32>; 0x20]),
            (0x0200 => pub GICD_ISPENDR: [ReadWrite<u32>; 0x20]),
            (0x0280 => pub GICD_ICPENDR: [ReadWrite<u32>; 0x20]),
            (0x0300 => pub GICD_ISACTIVER: [ReadWrite<u32>; 0x20]),
            (0x0380 => pub GICD_ICACTIVER: [ReadWrite<u32>; 0x20]),
            (0x0400 => pub GICD_IPRIORITYR: [ReadWrite<u32>; 0xFF]),
            (0x07FC => _reserved1),
            (0x0800 => pub GICD_ITARGETSR: [ReadWrite<u32, GICD_ITARGETSR::Register>; 0xFF]),
            (0x0BFC => _reserved2),
            (0x0C00 => pub GICD_ICFGR: [ReadWrite<u32>; 0x40]),
            (0x0D00 => pub GICD_PPISR: ReadOnly<u32>),
            (0x0D04 => pub GICD_SPISR: [ReadOnly<u32>; 0xE]),
            (0x0D3C => _reserved3),
            (0x0F00 => pub GICD_SGIR: WriteOnly<u32>),
            (0x0F04 => _reserved4),
            (0x0F10 => pub GICD_CPENDSGIR: [ReadWrite<u32>; 0x4]),
            (0x0F20 => pub GICD_SPENDSGIR: [ReadWrite<u32>; 0x4]),
            (0x0F30 => _reserved5),
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
