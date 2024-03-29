//! Abstractions over the Memory Controller registers of the Tegra X1.
//!
//! See Chapter 18.11 in the Tegra X1 Technical Reference Manual
//! for details.

use tock_registers::{register_structs, registers::*};

use crate::memory_map::MC;

/// A pointer to the MC register block that can be accessed by dereferencing it.
pub const REGISTERS: *const Registers = MC as *const Registers;

register_structs! {
    /// Representation of the Memory Controller registers.
    #[allow(non_snake_case)]
    pub Registers {
        (0x0 => pub MC_INTSTATUS_0: ReadWrite<u32>),
        (0x4 => pub MC_INTMASK_0: ReadWrite<u32>),
        (0x8 => pub MC_ERR_STATUS_0: ReadOnly<u32>),
        (0xC => pub MC_ERR_ADR_0: ReadOnly<u32>),
        (0x10 => pub MC_SMMU_CONFIG_0: ReadWrite<u32>),
        (0x14 => pub MC_SMMU_TLB_CONFIG_0: ReadWrite<u32>),
        (0x18 => pub MC_SMMU_PTC_CONFIG_0: ReadWrite<u32>),
        (0x1C => pub MC_SMMU_PTB_ASID_0: ReadWrite<u32>),
        (0x20 => pub MC_SMMU_PTB_DATA_0: ReadWrite<u32>),
        (0x24 => _reserved0: [ReadWrite<u8>; 0xC]),
        (0x30 => pub MC_SMMU_TLB_FLUSH_0: ReadWrite<u32>),
        (0x34 => pub MC_SMMU_PTC_FLUSH_0: ReadWrite<u32>),
        (0x38 => pub MC_SMMU_ASID_SECURITY_0: ReadWrite<u32>),
        (0x3C => pub MC_SMMU_ASID_SECURITY_1_0: ReadWrite<u32>),
        (0x40 => _reserved1: [ReadWrite<u8>; 0x10]),
        (0x50 => pub MC_EMEM_CFG_0: ReadWrite<u32>),
        (0x54 => pub MC_EMEM_ADR_CFG_0: ReadWrite<u32>),
        (0x58 => pub MC_EMEM_ADR_CFG_DEV0_0: ReadWrite<u32>),
        (0x5C => pub MC_EMEM_ADR_CFG_DEV1_0: ReadWrite<u32>),
        (0x60 => pub MC_EMEM_ADR_CFG_CHANNEL_MASK_0: ReadWrite<u32>),
        (0x64 => pub MC_EMEM_ADR_CFG_BANK_MASK_0_0: ReadWrite<u32>),
        (0x68 => pub MC_EMEM_ADR_CFG_BANK_MASK_1_0: ReadWrite<u32>),
        (0x6C => pub MC_EMEM_ADR_CFG_BANK_MASK_2_0: ReadWrite<u32>),
        (0x70 => pub MC_SECURITY_CFG0_0: ReadWrite<u32>),
        (0x74 => pub MC_SECURITY_CFG1_0: ReadWrite<u32>),
        (0x78 => _reserved2: [ReadWrite<u8>; 0x18]),
        (0x90 => pub MC_EMEM_ARB_CFG_0: ReadWrite<u32>),
        (0x94 => pub MC_EMEM_ARB_OUTSTANDING_REQ_0: ReadWrite<u32>),
        (0x98 => pub MC_EMEM_ARB_TIMING_RCD_0: ReadWrite<u32>),
        (0x9C => pub MC_EMEM_ARB_TIMING_RP_0: ReadWrite<u32>),
        (0xA0 => pub MC_EMEM_ARB_TIMING_RC_0: ReadWrite<u32>),
        (0xA4 => pub MC_EMEM_ARB_TIMING_RAS_0: ReadWrite<u32>),
        (0xA8 => pub MC_EMEM_ARB_TIMING_FAW_0: ReadWrite<u32>),
        (0xAC => pub MC_EMEM_ARB_TIMING_RRD_0: ReadWrite<u32>),
        (0xB0 => pub MC_EMEM_ARB_TIMING_RAP2PRE_0: ReadWrite<u32>),
        (0xB4 => pub MC_EMEM_ARB_TIMING_WAP2PRE_0: ReadWrite<u32>),
        (0xB8 => pub MC_EMEM_ARB_TIMING_R2R_0: ReadWrite<u32>),
        (0xBC => pub MC_EMEM_ARB_TIMING_W2W_0: ReadWrite<u32>),
        (0xC0 => pub MC_EMEM_ARB_TIMING_R2W_0: ReadWrite<u32>),
        (0xC4 => pub MC_EMEM_ARB_TIMING_W2R_0: ReadWrite<u32>),
        (0xC8 => pub MC_EMEM_ARB_MISC2_0: ReadWrite<u32>),
        (0xCC => _reserved3: [ReadWrite<u8>; 0x4]),
        (0xD0 => pub MC_EMEM_ARB_DA_TURNS_0: ReadWrite<u32>),
        (0xD4 => pub MC_EMEM_ARB_DA_COVERS_0: ReadWrite<u32>),
        (0xD8 => pub MC_EMEM_ARB_MISC0_0: ReadWrite<u32>),
        (0xDC => pub MC_EMEM_ARB_MISC1_0: ReadWrite<u32>),
        (0xE0 => pub MC_EMEM_ARB_RING1_THROTTLE_0: ReadWrite<u32>),
        (0xE4 => pub MC_EMEM_ARB_RING3_THROTTLE_0: ReadWrite<u32>),
        (0xE8 => pub MC_EMEM_ARB_OVERRIDE_0: ReadWrite<u32>),
        (0xEC => pub MC_EMEM_ARB_RSV_0: ReadWrite<u32>),
        (0xF0 => _reserved4: [ReadWrite<u8>; 0x4]),
        (0xF4 => pub MC_CLKEN_OVERRIDE_0: ReadWrite<u32>),
        (0xF8 => _reserved5: [ReadWrite<u8>; 0x4]),
        (0xFC => pub MC_TIMING_CONTROL_0: ReadWrite<u32>),
        (0x100 => pub MC_STAT_CONTROL_0: ReadWrite<u32>),
        (0x104 => _reserved6: [ReadWrite<u8>; 0xFC]),
        (0x200 => pub MC_CLIENT_HOTRESET_CTRL_0: ReadWrite<u32>),
        (0x204 => pub MC_CLIENT_HOTRESET_STATUS_0: ReadOnly<u32>),
        (0x208 => pub MC_EMEM_ARB_ISOCHRONOUS_0_0: ReadWrite<u32>),
        (0x20C => pub MC_EMEM_ARB_ISOCHRONOUS_1_0: ReadWrite<u32>),
        (0x210 => pub MC_EMEM_ARB_ISOCHRONOUS_2_0: ReadWrite<u32>),
        (0x214 => pub MC_EMEM_ARB_ISOCHRONOUS_3_0: ReadWrite<u32>),
        (0x218 => pub MC_EMEM_ARB_HYSTERESIS_0_0: ReadWrite<u32>),
        (0x21C => pub MC_EMEM_ARB_HYSTERESIS_1_0: ReadWrite<u32>),
        (0x220 => pub MC_EMEM_ARB_HYSTERESIS_2_0: ReadWrite<u32>),
        (0x224 => pub MC_EMEM_ARB_HYSTERESIS_3_0: ReadWrite<u32>),
        (0x228 => pub MC_SMMU_TRANSLATION_ENABLE_0_0: ReadWrite<u32>),
        (0x22C => pub MC_SMMU_TRANSLATION_ENABLE_1_0: ReadWrite<u32>),
        (0x230 => pub MC_SMMU_TRANSLATION_ENABLE_2_0: ReadWrite<u32>),
        (0x234 => pub MC_SMMU_TRANSLATION_ENABLE_3_0: ReadWrite<u32>),
        (0x238 => pub MC_SMMU_AFI_ASID_0: ReadWrite<u32>),
        (0x23C => pub MC_SMMU_AVPC_ASID_0: ReadWrite<u32>),
        (0x240 => pub MC_SMMU_DC_ASID_0: ReadWrite<u32>),
        (0x244 => pub MC_SMMU_DCB_ASID_0: ReadWrite<u32>),
        (0x248 => _reserved7: [ReadWrite<u8>; 0x8]),
        (0x250 => pub MC_SMMU_HC_ASID_0: ReadWrite<u32>),
        (0x254 => pub MC_SMMU_HDA_ASID_0: ReadWrite<u32>),
        (0x258 => pub MC_SMMU_ISP2_ASID_0: ReadWrite<u32>),
        (0x25C => _reserved8: [ReadWrite<u8>; 0x8]),
        (0x264 => pub MC_SMMU_NVENC_ASID_0: ReadWrite<u32>),
        (0x268 => pub MC_SMMU_NV_ASID_0: ReadWrite<u32>),
        (0x26C => pub MC_SMMU_NV2_ASID_0: ReadWrite<u32>),
        (0x270 => pub MC_SMMU_PPCS_ASID_0: ReadWrite<u32>),
        (0x274 => pub MC_SMMU_SATA_ASID_0: ReadWrite<u32>),
        (0x278 => _reserved9: [ReadWrite<u8>; 0x8]),
        (0x280 => pub MC_SMMU_VI_ASID_0: ReadWrite<u32>),
        (0x284 => pub MC_SMMU_VIC_ASID_0: ReadWrite<u32>),
        (0x288 => pub MC_SMMU_XUSB_HOST_ASID_0: ReadWrite<u32>),
        (0x28C => pub MC_SMMU_XUSB_DEV_ASID_0: ReadWrite<u32>),
        (0x290 => _reserved10: [ReadWrite<u8>; 0x4]),
        (0x294 => pub MC_SMMU_TSEC_ASID_0: ReadWrite<u32>),
        (0x298 => pub MC_SMMU_PPCS1_ASID_0: ReadWrite<u32>),
        (0x29C => _reserved11: [ReadWrite<u8>; 0x44]),
        (0x2E0 => pub MC_LATENCY_ALLOWANCE_AFI_0_0: ReadWrite<u32>),
        (0x2E4 => pub MC_LATENCY_ALLOWANCE_AVPC_0_0: ReadWrite<u32>),
        (0x2E8 => pub MC_LATENCY_ALLOWANCE_DC_0_0: ReadWrite<u32>),
        (0x2EC => pub MC_LATENCY_ALLOWANCE_DC_1_0: ReadWrite<u32>),
        (0x2F0 => pub MC_LATENCY_ALLOWANCE_DC_2_0: ReadWrite<u32>),
        (0x2F4 => pub MC_LATENCY_ALLOWANCE_DCB_0_0: ReadWrite<u32>),
        (0x2F8 => pub MC_LATENCY_ALLOWANCE_DCB_1_0: ReadWrite<u32>),
        (0x2FC => pub MC_LATENCY_ALLOWANCE_DCB_2_0: ReadWrite<u32>),
        (0x300 => _reserved12: [ReadWrite<u8>; 0x10]),
        (0x310 => pub MC_LATENCY_ALLOWANCE_HC_0_0: ReadWrite<u32>),
        (0x314 => pub MC_LATENCY_ALLOWANCE_HC_1_0: ReadWrite<u32>),
        (0x318 => pub MC_LATENCY_ALLOWANCE_HDA_0_0: ReadWrite<u32>),
        (0x31C => _reserved13: [ReadWrite<u8>; 0x4]),
        (0x320 => pub MC_LATENCY_ALLOWANCE_MPCORE_0_0: ReadWrite<u32>),
        (0x324 => _reserved14: [ReadWrite<u8>; 0x4]),
        (0x328 => pub MC_LATENCY_ALLOWANCE_NVENC_0_0: ReadWrite<u32>),
        (0x32C => _reserved15: [ReadWrite<u8>; 0x18]),
        (0x344 => pub MC_LATENCY_ALLOWANCE_PPCS_0_0: ReadWrite<u32>),
        (0x348 => pub MC_LATENCY_ALLOWANCE_PPCS_1_0: ReadWrite<u32>),
        (0x34C => pub MC_LATENCY_ALLOWANCE_PTC_0_0: ReadWrite<u32>),
        (0x350 => pub MC_LATENCY_ALLOWANCE_SATA_0_0: ReadWrite<u32>),
        (0x354 => _reserved16: [ReadWrite<u8>; 0x1C]),
        (0x370 => pub MC_LATENCY_ALLOWANCE_ISP2_0_0: ReadWrite<u32>),
        (0x374 => pub MC_LATENCY_ALLOWANCE_ISP2_1_0: ReadWrite<u32>),
        (0x378 => _reserved17: [ReadWrite<u8>; 0x4]),
        (0x37C => pub MC_LATENCY_ALLOWANCE_XUSB_0_0: ReadWrite<u32>),
        (0x380 => pub MC_LATENCY_ALLOWANCE_XUSB_1_0: ReadWrite<u32>),
        (0x384 => pub MC_LATENCY_ALLOWANCE_ISP2B_0_0: ReadWrite<u32>),
        (0x388 => pub MC_LATENCY_ALLOWANCE_ISP2B_1_0: ReadWrite<u32>),
        (0x38C => _reserved18: [ReadWrite<u8>; 0x4]),
        (0x390 => pub MC_LATENCY_ALLOWANCE_TSEC_0_0: ReadWrite<u32>),
        (0x394 => pub MC_LATENCY_ALLOWANCE_VIC_0_0: ReadWrite<u32>),
        (0x398 => pub MC_LATENCY_ALLOWANCE_VI2_0_0: ReadWrite<u32>),
        (0x39C => _reserved19: [ReadWrite<u8>; 0x4]),
        (0x3A0 => pub MC_LATENCY_ALLOWANCE_AXIAP_0_0: ReadWrite<u32>),
        (0x3A4 => pub MC_LATENCY_ALLOWANCE_A9AVP_0_0: ReadWrite<u32>),
        (0x3A8 => _reserved20: [ReadWrite<u8>; 0x4]),
        (0x3AC => pub MC_LATENCY_ALLOWANCE_GPU_0_0: ReadWrite<u32>),
        (0x3B0 => _reserved21: [ReadWrite<u8>; 0x8]),
        (0x3B8 => pub MC_LATENCY_ALLOWANCE_SDMMCA_0_0: ReadWrite<u32>),
        (0x3BC => pub MC_LATENCY_ALLOWANCE_SDMMCAA_0_0: ReadWrite<u32>),
        (0x3C0 => pub MC_LATENCY_ALLOWANCE_SDMMC_0_0: ReadWrite<u32>),
        (0x3C4 => pub MC_LATENCY_ALLOWANCE_SDMMCAB_0_0: ReadWrite<u32>),
        (0x3C8 => pub MC_LATENCY_ALLOWANCE_DC_3_0: ReadWrite<u32>),
        (0x3CC => _reserved22: [ReadWrite<u8>; 0xC]),
        (0x3D8 => pub MC_LATENCY_ALLOWANCE_NVDEC_0_0: ReadWrite<u32>),
        (0x3DC => pub MC_LATENCY_ALLOWANCE_APE_0_0: ReadWrite<u32>),
        (0x3E0 => pub MC_LATENCY_ALLOWANCE_SE_0_0: ReadWrite<u32>),
        (0x3E4 => pub MC_LATENCY_ALLOWANCE_NVJPG_0_0: ReadWrite<u32>),
        (0x3E8 => pub MC_LATENCY_ALLOWANCE_GPU2_0_0: ReadWrite<u32>),
        (0x3EC => pub MC_LATENCY_ALLOWANCE_ETR_0_0: ReadWrite<u32>),
        (0x3F0 => pub MC_LATENCY_ALLOWANCE_TSECB_0_0: ReadWrite<u32>),
        (0x3F4 => _reserved23: [ReadWrite<u8>; 0x24]),
        (0x418 => pub MC_VIDEO_PROTECT_VPR_OVERRIDE_0: ReadWrite<u32>),
        (0x41C => pub MC_DIS_PTSA_RATE_0: ReadWrite<u32>),
        (0x420 => pub MC_DIS_PTSA_MIN_0: ReadWrite<u32>),
        (0x424 => pub MC_DIS_PTSA_MAX_0: ReadWrite<u32>),
        (0x428 => pub MC_DISB_PTSA_RATE_0: ReadWrite<u32>),
        (0x42C => pub MC_DISB_PTSA_MIN_0: ReadWrite<u32>),
        (0x430 => pub MC_DISB_PTSA_MAX_0: ReadWrite<u32>),
        (0x434 => pub MC_VE_PTSA_RATE_0: ReadWrite<u32>),
        (0x438 => pub MC_VE_PTSA_MIN_0: ReadWrite<u32>),
        (0x43C => pub MC_VE_PTSA_MAX_0: ReadWrite<u32>),
        (0x440 => pub MC_RING2_PTSA_RATE_0: ReadWrite<u32>),
        (0x444 => pub MC_RING2_PTSA_MIN_0: ReadWrite<u32>),
        (0x448 => pub MC_RING2_PTSA_MAX_0: ReadWrite<u32>),
        (0x44C => pub MC_MLL_MPCORER_PTSA_RATE_0: ReadWrite<u32>),
        (0x450 => pub MC_MLL_MPCORER_PTSA_MIN_0: ReadWrite<u32>),
        (0x454 => pub MC_MLL_MPCORER_PTSA_MAX_0: ReadWrite<u32>),
        (0x458 => pub MC_SMMU_SMMU_PTSA_RATE_0: ReadWrite<u32>),
        (0x45C => pub MC_SMMU_SMMU_PTSA_MIN_0: ReadWrite<u32>),
        (0x460 => pub MC_SMMU_SMMU_PTSA_MAX_0: ReadWrite<u32>),
        (0x464 => _reserved24: [ReadWrite<u8>; 0x18]),
        (0x47C => pub MC_RING1_PTSA_RATE_0: ReadWrite<u32>),
        (0x480 => pub MC_RING1_PTSA_MIN_0: ReadWrite<u32>),
        (0x484 => pub MC_RING1_PTSA_MAX_0: ReadWrite<u32>),
        (0x488 => pub MC_A9AVPPC_PTSA_RATE_0: ReadWrite<u32>),
        (0x48C => pub MC_A9AVPPC_PTSA_MIN_0: ReadWrite<u32>),
        (0x490 => pub MC_A9AVPPC_PTSA_MAX_0: ReadWrite<u32>),
        (0x494 => pub MC_VE2_PTSA_RATE_0: ReadWrite<u32>),
        (0x498 => pub MC_VE2_PTSA_MIN_0: ReadWrite<u32>),
        (0x49C => pub MC_VE2_PTSA_MAX_0: ReadWrite<u32>),
        (0x4A0 => pub MC_ISP_PTSA_RATE_0: ReadWrite<u32>),
        (0x4A4 => pub MC_ISP_PTSA_MIN_0: ReadWrite<u32>),
        (0x4A8 => pub MC_ISP_PTSA_MAX_0: ReadWrite<u32>),
        (0x4AC => pub MC_PCX_PTSA_RATE_0: ReadWrite<u32>),
        (0x4B0 => pub MC_PCX_PTSA_MIN_0: ReadWrite<u32>),
        (0x4B4 => pub MC_PCX_PTSA_MAX_0: ReadWrite<u32>),
        (0x4B8 => pub MC_SAX_PTSA_RATE_0: ReadWrite<u32>),
        (0x4BC => pub MC_SAX_PTSA_MIN_0: ReadWrite<u32>),
        (0x4C0 => pub MC_SAX_PTSA_MAX_0: ReadWrite<u32>),
        (0x4C4 => pub MC_MSE_PTSA_RATE_0: ReadWrite<u32>),
        (0x4C8 => pub MC_MSE_PTSA_MIN_0: ReadWrite<u32>),
        (0x4CC => pub MC_MSE_PTSA_MAX_0: ReadWrite<u32>),
        (0x4D0 => pub MC_SD_PTSA_RATE_0: ReadWrite<u32>),
        (0x4D4 => pub MC_SD_PTSA_MIN_0: ReadWrite<u32>),
        (0x4D8 => pub MC_SD_PTSA_MAX_0: ReadWrite<u32>),
        (0x4DC => pub MC_AHB_PTSA_RATE_0: ReadWrite<u32>),
        (0x4E0 => pub MC_AHB_PTSA_MIN_0: ReadWrite<u32>),
        (0x4E4 => pub MC_AHB_PTSA_MAX_0: ReadWrite<u32>),
        (0x4E8 => pub MC_APB_PTSA_RATE_0: ReadWrite<u32>),
        (0x4EC => pub MC_APB_PTSA_MIN_0: ReadWrite<u32>),
        (0x4F0 => pub MC_APB_PTSA_MAX_0: ReadWrite<u32>),
        (0x4F4 => pub MC_AVP_PTSA_RATE_0: ReadWrite<u32>),
        (0x4F8 => pub MC_AVP_PTSA_MIN_0: ReadWrite<u32>),
        (0x4FC => pub MC_AVP_PTSA_MAX_0: ReadWrite<u32>),
        (0x500 => _reserved25: [ReadWrite<u8>; 0xC]),
        (0x50C => pub MC_FTOP_PTSA_RATE_0: ReadWrite<u32>),
        (0x510 => pub MC_FTOP_PTSA_MIN_0: ReadWrite<u32>),
        (0x514 => pub MC_FTOP_PTSA_MAX_0: ReadWrite<u32>),
        (0x518 => pub MC_HOST_PTSA_RATE_0: ReadWrite<u32>),
        (0x51C => pub MC_HOST_PTSA_MIN_0: ReadWrite<u32>),
        (0x520 => pub MC_HOST_PTSA_MAX_0: ReadWrite<u32>),
        (0x524 => pub MC_USBX_PTSA_RATE_0: ReadWrite<u32>),
        (0x528 => pub MC_USBX_PTSA_MIN_0: ReadWrite<u32>),
        (0x52C => pub MC_USBX_PTSA_MAX_0: ReadWrite<u32>),
        (0x530 => pub MC_USBD_PTSA_RATE_0: ReadWrite<u32>),
        (0x534 => pub MC_USBD_PTSA_MIN_0: ReadWrite<u32>),
        (0x538 => pub MC_USBD_PTSA_MAX_0: ReadWrite<u32>),
        (0x53C => pub MC_GK_PTSA_RATE_0: ReadWrite<u32>),
        (0x540 => pub MC_GK_PTSA_MIN_0: ReadWrite<u32>),
        (0x544 => pub MC_GK_PTSA_MAX_0: ReadWrite<u32>),
        (0x548 => pub MC_AUD_PTSA_RATE_0: ReadWrite<u32>),
        (0x54C => pub MC_AUD_PTSA_MIN_0: ReadWrite<u32>),
        (0x550 => pub MC_AUD_PTSA_MAX_0: ReadWrite<u32>),
        (0x554 => pub MC_VICPC_PTSA_RATE_0: ReadWrite<u32>),
        (0x558 => pub MC_VICPC_PTSA_MIN_0: ReadWrite<u32>),
        (0x55C => pub MC_VICPC_PTSA_MAX_0: ReadWrite<u32>),
        (0x560 => _reserved26: [ReadWrite<u8>; 0x24]),
        (0x584 => pub MC_JPG_PTSA_RATE_0: ReadWrite<u32>),
        (0x588 => pub MC_JPG_PTSA_MIN_0: ReadWrite<u32>),
        (0x58C => pub MC_JPG_PTSA_MAX_0: ReadWrite<u32>),
        (0x590 => pub MC_VIDEO_PROTECT_VPR_OVERRIDE1_0: ReadWrite<u32>),
        (0x594 => _reserved27: [ReadWrite<u8>; 0x6C]),
        (0x600 => pub MC_SMMU_TLB_SET_SELECTION_MASK_0_0: ReadWrite<u32>),
        (0x604 => _reserved28: [ReadWrite<u8>; 0x4]),
        (0x608 => pub MC_DISPLAY_SNAP_RING_0: ReadWrite<u32>),
        (0x60C => _reserved29: [ReadWrite<u8>; 0x4]),
        (0x610 => pub MC_GK2_PTSA_RATE_0: ReadWrite<u32>),
        (0x614 => pub MC_GK2_PTSA_MIN_0: ReadWrite<u32>),
        (0x618 => pub MC_GK2_PTSA_MAX_0: ReadWrite<u32>),
        (0x61C => pub MC_SDM_PTSA_RATE_0: ReadWrite<u32>),
        (0x620 => pub MC_SDM_PTSA_MIN_0: ReadWrite<u32>),
        (0x624 => pub MC_SDM_PTSA_MAX_0: ReadWrite<u32>),
        (0x628 => pub MC_HDAPC_PTSA_RATE_0: ReadWrite<u32>),
        (0x62C => pub MC_HDAPC_PTSA_MIN_0: ReadWrite<u32>),
        (0x630 => pub MC_HDAPC_PTSA_MAX_0: ReadWrite<u32>),
        (0x634 => pub MC_DFD_PTSA_RATE_0: ReadWrite<u32>),
        (0x638 => pub MC_DFD_PTSA_MIN_0: ReadWrite<u32>),
        (0x63C => pub MC_DFD_PTSA_MAX_0: ReadWrite<u32>),
        (0x640 => _reserved30: [ReadWrite<u8>; 0x8]),
        (0x648 => pub MC_VIDEO_PROTECT_BOM_0: ReadWrite<u32>),
        (0x64C => pub MC_VIDEO_PROTECT_SIZE_MB_0: ReadWrite<u32>),
        (0x650 => pub MC_VIDEO_PROTECT_REG_CTRL_0: ReadWrite<u32>),
        (0x654 => pub MC_ERR_VPR_STATUS_0: ReadOnly<u32>),
        (0x658 => pub MC_ERR_VPR_ADR_0: ReadOnly<u32>),
        (0x65C => pub MC_IRAM_BOM_0: ReadWrite<u32>),
        (0x660 => pub MC_IRAM_TOM_0: ReadWrite<u32>),
        (0x664 => pub MC_EMEM_CFG_ACCESS_CTRL_0: ReadWrite<u32>),
        (0x668 => pub MC_TZ_SECURITY_CTRL_0: ReadWrite<u32>),
        (0x66C => pub MC_EMEM_ARB_OUTSTANDING_REQ_RING3_0: ReadWrite<u32>),
        (0x670 => pub MC_SEC_CARVEOUT_BOM_0: ReadWrite<u32>),
        (0x674 => pub MC_SEC_CARVEOUT_SIZE_MB_0: ReadWrite<u32>),
        (0x678 => pub MC_SEC_CARVEOUT_REG_CTRL_0: ReadWrite<u32>),
        (0x67C => pub MC_ERR_SEC_STATUS_0: ReadOnly<u32>),
        (0x680 => pub MC_ERR_SEC_ADR_0: ReadOnly<u32>),
        (0x684 => pub MC_PC_IDLE_CLOCK_GATE_CONFIG_0: ReadWrite<u32>),
        (0x688 => pub MC_STUTTER_CONTROL_0: ReadWrite<u32>),
        (0x68C => _reserved32: [ReadWrite<u8>; 0x4]),
        (0x690 => pub MC_SCALED_LATENCY_ALLOWANCE_DISPLAY0A_0: ReadWrite<u32>),
        (0x694 => pub MC_SCALED_LATENCY_ALLOWANCE_DISPLAY0AB_0: ReadWrite<u32>),
        (0x698 => pub MC_SCALED_LATENCY_ALLOWANCE_DISPLAY0B_0: ReadWrite<u32>),
        (0x69C => pub MC_SCALED_LATENCY_ALLOWANCE_DISPLAY0BB_0: ReadWrite<u32>),
        (0x6A0 => pub MC_SCALED_LATENCY_ALLOWANCE_DISPLAY0C_0: ReadWrite<u32>),
        (0x6A4 => pub MC_SCALED_LATENCY_ALLOWANCE_DISPLAY0CB_0: ReadWrite<u32>),
        (0x6A8 => _reserved33: [ReadWrite<u8>; 0x8]),
        (0x6B0 => pub MC_EMEM_ARB_NISO_THROTTLE_0: ReadWrite<u32>),
        (0x6B4 => pub MC_EMEM_ARB_OUTSTANDING_REQ_NISO_0: ReadWrite<u32>),
        (0x6B8 => pub MC_EMEM_ARB_NISO_THROTTLE_MASK_0: ReadWrite<u32>),
        (0x6BC => pub MC_EMEM_ARB_RING0_THROTTLE_MASK_0: ReadWrite<u32>),
        (0x6C0 => pub MC_EMEM_ARB_TIMING_CCDMW_0: ReadWrite<u32>),
        (0x6C4 => pub MC_EMEM_ARB_TIMING_RFCPB_0: ReadWrite<u32>),
        (0x6C8 => _reserved34: [ReadWrite<u8>; 0x28]),
        (0x6F0 => pub MC_EMEM_ARB_REFPB_HP_CTRL_0: ReadWrite<u32>),
        (0x6F4 => pub MC_EMEM_ARB_REFPB_BANK_CTRL_0: ReadWrite<u32>),
        (0x6F8 => _reserved35: [ReadWrite<u8>; 0x268]),
        (0x960 => pub MC_PTSA_GRANT_DECREMENT_0: ReadWrite<u32>),
        (0x964 => pub MC_IRAM_REG_CTRL_0: ReadWrite<u32>),
        (0x968 => pub MC_EMEM_ARB_OVERRIDE_1_0: ReadWrite<u32>),
        (0x96C => _reserved36: [ReadWrite<u8>; 0x4]),
        (0x970 => pub MC_CLIENT_HOTRESET_CTRL_1_0: ReadWrite<u32>),
        (0x974 => pub MC_CLIENT_HOTRESET_STATUS_1_0: ReadOnly<u32>),
        (0x978 => _reserved37: [ReadWrite<u8>; 0xC]),
        (0x984 => pub MC_VIDEO_PROTECT_GPU_OVERRIDE_0_0: ReadWrite<u32>),
        (0x988 => pub MC_VIDEO_PROTECT_GPU_OVERRIDE_1_0: ReadWrite<u32>),
        (0x98C => _reserved38: [ReadWrite<u8>; 0x14]),
        (0x9A0 => pub MC_MTS_CARVEOUT_BOM_0: ReadWrite<u32>),
        (0x9A4 => pub MC_MTS_CARVEOUT_SIZE_MB_0: ReadWrite<u32>),
        (0x9A8 => pub MC_MTS_CARVEOUT_ADR_HI_0: ReadWrite<u32>),
        (0x9AC => pub MC_MTS_CARVEOUT_REG_CTRL_0: ReadWrite<u32>),
        (0x9B0 => _reserved39: [ReadWrite<u8>; 0x8]),
        (0x9B8 => pub MC_SMMU_PTC_FLUSH_1_0: ReadWrite<u32>),
        (0x9BC => pub MC_SECURITY_CFG3_0: ReadWrite<u32>),
        (0x9C0 => pub MC_EMEM_BANK_SWIZZLE_CFG0_0: ReadWrite<u32>),
        (0x9C4 => pub MC_EMEM_BANK_SWIZZLE_CFG1_0: ReadWrite<u32>),
        (0x9C8 => pub MC_EMEM_BANK_SWIZZLE_CFG2_0: ReadWrite<u32>),
        (0x9CC => pub MC_EMEM_BANK_SWIZZLE_CFG3_0: ReadWrite<u32>),
        (0x9D0 => _reserved40: [ReadWrite<u8>; 0x4]),
        (0x9D4 => pub MC_SEC_CARVEOUT_ADR_HI_0: ReadWrite<u32>),
        (0x9D8 => _reserved41: [ReadWrite<u8>; 0x4]),
        (0x9DC => pub MC_DA_CONFIG0_0: ReadWrite<u32>),
        (0x9E0 => pub MC_SMMU_ASID_SECURITY_2_0: ReadWrite<u32>),
        (0x9E4 => pub MC_SMMU_ASID_SECURITY_3_0: ReadWrite<u32>),
        (0x9E8 => pub MC_SMMU_ASID_SECURITY_4_0: ReadWrite<u32>),
        (0x9EC => pub MC_SMMU_ASID_SECURITY_5_0: ReadWrite<u32>),
        (0x9F0 => pub MC_SMMU_ASID_SECURITY_6_0: ReadWrite<u32>),
        (0x9F4 => pub MC_SMMU_ASID_SECURITY_7_0: ReadWrite<u32>),
        (0x9F8 => _reserved42: [ReadWrite<u8>; 0x90]),
        (0xA88 => pub MC_SMMU_DC1_ASID_0: ReadWrite<u32>),
        (0xA8C => _reserved43: [ReadWrite<u8>; 0x8]),
        (0xA94 => pub MC_SMMU_SDMMC1A_ASID_0: ReadWrite<u32>),
        (0xA98 => pub MC_SMMU_SDMMC2A_ASID_0: ReadWrite<u32>),
        (0xA9C => pub MC_SMMU_SDMMC3A_ASID_0: ReadWrite<u32>),
        (0xAA0 => pub MC_SMMU_SDMMC4A_ASID_0: ReadWrite<u32>),
        (0xAA4 => pub MC_SMMU_ISP2B_ASID_0: ReadWrite<u32>),
        (0xAA8 => pub MC_SMMU_GPU_ASID_0: ReadWrite<u32>),
        (0xAAC => pub MC_SMMU_GPUB_ASID_0: ReadWrite<u32>),
        (0xAB0 => pub MC_SMMU_PPCS2_ASID_0: ReadWrite<u32>),
        (0xAB4 => pub MC_SMMU_NVDEC_ASID_0: ReadWrite<u32>),
        (0xAB8 => pub MC_SMMU_APE_ASID_0: ReadWrite<u32>),
        (0xABC => pub MC_SMMU_SE_ASID_0: ReadWrite<u32>),
        (0xAC0 => pub MC_SMMU_NVJPG_ASID_0: ReadWrite<u32>),
        (0xAC4 => pub MC_SMMU_HC1_ASID_0: ReadWrite<u32>),
        (0xAC8 => pub MC_SMMU_SE1_ASID_0: ReadWrite<u32>),
        (0xACC => pub MC_SMMU_AXIAP_ASID_0: ReadWrite<u32>),
        (0xAD0 => pub MC_SMMU_ETR_ASID_0: ReadWrite<u32>),
        (0xAD4 => pub MC_SMMU_TSECB_ASID_0: ReadWrite<u32>),
        (0xAD8 => pub MC_SMMU_TSEC1_ASID_0: ReadWrite<u32>),
        (0xADC => pub MC_SMMU_TSECB1_ASID_0: ReadWrite<u32>),
        (0xAE0 => pub MC_SMMU_NVDEC1_ASID_0: ReadWrite<u32>),
        (0xAE4 => _reserved44: [ReadWrite<u8>; 0x9C]),
        (0xB80 => pub MC_EMEM_ARB_NISO_THROTTLE_MASK_1_0: ReadWrite<u32>),
        (0xB84 => pub MC_EMEM_ARB_HYSTERESIS_4_0: ReadWrite<u32>),
        (0xB88 => _reserved45: [ReadWrite<u8>; 0xC]),
        (0xB94 => pub MC_EMEM_ARB_ISOCHRONOUS_4_0: ReadWrite<u32>),
        (0xB98 => pub MC_SMMU_TRANSLATION_ENABLE_4_0: ReadWrite<u32>),
        (0xB9C => _reserved46: [ReadWrite<u8>; 0x14]),
        (0xBB0 => pub MC_EMEM_ARB_DHYSTERESIS_0_0: ReadWrite<u32>),
        (0xBB4 => pub MC_EMEM_ARB_DHYSTERESIS_1_0: ReadWrite<u32>),
        (0xBB8 => pub MC_EMEM_ARB_DHYSTERESIS_2_0: ReadWrite<u32>),
        (0xBBC => pub MC_EMEM_ARB_DHYSTERESIS_3_0: ReadWrite<u32>),
        (0xBC0 => pub MC_EMEM_ARB_DHYSTERESIS_4_0: ReadWrite<u32>),
        (0xBC4 => _reserved47: [ReadWrite<u8>; 0x8]),
        (0xBCC => pub MC_EMEM_ARB_DHYST_CTRL_0: ReadWrite<u32>),
        (0xBD0 => pub MC_EMEM_ARB_DHYST_TIMEOUT_UTIL_0_0: ReadWrite<u32>),
        (0xBD4 => pub MC_EMEM_ARB_DHYST_TIMEOUT_UTIL_1_0: ReadWrite<u32>),
        (0xBD8 => pub MC_EMEM_ARB_DHYST_TIMEOUT_UTIL_2_0: ReadWrite<u32>),
        (0xBDC => pub MC_EMEM_ARB_DHYST_TIMEOUT_UTIL_3_0: ReadWrite<u32>),
        (0xBE0 => pub MC_EMEM_ARB_DHYST_TIMEOUT_UTIL_4_0: ReadWrite<u32>),
        (0xBE4 => pub MC_EMEM_ARB_DHYST_TIMEOUT_UTIL_5_0: ReadWrite<u32>),
        (0xBE8 => pub MC_EMEM_ARB_DHYST_TIMEOUT_UTIL_6_0: ReadWrite<u32>),
        (0xBEC => pub MC_EMEM_ARB_DHYST_TIMEOUT_UTIL_7_0: ReadWrite<u32>),
        (0xBF0 => _reserved48: [ReadWrite<u8>; 0x18]),
        (0xC08 => pub MC_SECURITY_CARVEOUT1_CFG0_0: ReadWrite<u32>),
        (0xC0C => pub MC_SECURITY_CARVEOUT1_BOM_0: ReadWrite<u32>),
        (0xC10 => pub MC_SECURITY_CARVEOUT1_BOM_HI_0: ReadWrite<u32>),
        (0xC14 => pub MC_SECURITY_CARVEOUT1_SIZE_128KB_0: ReadWrite<u32>),
        (0xC18 => pub MC_SECURITY_CARVEOUT1_CA0_0: ReadWrite<u32>),
        (0xC1C => pub MC_SECURITY_CARVEOUT1_CA1_0: ReadWrite<u32>),
        (0xC20 => pub MC_SECURITY_CARVEOUT1_CA2_0: ReadWrite<u32>),
        (0xC24 => pub MC_SECURITY_CARVEOUT1_CA3_0: ReadWrite<u32>),
        (0xC28 => pub MC_SECURITY_CARVEOUT1_CA4_0: ReadWrite<u32>),
        (0xC2C => pub MC_SECURITY_CARVEOUT1_CFIA0_0: ReadWrite<u32>),
        (0xC30 => pub MC_SECURITY_CARVEOUT1_CFIA1_0: ReadWrite<u32>),
        (0xC34 => pub MC_SECURITY_CARVEOUT1_CFIA2_0: ReadWrite<u32>),
        (0xC38 => pub MC_SECURITY_CARVEOUT1_CFIA3_0: ReadWrite<u32>),
        (0xC3C => pub MC_SECURITY_CARVEOUT1_CFIA4_0: ReadWrite<u32>),
        (0xC40 => _reserved49: [ReadWrite<u8>; 0x18]),
        (0xC58 => pub MC_SECURITY_CARVEOUT2_CFG0_0: ReadWrite<u32>),
        (0xC5C => pub MC_SECURITY_CARVEOUT2_BOM_0: ReadWrite<u32>),
        (0xC60 => pub MC_SECURITY_CARVEOUT2_BOM_HI_0: ReadWrite<u32>),
        (0xC64 => pub MC_SECURITY_CARVEOUT2_SIZE_128KB_0: ReadWrite<u32>),
        (0xC68 => pub MC_SECURITY_CARVEOUT2_CA0_0: ReadWrite<u32>),
        (0xC6C => pub MC_SECURITY_CARVEOUT2_CA1_0: ReadWrite<u32>),
        (0xC70 => pub MC_SECURITY_CARVEOUT2_CA2_0: ReadWrite<u32>),
        (0xC74 => pub MC_SECURITY_CARVEOUT2_CA3_0: ReadWrite<u32>),
        (0xC78 => pub MC_SECURITY_CARVEOUT2_CA4_0: ReadWrite<u32>),
        (0xC7C => pub MC_SECURITY_CARVEOUT2_CFIA0_0: ReadWrite<u32>),
        (0xC80 => pub MC_SECURITY_CARVEOUT2_CFIA1_0: ReadWrite<u32>),
        (0xC84 => pub MC_SECURITY_CARVEOUT2_CFIA2_0: ReadWrite<u32>),
        (0xC88 => pub MC_SECURITY_CARVEOUT2_CFIA3_0: ReadWrite<u32>),
        (0xC8C => pub MC_SECURITY_CARVEOUT2_CFIA4_0: ReadWrite<u32>),
        (0xC90 => _reserved50: [ReadWrite<u8>; 0x18]),
        (0xCA8 => pub MC_SECURITY_CARVEOUT3_CFG0_0: ReadWrite<u32>),
        (0xCAC => pub MC_SECURITY_CARVEOUT3_BOM_0: ReadWrite<u32>),
        (0xCB0 => pub MC_SECURITY_CARVEOUT3_BOM_HI_0: ReadWrite<u32>),
        (0xCB4 => pub MC_SECURITY_CARVEOUT3_SIZE_128KB_0: ReadWrite<u32>),
        (0xCB8 => pub MC_SECURITY_CARVEOUT3_CA0_0: ReadWrite<u32>),
        (0xCBC => pub MC_SECURITY_CARVEOUT3_CA1_0: ReadWrite<u32>),
        (0xCC0 => pub MC_SECURITY_CARVEOUT3_CA2_0: ReadWrite<u32>),
        (0xCC4 => pub MC_SECURITY_CARVEOUT3_CA3_0: ReadWrite<u32>),
        (0xCC8 => pub MC_SECURITY_CARVEOUT3_CA4_0: ReadWrite<u32>),
        (0xCCC => pub MC_SECURITY_CARVEOUT3_CFIA0_0: ReadWrite<u32>),
        (0xCD0 => pub MC_SECURITY_CARVEOUT3_CFIA1_0: ReadWrite<u32>),
        (0xCD4 => pub MC_SECURITY_CARVEOUT3_CFIA2_0: ReadWrite<u32>),
        (0xCD8 => pub MC_SECURITY_CARVEOUT3_CFIA3_0: ReadWrite<u32>),
        (0xCDC => pub MC_SECURITY_CARVEOUT3_CFIA4_0: ReadWrite<u32>),
        (0xCE0 => _reserved51: [ReadWrite<u8>; 0x18]),
        (0xCF8 => pub MC_SECURITY_CARVEOUT4_CFG0_0: ReadWrite<u32>),
        (0xCFC => pub MC_SECURITY_CARVEOUT4_BOM_0: ReadWrite<u32>),
        (0xD00 => pub MC_SECURITY_CARVEOUT4_BOM_HI_0: ReadWrite<u32>),
        (0xD04 => pub MC_SECURITY_CARVEOUT4_SIZE_128KB_0: ReadWrite<u32>),
        (0xD08 => pub MC_SECURITY_CARVEOUT4_CA0_0: ReadWrite<u32>),
        (0xD0C => pub MC_SECURITY_CARVEOUT4_CA1_0: ReadWrite<u32>),
        (0xD10 => pub MC_SECURITY_CARVEOUT4_CA2_0: ReadWrite<u32>),
        (0xD14 => pub MC_SECURITY_CARVEOUT4_CA3_0: ReadWrite<u32>),
        (0xD18 => pub MC_SECURITY_CARVEOUT4_CA4_0: ReadWrite<u32>),
        (0xD1C => pub MC_SECURITY_CARVEOUT4_CFIA0_0: ReadWrite<u32>),
        (0xD20 => pub MC_SECURITY_CARVEOUT4_CFIA1_0: ReadWrite<u32>),
        (0xD24 => pub MC_SECURITY_CARVEOUT4_CFIA2_0: ReadWrite<u32>),
        (0xD28 => pub MC_SECURITY_CARVEOUT4_CFIA3_0: ReadWrite<u32>),
        (0xD2C => pub MC_SECURITY_CARVEOUT4_CFIA4_0: ReadWrite<u32>),
        (0xD30 => _reserved52: [ReadWrite<u8>; 0x18]),
        (0xD48 => pub MC_SECURITY_CARVEOUT5_CFG0_0: ReadWrite<u32>),
        (0xD4C => pub MC_SECURITY_CARVEOUT5_BOM_0: ReadWrite<u32>),
        (0xD50 => pub MC_SECURITY_CARVEOUT5_BOM_HI_0: ReadWrite<u32>),
        (0xD54 => pub MC_SECURITY_CARVEOUT5_SIZE_128KB_0: ReadWrite<u32>),
        (0xD58 => pub MC_SECURITY_CARVEOUT5_CA0_0: ReadWrite<u32>),
        (0xD5C => pub MC_SECURITY_CARVEOUT5_CA1_0: ReadWrite<u32>),
        (0xD60 => pub MC_SECURITY_CARVEOUT5_CA2_0: ReadWrite<u32>),
        (0xD64 => pub MC_SECURITY_CARVEOUT5_CA3_0: ReadWrite<u32>),
        (0xD68 => pub MC_SECURITY_CARVEOUT5_CA4_0: ReadWrite<u32>),
        (0xD6C => pub MC_SECURITY_CARVEOUT5_CFIA0_0: ReadWrite<u32>),
        (0xD70 => pub MC_SECURITY_CARVEOUT5_CFIA1_0: ReadWrite<u32>),
        (0xD74 => pub MC_SECURITY_CARVEOUT5_CFIA2_0: ReadWrite<u32>),
        (0xD78 => pub MC_SECURITY_CARVEOUT5_CFIA3_0: ReadWrite<u32>),
        (0xD7C => pub MC_SECURITY_CARVEOUT5_CFIA4_0: ReadWrite<u32>),
        (0xD80 => @END),
    }
}

assert_eq_size!(Registers, [u8; 0xD80]);
