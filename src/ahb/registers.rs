/// Abstractions over the AHB Memory Controller Slave registers.
///
/// See Chapter 19.4.2 in the Tegra X1 Technical Reference
/// Manual for details.
pub mod mem {
    use register::{mmio::*, register_structs};

    use crate::memory_map::SYSREG;

    /// A pointer to the AHB MC Slave registers that can be accessed by dereferencing it.
    pub const REGISTERS: *const Registers = SYSREG as *const Registers;

    register_structs! {
        /// Representation of the AHB Memory Controller Slave registers.
        #[allow(non_snake_case)]
        pub Registers {
            (0x000 => _reserved0: [ReadWrite<u8>; 0xCC]),
            (0x0CC => pub AHB_AHB_MEM_PREFETCH_CFG5_0: ReadWrite<u32>),
            (0x0D0 => pub AHB_AHB_MEM_PREFETCH_CFG6_0: ReadWrite<u32>),
            (0x0D4 => pub AHB_AHB_MEM_PREFETCH_CFG7_0: ReadWrite<u32>),
            (0x0D8 => pub AHB_AHB_MEM_PREFETCH_CFG8_0: ReadWrite<u32>),
            (0x0DC => pub AHB_AHB_MEM_PREFETCH_CFG_X_0: ReadWrite<u32>),
            (0x0E0 => pub AHB_ARBITRATION_XBAR_CTRL_0: ReadWrite<u32>),
            (0x0E4 => pub AHB_AHB_MEM_PREFETCH_CFG3_0: ReadWrite<u32>),
            (0x0E8 => pub AHB_AHB_MEM_PREFETCH_CFG4_0: ReadWrite<u32>),
            (0x0EC => pub AHB_AVP_PPCS_RD_COH_STATUS_0: ReadOnly<u32>),
            (0x0F0 => pub AHB_AHB_MEM_PREFETCH_CFG1_0: ReadWrite<u32>),
            (0x0F4 => pub AHB_AHB_MEM_PREFETCH_CFG2_0: ReadWrite<u32>),
            (0x0F8 => pub AHB_AHBSLVMEM_STATUS_0: ReadOnly<u32>),
            (0x0FC => pub AHB_ARBITRATION_AHB_MEM_WRQUE_MST_ID_0: ReadOnly<u32>),
            (0x100 => pub AHB_ARBITRATION_CPU_ABORT_ADDR_0: ReadOnly<u32>),
            (0x104 => pub AHB_ARBITRATION_CPU_ABORT_INFO_0: ReadOnly<u32>),
            (0x108 => pub AHB_ARBITRATION_COP_ABORT_ADDR_0: ReadOnly<u32>),
            (0x10C => pub AHB_ARBITRATION_COP_ABORT_INFO_0: ReadOnly<u32>),
            (0x110 => pub AHB_AHB_SPARE_REG_0: ReadWrite<u32>),
            (0x114 => pub AHB_XBAR_SPARE_REG_0: ReadWrite<u32>),
            (0x118 => _reserved1: [ReadWrite<u8>; 0x8]),
            (0x120 => pub AHB_AVPC_MCCIF_FIFOCTRL_0: ReadWrite<u32>),
            (0x124 => pub AHB_TIMEOUT_WCOAL_AVPC_0: ReadWrite<u32>),
            (0x128 => pub AHB_MPCORE_MCCIF_FIFOCTRL_0: ReadWrite<u32>),
            (0x12C => @END),
        }
    }
}
