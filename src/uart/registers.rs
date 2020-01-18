use register::{mmio::*, register_bitfields, register_structs};

/// Base address for UART registers.
pub const UART_BASE: u32 = 0x7000_6000;

/// A pointer to the UART A register block that can be accessed by dereferencing it.
pub const UART_A_REGISTERS: *const Registers = (UART_BASE + 0x000) as *const Registers;
/// A pointer to the UART B register block that can be accessed by dereferencing it.
pub const UART_B_REGISTERS: *const Registers = (UART_BASE + 0x040) as *const Registers;
/// A pointer to the UART C register block that can be accessed by dereferencing it.
pub const UART_C_REGISTERS: *const Registers = (UART_BASE + 0x200) as *const Registers;
/// A pointer to the UART D register block that can be accessed by dereferencing it.
pub const UART_D_REGISTERS: *const Registers = (UART_BASE + 0x300) as *const Registers;
/// A pointer to the UART (AP)E register block that can be accessed by dereferencing it.
pub const UART_E_REGISTERS: *const Registers = (UART_BASE + 0x400) as *const Registers;

register_structs! {
    /// Representation of the UART registers.
    #[allow(non_snake_case)]
    pub Registers {
        (0x00 => pub UART_THR_DLAB_0_0: ReadWrite<u32>),
        (0x04 => pub UART_IER_DLAB_0_0: ReadWrite<u32>),
        (0x08 => pub UART_IIR_FCR_0: ReadWrite<u32>),
        (0x0C => pub UART_LCR_0: ReadWrite<u32>),
        (0x10 => pub UART_MCR_0: ReadWrite<u32>),
        (0x14 => pub UART_LSR_0: ReadOnly<u32>),
        (0x18 => pub UART_MSR_0: ReadWrite<u32>),
        (0x1C => pub UART_SPR_0: ReadWrite<u32>),
        (0x20 => pub UART_IRDA_CSR_0: ReadWrite<u32>),
        (0x24 => pub UART_RX_FIFO_CFG_0: ReadWrite<u32>),
        (0x28 => pub UART_MIE_0: ReadWrite<u32>),
        (0x2C => pub UART_VENDOR_STATUS_0_0: ReadOnly<u32>),
        (0x30 => _reserved: [ReadWrite<u8>; 0xC]),
        (0x3C => pub UART_ASR_0: ReadWrite<u32>),
        (0x40 => @END),
    }
}

assert_eq_size!(Registers, [u8; 0x40]);
