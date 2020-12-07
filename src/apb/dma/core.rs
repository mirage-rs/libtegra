use register::{mmio::*, register_bitfields, register_structs};

use crate::memory_map::apb_dma;

/// A pointer to the APB DMA register block that can be accessed by dereferencing it.
pub const REGISTERS: *const Registers = apb_dma::BASE as *const Registers;

register_bitfields! {
    u32,

    /// Bitfields of the `APBDMA_COMMAND_0` register.
    pub APBDMA_COMMAND_0 [
        /// Enables Global APB-DMA.
        GEN OFFSET(31) NUMBITS(1) []
    ],

    /// Bitfields of the `APBDMA_STATUS_0` register.
    pub APBDMA_STATUS_0 [
        /// DMA Channel 31 Status.
        BSY_31 OFFSET(31) NUMBITS(1) [
            NotBusy = 0,
            Busy = 1
        ],

        /// DMA Channel 30 Status.
        BSY_30 OFFSET(30) NUMBITS(1) [
            NotBusy = 0,
            Busy = 1
        ],

        /// DMA Channel 29 Status.
        BSY_29 OFFSET(29) NUMBITS(1) [
            NotBusy = 0,
            Busy = 1
        ],

        /// DMA Channel 28 Status.
        BSY_28 OFFSET(28) NUMBITS(1) [
            NotBusy = 0,
            Busy = 1
        ],

        /// DMA Channel 27 Status.
        BSY_27 OFFSET(27) NUMBITS(1) [
            NotBusy = 0,
            Busy = 1
        ],

        /// DMA Channel 26 Status.
        BSY_26 OFFSET(26) NUMBITS(1) [
            NotBusy = 0,
            Busy = 1
        ],

        /// DMA Channel 25 Status.
        BSY_25 OFFSET(25) NUMBITS(1) [
            NotBusy = 0,
            Busy = 1
        ],

        /// DMA Channel 24 Status.
        BSY_24 OFFSET(24) NUMBITS(1) [
            NotBusy = 0,
            Busy = 1
        ],

        /// DMA Channel 23 Status.
        BSY_23 OFFSET(23) NUMBITS(1) [
            NotBusy = 0,
            Busy = 1
        ],

        /// DMA Channel 22 Status.
        BSY_22 OFFSET(22) NUMBITS(1) [
            NotBusy = 0,
            Busy = 1
        ],

        /// DMA Channel 21 Status.
        BSY_21 OFFSET(21) NUMBITS(1) [
            NotBusy = 0,
            Busy = 1
        ],

        /// DMA Channel 20 Status.
        BSY_20 OFFSET(20) NUMBITS(1) [
            NotBusy = 0,
            Busy = 1
        ],

        /// DMA Channel 19 Status.
        BSY_19 OFFSET(19) NUMBITS(1) [
            NotBusy = 0,
            Busy = 1
        ],

        /// DMA Channel 18 Status.
        BSY_18 OFFSET(18) NUMBITS(1) [
            NotBusy = 0,
            Busy = 1
        ],

        /// DMA Channel 17 Status.
        BSY_17 OFFSET(17) NUMBITS(1) [
            NotBusy = 0,
            Busy = 1
        ],

        /// DMA Channel 16 Status.
        BSY_16 OFFSET(16) NUMBITS(1) [
            NotBusy = 0,
            Busy = 1
        ],

        /// DMA Channel 15 Status.
        BSY_15 OFFSET(15) NUMBITS(1) [
            NotBusy = 0,
            Busy = 1
        ],

        /// DMA Channel 14 Status.
        BSY_14 OFFSET(14) NUMBITS(1) [
            NotBusy = 0,
            Busy = 1
        ],

        /// DMA Channel 13 Status.
        BSY_13 OFFSET(13) NUMBITS(1) [
            NotBusy = 0,
            Busy = 1
        ],

        /// DMA Channel 12 Status.
        BSY_12 OFFSET(12) NUMBITS(1) [
            NotBusy = 0,
            Busy = 1
        ],

        /// DMA Channel 11 Status.
        BSY_11 OFFSET(11) NUMBITS(1) [
            NotBusy = 0,
            Busy = 1
        ],

        /// DMA Channel 10 Status.
        BSY_10 OFFSET(10) NUMBITS(1) [
            NotBusy = 0,
            Busy = 1
        ],

        /// DMA Channel 9 Status.
        BSY_9 OFFSET(9) NUMBITS(1) [
            NotBusy = 0,
            Busy = 1
        ],

        /// DMA Channel 8 Status.
        BSY_8 OFFSET(8) NUMBITS(1) [
            NotBusy = 0,
            Busy = 1
        ],

        /// DMA Channel 7 Status.
        BSY_7 OFFSET(7) NUMBITS(1) [
            NotBusy = 0,
            Busy = 1
        ],

        /// DMA Channel 6 Status.
        BSY_6 OFFSET(6) NUMBITS(1) [
            NotBusy = 0,
            Busy = 1
        ],

        /// DMA Channel 5 Status.
        BSY_5 OFFSET(5) NUMBITS(1) [
            NotBusy = 0,
            Busy = 1
        ],

        /// DMA Channel 4 Status.
        BSY_4 OFFSET(4) NUMBITS(1) [
            NotBusy = 0,
            Busy = 1
        ],

        /// DMA Channel 3 Status.
        BSY_3 OFFSET(3) NUMBITS(1) [
            NotBusy = 0,
            Busy = 1
        ],

        /// DMA Channel 2 Status.
        BSY_2 OFFSET(2) NUMBITS(1) [
            NotBusy = 0,
            Busy = 1
        ],

        /// DMA Channel 1 Status.
        BSY_1 OFFSET(1) NUMBITS(1) [
            NotBusy = 0,
            Busy = 1
        ],

        /// DMA Channel 0 Status.
        BSY_0 OFFSET(0) NUMBITS(1) [
            NotBusy = 0,
            Busy = 1
        ]
    ],

    /// Bitfields of the `APBDMA_CNTRL_REG_0` register.
    pub APBDMA_CNTRL_REG_0 [
        /// DMA COUNT Value.
        COUNT_VALUE OFFSET(0) NUMBITS(16) []
    ],

    /// Bitfields of the `APBDMA_IRQ_STA_CPU_0` register.
    pub APBDMA_IRQ_STA_CPU_0 [
        /// Gathers all the after-masking CPU directed IRQ status bits from Channel 31.
        CH31 OFFSET(31) NUMBITS(1) [],

        /// Gathers all the after-masking CPU directed IRQ status bits from Channel 30.
        CH30 OFFSET(30) NUMBITS(1) [],

        /// Gathers all the after-masking CPU directed IRQ status bits from Channel 29.
        CH29 OFFSET(29) NUMBITS(1) [],

        /// Gathers all the after-masking CPU directed IRQ status bits from Channel 28.
        CH28 OFFSET(28) NUMBITS(1) [],

        /// Gathers all the after-masking CPU directed IRQ status bits from Channel 27.
        CH27 OFFSET(27) NUMBITS(1) [],

        /// Gathers all the after-masking CPU directed IRQ status bits from Channel 26.
        CH26 OFFSET(26) NUMBITS(1) [],

        /// Gathers all the after-masking CPU directed IRQ status bits from Channel 25.
        CH25 OFFSET(25) NUMBITS(1) [],

        /// Gathers all the after-masking CPU directed IRQ status bits from Channel 24.
        CH24 OFFSET(24) NUMBITS(1) [],

        /// Gathers all the after-masking CPU directed IRQ status bits from Channel 23.
        CH23 OFFSET(23) NUMBITS(1) [],

        /// Gathers all the after-masking CPU directed IRQ status bits from Channel 22.
        CH22 OFFSET(22) NUMBITS(1) [],

        /// Gathers all the after-masking CPU directed IRQ status bits from Channel 21.
        CH21 OFFSET(21) NUMBITS(1) [],

        /// Gathers all the after-masking CPU directed IRQ status bits from Channel 20.
        CH20 OFFSET(20) NUMBITS(1) [],

        /// Gathers all the after-masking CPU directed IRQ status bits from Channel 19.
        CH19 OFFSET(19) NUMBITS(1) [],

        /// Gathers all the after-masking CPU directed IRQ status bits from Channel 18.
        CH18 OFFSET(18) NUMBITS(1) [],

        /// Gathers all the after-masking CPU directed IRQ status bits from Channel 17.
        CH17 OFFSET(17) NUMBITS(1) [],

        /// Gathers all the after-masking CPU directed IRQ status bits from Channel 16.
        CH16 OFFSET(16) NUMBITS(1) [],

        /// Gathers all the after-masking CPU directed IRQ status bits from Channel 15.
        CH15 OFFSET(15) NUMBITS(1) [],

        /// Gathers all the after-masking CPU directed IRQ status bits from Channel 14.
        CH14 OFFSET(14) NUMBITS(1) [],

        /// Gathers all the after-masking CPU directed IRQ status bits from Channel 13.
        CH13 OFFSET(13) NUMBITS(1) [],

        /// Gathers all the after-masking CPU directed IRQ status bits from Channel 12.
        CH12 OFFSET(12) NUMBITS(1) [],

        /// Gathers all the after-masking CPU directed IRQ status bits from Channel 11.
        CH11 OFFSET(11) NUMBITS(1) [],

        /// Gathers all the after-masking CPU directed IRQ status bits from Channel 10.
        CH10 OFFSET(10) NUMBITS(1) [],

        /// Gathers all the after-masking CPU directed IRQ status bits from Channel 9.
        CH9 OFFSET(9) NUMBITS(1) [],

        /// Gathers all the after-masking CPU directed IRQ status bits from Channel 8.
        CH8 OFFSET(8) NUMBITS(1) [],

        /// Gathers all the after-masking CPU directed IRQ status bits from Channel 7.
        CH7 OFFSET(7) NUMBITS(1) [],

        /// Gathers all the after-masking CPU directed IRQ status bits from Channel 6.
        CH6 OFFSET(6) NUMBITS(1) [],

        /// Gathers all the after-masking CPU directed IRQ status bits from Channel 5.
        CH5 OFFSET(5) NUMBITS(1) [],

        /// Gathers all the after-masking CPU directed IRQ status bits from Channel 4.
        CH4 OFFSET(4) NUMBITS(1) [],

        /// Gathers all the after-masking CPU directed IRQ status bits from Channel 3.
        CH3 OFFSET(3) NUMBITS(1) [],

        /// Gathers all the after-masking CPU directed IRQ status bits from Channel 2.
        CH2 OFFSET(2) NUMBITS(1) [],

        /// Gathers all the after-masking CPU directed IRQ status bits from Channel 1.
        CH1 OFFSET(1) NUMBITS(1) [],

        /// Gathers all the after-masking CPU directed IRQ status bits from Channel 0.
        CH0 OFFSET(0) NUMBITS(1) []
    ],

    /// Bitfields of the `APBDMA_IRQ_STA_COP_0` register.
    pub APBDMA_IRQ_STA_COP_0 [
        /// Gathers all the after-masking COP directed IRQ status bits from Channel 31.
        CH31 OFFSET(31) NUMBITS(1) [],

        /// Gathers all the after-masking COP directed IRQ status bits from Channel 30.
        CH30 OFFSET(30) NUMBITS(1) [],

        /// Gathers all the after-masking COP directed IRQ status bits from Channel 29.
        CH29 OFFSET(29) NUMBITS(1) [],

        /// Gathers all the after-masking COP directed IRQ status bits from Channel 28.
        CH28 OFFSET(28) NUMBITS(1) [],

        /// Gathers all the after-masking COP directed IRQ status bits from Channel 27.
        CH27 OFFSET(27) NUMBITS(1) [],

        /// Gathers all the after-masking COP directed IRQ status bits from Channel 26.
        CH26 OFFSET(26) NUMBITS(1) [],

        /// Gathers all the after-masking COP directed IRQ status bits from Channel 25.
        CH25 OFFSET(25) NUMBITS(1) [],

        /// Gathers all the after-masking COP directed IRQ status bits from Channel 24.
        CH24 OFFSET(24) NUMBITS(1) [],

        /// Gathers all the after-masking COP directed IRQ status bits from Channel 23.
        CH23 OFFSET(23) NUMBITS(1) [],

        /// Gathers all the after-masking COP directed IRQ status bits from Channel 22.
        CH22 OFFSET(22) NUMBITS(1) [],

        /// Gathers all the after-masking COP directed IRQ status bits from Channel 21.
        CH21 OFFSET(21) NUMBITS(1) [],

        /// Gathers all the after-masking COP directed IRQ status bits from Channel 20.
        CH20 OFFSET(20) NUMBITS(1) [],

        /// Gathers all the after-masking COP directed IRQ status bits from Channel 19.
        CH19 OFFSET(19) NUMBITS(1) [],

        /// Gathers all the after-masking COP directed IRQ status bits from Channel 18.
        CH18 OFFSET(18) NUMBITS(1) [],

        /// Gathers all the after-masking COP directed IRQ status bits from Channel 17.
        CH17 OFFSET(17) NUMBITS(1) [],

        /// Gathers all the after-masking COP directed IRQ status bits from Channel 16.
        CH16 OFFSET(16) NUMBITS(1) [],

        /// Gathers all the after-masking COP directed IRQ status bits from Channel 15.
        CH15 OFFSET(15) NUMBITS(1) [],

        /// Gathers all the after-masking COP directed IRQ status bits from Channel 14.
        CH14 OFFSET(14) NUMBITS(1) [],

        /// Gathers all the after-masking COP directed IRQ status bits from Channel 13.
        CH13 OFFSET(13) NUMBITS(1) [],

        /// Gathers all the after-masking COP directed IRQ status bits from Channel 12.
        CH12 OFFSET(12) NUMBITS(1) [],

        /// Gathers all the after-masking COP directed IRQ status bits from Channel 11.
        CH11 OFFSET(11) NUMBITS(1) [],

        /// Gathers all the after-masking COP directed IRQ status bits from Channel 10.
        CH10 OFFSET(10) NUMBITS(1) [],

        /// Gathers all the after-masking COP directed IRQ status bits from Channel 9.
        CH9 OFFSET(9) NUMBITS(1) [],

        /// Gathers all the after-masking COP directed IRQ status bits from Channel 8.
        CH8 OFFSET(8) NUMBITS(1) [],

        /// Gathers all the after-masking COP directed IRQ status bits from Channel 7.
        CH7 OFFSET(7) NUMBITS(1) [],

        /// Gathers all the after-masking COP directed IRQ status bits from Channel 6.
        CH6 OFFSET(6) NUMBITS(1) [],

        /// Gathers all the after-masking COP directed IRQ status bits from Channel 5.
        CH5 OFFSET(5) NUMBITS(1) [],

        /// Gathers all the after-masking COP directed IRQ status bits from Channel 4.
        CH4 OFFSET(4) NUMBITS(1) [],

        /// Gathers all the after-masking COP directed IRQ status bits from Channel 3.
        CH3 OFFSET(3) NUMBITS(1) [],

        /// Gathers all the after-masking COP directed IRQ status bits from Channel 2.
        CH2 OFFSET(2) NUMBITS(1) [],

        /// Gathers all the after-masking COP directed IRQ status bits from Channel 1.
        CH1 OFFSET(1) NUMBITS(1) [],

        /// Gathers all the after-masking COP directed IRQ status bits from Channel 0.
        CH0 OFFSET(0) NUMBITS(1) []
    ],

    /// Bitfields of the `APBDMA_IRQ_MASK_0` register.
    pub APBDMA_IRQ_MASK_0 [
        /// Each bit allows the associated Channel 31 IRQ to propagate when `1`.
        CH31 OFFSET(31) NUMBITS(1) [],

        /// Each bit allows the associated Channel 30 IRQ to propagate when `1`.
        CH30 OFFSET(30) NUMBITS(1) [],

        /// Each bit allows the associated Channel 29 IRQ to propagate when `1`.
        CH29 OFFSET(29) NUMBITS(1) [],

        /// Each bit allows the associated Channel 28 IRQ to propagate when `1`.
        CH28 OFFSET(28) NUMBITS(1) [],

        /// Each bit allows the associated Channel 27 IRQ to propagate when `1`.
        CH27 OFFSET(27) NUMBITS(1) [],

        /// Each bit allows the associated Channel 26 IRQ to propagate when `1`.
        CH26 OFFSET(26) NUMBITS(1) [],

        /// Each bit allows the associated Channel 25 IRQ to propagate when `1`.
        CH25 OFFSET(25) NUMBITS(1) [],

        /// Each bit allows the associated Channel 24 IRQ to propagate when `1`.
        CH24 OFFSET(24) NUMBITS(1) [],

        /// Each bit allows the associated Channel 23 IRQ to propagate when `1`.
        CH23 OFFSET(23) NUMBITS(1) [],

        /// Each bit allows the associated Channel 22 IRQ to propagate when `1`.
        CH22 OFFSET(22) NUMBITS(1) [],

        /// Each bit allows the associated Channel 21 IRQ to propagate when `1`.
        CH21 OFFSET(21) NUMBITS(1) [],

        /// Each bit allows the associated Channel 20 IRQ to propagate when `1`.
        CH20 OFFSET(20) NUMBITS(1) [],

        /// Each bit allows the associated Channel 19 IRQ to propagate when `1`.
        CH19 OFFSET(19) NUMBITS(1) [],

        /// Each bit allows the associated Channel 18 IRQ to propagate when `1`.
        CH18 OFFSET(18) NUMBITS(1) [],

        /// Each bit allows the associated Channel 17 IRQ to propagate when `1`.
        CH17 OFFSET(17) NUMBITS(1) [],

        /// Each bit allows the associated Channel 16 IRQ to propagate when `1`.
        CH16 OFFSET(16) NUMBITS(1) [],

        /// Each bit allows the associated Channel 15 IRQ to propagate when `1`.
        CH15 OFFSET(15) NUMBITS(1) [],

        /// Each bit allows the associated Channel 14 IRQ to propagate when `1`.
        CH14 OFFSET(14) NUMBITS(1) [],

        /// Each bit allows the associated Channel 13 IRQ to propagate when `1`.
        CH13 OFFSET(13) NUMBITS(1) [],

        /// Each bit allows the associated Channel 12 IRQ to propagate when `1`.
        CH12 OFFSET(12) NUMBITS(1) [],

        /// Each bit allows the associated Channel 11 IRQ to propagate when `1`.
        CH11 OFFSET(11) NUMBITS(1) [],

        /// Each bit allows the associated Channel 10 IRQ to propagate when `1`.
        CH10 OFFSET(10) NUMBITS(1) [],

        /// Each bit allows the associated Channel 9 IRQ to propagate when `1`.
        CH9 OFFSET(9) NUMBITS(1) [],

        /// Each bit allows the associated Channel 8 IRQ to propagate when `1`.
        CH8 OFFSET(8) NUMBITS(1) [],

        /// Each bit allows the associated Channel 7 IRQ to propagate when `1`.
        CH7 OFFSET(7) NUMBITS(1) [],

        /// Each bit allows the associated Channel 6 IRQ to propagate when `1`.
        CH6 OFFSET(6) NUMBITS(1) [],

        /// Each bit allows the associated Channel 5 IRQ to propagate when `1`.
        CH5 OFFSET(5) NUMBITS(1) [],

        /// Each bit allows the associated Channel 4 IRQ to propagate when `1`.
        CH4 OFFSET(4) NUMBITS(1) [],

        /// Each bit allows the associated Channel 3 IRQ to propagate when `1`.
        CH3 OFFSET(3) NUMBITS(1) [],

        /// Each bit allows the associated Channel 2 IRQ to propagate when `1`.
        CH2 OFFSET(2) NUMBITS(1) [],

        /// Each bit allows the associated Channel 1 IRQ to propagate when `1`.
        CH1 OFFSET(1) NUMBITS(1) [],

        /// Each bit allows the associated Channel 0 IRQ to propagate when `1`.
        CH0 OFFSET(0) NUMBITS(1) []
    ],

    /// Bitfields of the `APBDMA_IRQ_MASK_SET_0` register.
    pub APBDMA_IRQ_MASK_SET_0 [
        /// Sets the Mask Register.
        CH31 OFFSET(31) NUMBITS(1) [],

        /// Sets the Mask Register.
        CH30 OFFSET(30) NUMBITS(1) [],

        /// Sets the Mask Register.
        CH29 OFFSET(29) NUMBITS(1) [],

        /// Sets the Mask Register.
        CH28 OFFSET(28) NUMBITS(1) [],

        /// Sets the Mask Register.
        CH27 OFFSET(27) NUMBITS(1) [],

        /// Sets the Mask Register.
        CH26 OFFSET(26) NUMBITS(1) [],

        /// Sets the Mask Register.
        CH25 OFFSET(25) NUMBITS(1) [],

        /// Sets the Mask Register.
        CH24 OFFSET(24) NUMBITS(1) [],

        /// Sets the Mask Register.
        CH23 OFFSET(23) NUMBITS(1) [],

        /// Sets the Mask Register.
        CH22 OFFSET(22) NUMBITS(1) [],

        /// Sets the Mask Register.
        CH21 OFFSET(21) NUMBITS(1) [],

        /// Sets the Mask Register.
        CH20 OFFSET(20) NUMBITS(1) [],

        /// Sets the Mask Register.
        CH19 OFFSET(19) NUMBITS(1) [],

        /// Sets the Mask Register.
        CH18 OFFSET(18) NUMBITS(1) [],

        /// Sets the Mask Register.
        CH17 OFFSET(17) NUMBITS(1) [],

        /// Sets the Mask Register.
        CH16 OFFSET(16) NUMBITS(1) [],

        /// Sets the Mask Register.
        CH15 OFFSET(15) NUMBITS(1) [],

        /// Sets the Mask Register.
        CH14 OFFSET(14) NUMBITS(1) [],

        /// Sets the Mask Register.
        CH13 OFFSET(13) NUMBITS(1) [],

        /// Sets the Mask Register.
        CH12 OFFSET(12) NUMBITS(1) [],

        /// Sets the Mask Register.
        CH11 OFFSET(11) NUMBITS(1) [],

        /// Sets the Mask Register.
        CH10 OFFSET(10) NUMBITS(1) [],

        /// Sets the Mask Register.
        CH9 OFFSET(9) NUMBITS(1) [],

        /// Sets the Mask Register.
        CH8 OFFSET(8) NUMBITS(1) [],

        /// Sets the Mask Register.
        CH7 OFFSET(7) NUMBITS(1) [],

        /// Sets the Mask Register.
        CH6 OFFSET(6) NUMBITS(1) [],

        /// Sets the Mask Register.
        CH5 OFFSET(5) NUMBITS(1) [],

        /// Sets the Mask Register.
        CH4 OFFSET(4) NUMBITS(1) [],

        /// Sets the Mask Register.
        CH3 OFFSET(3) NUMBITS(1) [],

        /// Sets the Mask Register.
        CH2 OFFSET(2) NUMBITS(1) [],

        /// Sets the Mask Register.
        CH1 OFFSET(1) NUMBITS(1) [],

        /// Sets the Mask Register.
        CH0 OFFSET(0) NUMBITS(1) []
    ],

    /// Bitfields of the `APBDMA_IRQ_MASK_CLR_0` register.
    pub APBDMA_IRQ_MASK_CLR_0 [
        /// Clears the Mask Register.
        CH31 OFFSET(31) NUMBITS(1) [],

        /// Clears the Mask Register.
        CH30 OFFSET(30) NUMBITS(1) [],

        /// Clears the Mask Register.
        CH29 OFFSET(29) NUMBITS(1) [],

        /// Clears the Mask Register.
        CH28 OFFSET(28) NUMBITS(1) [],

        /// Clears the Mask Register.
        CH27 OFFSET(27) NUMBITS(1) [],

        /// Clears the Mask Register.
        CH26 OFFSET(26) NUMBITS(1) [],

        /// Clears the Mask Register.
        CH25 OFFSET(25) NUMBITS(1) [],

        /// Clears the Mask Register.
        CH24 OFFSET(24) NUMBITS(1) [],

        /// Clears the Mask Register.
        CH23 OFFSET(23) NUMBITS(1) [],

        /// Clears the Mask Register.
        CH22 OFFSET(22) NUMBITS(1) [],

        /// Clears the Mask Register.
        CH21 OFFSET(21) NUMBITS(1) [],

        /// Clears the Mask Register.
        CH20 OFFSET(20) NUMBITS(1) [],

        /// Clears the Mask Register.
        CH19 OFFSET(19) NUMBITS(1) [],

        /// Clears the Mask Register.
        CH18 OFFSET(18) NUMBITS(1) [],

        /// Clears the Mask Register.
        CH17 OFFSET(17) NUMBITS(1) [],

        /// Clears the Mask Register.
        CH16 OFFSET(16) NUMBITS(1) [],

        /// Clears the Mask Register.
        CH15 OFFSET(15) NUMBITS(1) [],

        /// Clears the Mask Register.
        CH14 OFFSET(14) NUMBITS(1) [],

        /// Clears the Mask Register.
        CH13 OFFSET(13) NUMBITS(1) [],

        /// Clears the Mask Register.
        CH12 OFFSET(12) NUMBITS(1) [],

        /// Clears the Mask Register.
        CH11 OFFSET(11) NUMBITS(1) [],

        /// Clears the Mask Register.
        CH10 OFFSET(10) NUMBITS(1) [],

        /// Clears the Mask Register.
        CH9 OFFSET(9) NUMBITS(1) [],

        /// Clears the Mask Register.
        CH8 OFFSET(8) NUMBITS(1) [],

        /// Clears the Mask Register.
        CH7 OFFSET(7) NUMBITS(1) [],

        /// Clears the Mask Register.
        CH6 OFFSET(6) NUMBITS(1) [],

        /// Clears the Mask Register.
        CH5 OFFSET(5) NUMBITS(1) [],

        /// Clears the Mask Register.
        CH4 OFFSET(4) NUMBITS(1) [],

        /// Clears the Mask Register.
        CH3 OFFSET(3) NUMBITS(1) [],

        /// Clears the Mask Register.
        CH2 OFFSET(2) NUMBITS(1) [],

        /// Clears the Mask Register.
        CH1 OFFSET(1) NUMBITS(1) [],

        /// Clears the Mask Register.
        CH0 OFFSET(0) NUMBITS(1) []
    ],

    /// Bitfields of the `APBDMA_TRIG_REG_0` register.
    pub APBDMA_TRIG_REG_0 [
        /// Trigger select from Timer (Hardware initialized DMA request).
        TMR2 OFFSET(8) NUMBITS(1) [
            Inactive = 0,
            Active = 1
        ],

        /// Trigger select from Timer (Hardware initialized DMA request).
        TMR1 OFFSET(7) NUMBITS(1) [
            Inactive = 0,
            Active = 1
        ],

        /// XRQ.B (GPIO B) (Hardware initialized DMA request).
        XRQ_B OFFSET(6) NUMBITS(1) [
            Inactive = 0,
            Active = 1
        ],

        /// XRQ.A (GPIO A) (Hardware initialized DMA request).
        XRQ_A OFFSET(5) NUMBITS(1) [
            Inactive = 0,
            Active = 1
        ],

        /// Semaphore requests software-initialized DMA request.
        SMP_27 OFFSET(4) NUMBITS(1) [
            Inactive = 0,
            Active = 1
        ],

        /// Semaphore requests software-initialized DMA request.
        SMP_26 OFFSET(3) NUMBITS(1) [
            Inactive = 0,
            Active = 1
        ],

        /// Semaphore requests software-initialized DMA request.
        SMP_25 OFFSET(2) NUMBITS(1) [
            Inactive = 0,
            Active = 1
        ],

        /// Semaphore requests software-initialized DMA request.
        SMP_24 OFFSET(1) NUMBITS(1) [
            Inactive = 0,
            Active = 1
        ]
    ],

    /// Bitfields of the `APBDMA_CHANNEL_TRIG_REG_0` register.
    pub APBDMA_CHANNEL_TRIG_REG_0 [
        /// EOC-31 Initiated DMA Request after Transfer Completion.
        APB_31 OFFSET(31) NUMBITS(1) [],

        /// EOC-30 Initiated DMA Request after Transfer Completion.
        APB_30 OFFSET(30) NUMBITS(1) [],

        /// EOC-29 Initiated DMA Request after Transfer Completion.
        APB_29 OFFSET(29) NUMBITS(1) [],

        /// EOC-28 Initiated DMA Request after Transfer Completion.
        APB_28 OFFSET(28) NUMBITS(1) [],

        /// EOC-27 Initiated DMA Request after Transfer Completion.
        APB_27 OFFSET(27) NUMBITS(1) [],

        /// EOC-26 Initiated DMA Request after Transfer Completion.
        APB_26 OFFSET(26) NUMBITS(1) [],

        /// EOC-25 Initiated DMA Request after Transfer Completion.
        APB_25 OFFSET(25) NUMBITS(1) [],

        /// EOC-24 Initiated DMA Request after Transfer Completion.
        APB_24 OFFSET(24) NUMBITS(1) [],

        /// EOC-23 Initiated DMA Request after Transfer Completion.
        APB_23 OFFSET(23) NUMBITS(1) [],

        /// EOC-22 Initiated DMA Request after Transfer Completion.
        APB_22 OFFSET(22) NUMBITS(1) [],

        /// EOC-21 Initiated DMA Request after Transfer Completion.
        APB_21 OFFSET(21) NUMBITS(1) [],

        /// EOC-20 Initiated DMA Request after Transfer Completion.
        APB_20 OFFSET(20) NUMBITS(1) [],

        /// EOC-19 Initiated DMA Request after Transfer Completion.
        APB_19 OFFSET(19) NUMBITS(1) [],

        /// EOC-18 Initiated DMA Request after Transfer Completion.
        APB_18 OFFSET(18) NUMBITS(1) [],

        /// EOC-17 Initiated DMA Request after Transfer Completion.
        APB_17 OFFSET(17) NUMBITS(1) [],

        /// EOC-16 Initiated DMA Request after Transfer Completion.
        APB_16 OFFSET(16) NUMBITS(1) [],

        /// EOC-15 Initiated DMA Request after Transfer Completion.
        APB_15 OFFSET(15) NUMBITS(1) [],

        /// EOC-14 Initiated DMA Request after Transfer Completion.
        APB_14 OFFSET(14) NUMBITS(1) [],

        /// EOC-13 Initiated DMA Request after Transfer Completion.
        APB_13 OFFSET(13) NUMBITS(1) [],

        /// EOC-12 Initiated DMA Request after Transfer Completion.
        APB_12 OFFSET(12) NUMBITS(1) [],

        /// EOC-11 Initiated DMA Request after Transfer Completion.
        APB_11 OFFSET(11) NUMBITS(1) [],

        /// EOC-10 Initiated DMA Request after Transfer Completion.
        APB_10 OFFSET(10) NUMBITS(1) [],

        /// EOC-9 Initiated DMA Request after Transfer Completion.
        APB_9 OFFSET(9) NUMBITS(1) [],

        /// EOC-8 Initiated DMA Request after Transfer Completion.
        APB_8 OFFSET(8) NUMBITS(1) [],

        /// EOC-7 Initiated DMA Request after Transfer Completion.
        APB_7 OFFSET(7) NUMBITS(1) [],

        /// EOC-6 Initiated DMA Request after Transfer Completion.
        APB_6 OFFSET(6) NUMBITS(1) [],

        /// EOC-5 Initiated DMA Request after Transfer Completion.
        APB_5 OFFSET(5) NUMBITS(1) [],

        /// EOC-4 Initiated DMA Request after Transfer Completion.
        APB_4 OFFSET(4) NUMBITS(1) [],

        /// EOC-3 Initiated DMA Request after Transfer Completion.
        APB_3 OFFSET(3) NUMBITS(1) [],

        /// EOC-2 Initiated DMA Request after Transfer Completion.
        APB_2 OFFSET(2) NUMBITS(1) [],

        /// EOC-1 Initiated DMA Request after Transfer Completion.
        APB_1 OFFSET(1) NUMBITS(1) [],

        /// EOC-0 Initiated DMA Request after Transfer Completion.
        APB_0 OFFSET(0) NUMBITS(1) []
    ],

    /// Bitfields of the `APBDMA_DMA_STATUS_0` register.
    pub APBDMA_DMA_STATUS_0 [
        /// DMA Channel 31 Interrupt Status.
        ISE_EOC_31 OFFSET(31) NUMBITS(1) [
            Inactive = 0,
            Active = 1
        ],

        /// DMA Channel 30 Interrupt Status.
        ISE_EOC_30 OFFSET(30) NUMBITS(1) [
            Inactive = 0,
            Active = 1
        ],

        /// DMA Channel 29 Interrupt Status.
        ISE_EOC_29 OFFSET(29) NUMBITS(1) [
            Inactive = 0,
            Active = 1
        ],

        /// DMA Channel 28 Interrupt Status.
        ISE_EOC_28 OFFSET(28) NUMBITS(1) [
            Inactive = 0,
            Active = 1
        ],

        /// DMA Channel 27 Interrupt Status.
        ISE_EOC_27 OFFSET(27) NUMBITS(1) [
            Inactive = 0,
            Active = 1
        ],

        /// DMA Channel 26 Interrupt Status.
        ISE_EOC_26 OFFSET(26) NUMBITS(1) [
            Inactive = 0,
            Active = 1
        ],

        /// DMA Channel 25 Interrupt Status.
        ISE_EOC_25 OFFSET(25) NUMBITS(1) [
            Inactive = 0,
            Active = 1
        ],

        /// DMA Channel 24 Interrupt Status.
        ISE_EOC_24 OFFSET(24) NUMBITS(1) [
            Inactive = 0,
            Active = 1
        ],

        /// DMA Channel 23 Interrupt Status.
        ISE_EOC_23 OFFSET(23) NUMBITS(1) [
            Inactive = 0,
            Active = 1
        ],

        /// DMA Channel 22 Interrupt Status.
        ISE_EOC_22 OFFSET(22) NUMBITS(1) [
            Inactive = 0,
            Active = 1
        ],

        /// DMA Channel 21 Interrupt Status.
        ISE_EOC_21 OFFSET(21) NUMBITS(1) [
            Inactive = 0,
            Active = 1
        ],

        /// DMA Channel 20 Interrupt Status.
        ISE_EOC_20 OFFSET(20) NUMBITS(1) [
            Inactive = 0,
            Active = 1
        ],

        /// DMA Channel 19 Interrupt Status.
        ISE_EOC_19 OFFSET(19) NUMBITS(1) [
            Inactive = 0,
            Active = 1
        ],

        /// DMA Channel 18 Interrupt Status.
        ISE_EOC_18 OFFSET(18) NUMBITS(1) [
            Inactive = 0,
            Active = 1
        ],

        /// DMA Channel 17 Interrupt Status.
        ISE_EOC_17 OFFSET(17) NUMBITS(1) [
            Inactive = 0,
            Active = 1
        ],

        /// DMA Channel 16 Interrupt Status.
        ISE_EOC_16 OFFSET(16) NUMBITS(1) [
            Inactive = 0,
            Active = 1
        ],

        /// DMA Channel 15 Interrupt Status.
        ISE_EOC_15 OFFSET(15) NUMBITS(1) [
            Inactive = 0,
            Active = 1
        ],

        /// DMA Channel 14 Interrupt Status.
        ISE_EOC_14 OFFSET(14) NUMBITS(1) [
            Inactive = 0,
            Active = 1
        ],

        /// DMA Channel 13 Interrupt Status.
        ISE_EOC_13 OFFSET(13) NUMBITS(1) [
            Inactive = 0,
            Active = 1
        ],

        /// DMA Channel 12 Interrupt Status.
        ISE_EOC_12 OFFSET(12) NUMBITS(1) [
            Inactive = 0,
            Active = 1
        ],

        /// DMA Channel 11 Interrupt Status.
        ISE_EOC_11 OFFSET(11) NUMBITS(1) [
            Inactive = 0,
            Active = 1
        ],

        /// DMA Channel 10 Interrupt Status.
        ISE_EOC_10 OFFSET(10) NUMBITS(1) [
            Inactive = 0,
            Active = 1
        ],

        /// DMA Channel 9 Interrupt Status.
        ISE_EOC_9 OFFSET(9) NUMBITS(1) [
            Inactive = 0,
            Active = 1
        ],

        /// DMA Channel 8 Interrupt Status.
        ISE_EOC_8 OFFSET(8) NUMBITS(1) [
            Inactive = 0,
            Active = 1
        ],

        /// DMA Channel 7 Interrupt Status.
        ISE_EOC_7 OFFSET(7) NUMBITS(1) [
            Inactive = 0,
            Active = 1
        ],

        /// DMA Channel 6 Interrupt Status.
        ISE_EOC_6 OFFSET(6) NUMBITS(1) [
            Inactive = 0,
            Active = 1
        ],

        /// DMA Channel 5 Interrupt Status.
        ISE_EOC_5 OFFSET(5) NUMBITS(1) [
            Inactive = 0,
            Active = 1
        ],

        /// DMA Channel 4 Interrupt Status.
        ISE_EOC_4 OFFSET(4) NUMBITS(1) [
            Inactive = 0,
            Active = 1
        ],

        /// DMA Channel 3 Interrupt Status.
        ISE_EOC_3 OFFSET(3) NUMBITS(1) [
            Inactive = 0,
            Active = 1
        ],

        /// DMA Channel 2 Interrupt Status.
        ISE_EOC_2 OFFSET(2) NUMBITS(1) [
            Inactive = 0,
            Active = 1
        ],

        /// DMA Channel 1 Interrupt Status.
        ISE_EOC_1 OFFSET(1) NUMBITS(1) [
            Inactive = 0,
            Active = 1
        ],

        /// DMA Channel 0 Interrupt Status.
        ISE_EOC_0 OFFSET(0) NUMBITS(1) [
            Inactive = 0,
            Active = 1
        ]
    ],

    /// Bitfields of the `APBDMA_CHANNEL_EN_REG_0` register.
    pub APBDMA_CHANNEL_EN_REG_0 [
        /// Enables the Channel 31 Count.
        CH31_CNT_EN OFFSET(31) NUMBITS(1) [],

        /// Enables the Channel 30 Count.
        CH30_CNT_EN OFFSET(30) NUMBITS(1) [],

        /// Enables the Channel 29 Count.
        CH29_CNT_EN OFFSET(29) NUMBITS(1) [],

        /// Enables the Channel 28 Count.
        CH28_CNT_EN OFFSET(28) NUMBITS(1) [],

        /// Enables the Channel 27 Count.
        CH27_CNT_EN OFFSET(27) NUMBITS(1) [],

        /// Enables the Channel 26 Count.
        CH26_CNT_EN OFFSET(26) NUMBITS(1) [],

        /// Enables the Channel 25 Count.
        CH25_CNT_EN OFFSET(25) NUMBITS(1) [],

        /// Enables the Channel 24 Count.
        CH24_CNT_EN OFFSET(24) NUMBITS(1) [],

        /// Enables the Channel 23 Count.
        CH23_CNT_EN OFFSET(23) NUMBITS(1) [],

        /// Enables the Channel 22 Count.
        CH22_CNT_EN OFFSET(22) NUMBITS(1) [],

        /// Enables the Channel 21 Count.
        CH21_CNT_EN OFFSET(21) NUMBITS(1) [],

        /// Enables the Channel 20 Count.
        CH20_CNT_EN OFFSET(20) NUMBITS(1) [],

        /// Enables the Channel 19 Count.
        CH19_CNT_EN OFFSET(19) NUMBITS(1) [],

        /// Enables the Channel 18 Count.
        CH18_CNT_EN OFFSET(18) NUMBITS(1) [],

        /// Enables the Channel 17 Count.
        CH17_CNT_EN OFFSET(17) NUMBITS(1) [],

        /// Enables the Channel 16 Count.
        CH16_CNT_EN OFFSET(16) NUMBITS(1) [],

        /// Enables the Channel 15 Count.
        CH15_CNT_EN OFFSET(15) NUMBITS(1) [],

        /// Enables the Channel 14 Count.
        CH14_CNT_EN OFFSET(14) NUMBITS(1) [],

        /// Enables the Channel 13 Count.
        CH13_CNT_EN OFFSET(13) NUMBITS(1) [],

        /// Enables the Channel 12 Count.
        CH12_CNT_EN OFFSET(12) NUMBITS(1) [],

        /// Enables the Channel 11 Count.
        CH11_CNT_EN OFFSET(11) NUMBITS(1) [],

        /// Enables the Channel 10 Count.
        CH10_CNT_EN OFFSET(10) NUMBITS(1) [],

        /// Enables the Channel 9 Count.
        CH9_CNT_EN OFFSET(9) NUMBITS(1) [],

        /// Enables the Channel 8 Count.
        CH8_CNT_EN OFFSET(8) NUMBITS(1) [],

        /// Enables the Channel 7 Count.
        CH7_CNT_EN OFFSET(7) NUMBITS(1) [],

        /// Enables the Channel 6 Count.
        CH6_CNT_EN OFFSET(6) NUMBITS(1) [],

        /// Enables the Channel 5 Count.
        CH5_CNT_EN OFFSET(5) NUMBITS(1) [],

        /// Enables the Channel 4 Count.
        CH4_CNT_EN OFFSET(4) NUMBITS(1) [],

        /// Enables the Channel 3 Count.
        CH3_CNT_EN OFFSET(3) NUMBITS(1) [],

        /// Enables the Channel 2 Count.
        CH2_CNT_EN OFFSET(2) NUMBITS(1) [],

        /// Enables the Channel 1 Count.
        CH1_CNT_EN OFFSET(1) NUMBITS(1) [],

        /// Enables the Channel 0 Count.
        CH0_CNT_EN OFFSET(0) NUMBITS(1) []
    ],

    /// Bitfields of the `APBDMA_SECURITY_REG_0` register.
    pub APBDMA_SECURITY_REG_0 [
        /// Enables Secure Channel 31.
        CH_31_SECURITY_EN OFFSET(31) NUMBITS(1) [],

        /// Enables Secure Channel 30.
        CH_30_SECURITY_EN OFFSET(30) NUMBITS(1) [],

        /// Enables Secure Channel 29.
        CH_29_SECURITY_EN OFFSET(29) NUMBITS(1) [],

        /// Enables Secure Channel 28.
        CH_28_SECURITY_EN OFFSET(28) NUMBITS(1) [],

        /// Enables Secure Channel 27.
        CH_27_SECURITY_EN OFFSET(27) NUMBITS(1) [],

        /// Enables Secure Channel 26.
        CH_26_SECURITY_EN OFFSET(26) NUMBITS(1) [],

        /// Enables Secure Channel 25.
        CH_25_SECURITY_EN OFFSET(25) NUMBITS(1) [],

        /// Enables Secure Channel 24.
        CH_24_SECURITY_EN OFFSET(24) NUMBITS(1) [],

        /// Enables Secure Channel 23.
        CH_23_SECURITY_EN OFFSET(23) NUMBITS(1) [],

        /// Enables Secure Channel 22.
        CH_22_SECURITY_EN OFFSET(22) NUMBITS(1) [],

        /// Enables Secure Channel 21.
        CH_21_SECURITY_EN OFFSET(21) NUMBITS(1) [],

        /// Enables Secure Channel 20.
        CH_20_SECURITY_EN OFFSET(20) NUMBITS(1) [],

        /// Enables Secure Channel 19.
        CH_19_SECURITY_EN OFFSET(19) NUMBITS(1) [],

        /// Enables Secure Channel 18.
        CH_18_SECURITY_EN OFFSET(18) NUMBITS(1) [],

        /// Enables Secure Channel 17.
        CH_17_SECURITY_EN OFFSET(17) NUMBITS(1) [],

        /// Enables Secure Channel 16.
        CH_16_SECURITY_EN OFFSET(16) NUMBITS(1) [],

        /// Enables Secure Channel 15.
        CH_15_SECURITY_EN OFFSET(15) NUMBITS(1) [],

        /// Enables Secure Channel 14.
        CH_14_SECURITY_EN OFFSET(14) NUMBITS(1) [],

        /// Enables Secure Channel 13.
        CH_13_SECURITY_EN OFFSET(13) NUMBITS(1) [],

        /// Enables Secure Channel 12.
        CH_12_SECURITY_EN OFFSET(12) NUMBITS(1) [],

        /// Enables Secure Channel 11.
        CH_11_SECURITY_EN OFFSET(11) NUMBITS(1) [],

        /// Enables Secure Channel 10.
        CH_10_SECURITY_EN OFFSET(10) NUMBITS(1) [],

        /// Enables Secure Channel 9.
        CH_9_SECURITY_EN OFFSET(9) NUMBITS(1) [],

        /// Enables Secure Channel 8.
        CH_8_SECURITY_EN OFFSET(8) NUMBITS(1) [],

        /// Enables Secure Channel 7.
        CH_7_SECURITY_EN OFFSET(7) NUMBITS(1) [],

        /// Enables Secure Channel 6.
        CH_6_SECURITY_EN OFFSET(6) NUMBITS(1) [],

        /// Enables Secure Channel 5.
        CH_5_SECURITY_EN OFFSET(5) NUMBITS(1) [],

        /// Enables Secure Channel 4.
        CH_4_SECURITY_EN OFFSET(4) NUMBITS(1) [],

        /// Enables Secure Channel 3.
        CH_3_SECURITY_EN OFFSET(3) NUMBITS(1) [],

        /// Enables Secure Channel 2.
        CH_2_SECURITY_EN OFFSET(2) NUMBITS(1) [],

        /// Enables Secure Channel 1.
        CH_1_SECURITY_EN OFFSET(1) NUMBITS(1) [],

        /// Enables Secure Channel 0.
        CH_0_SECURITY_EN OFFSET(0) NUMBITS(1) []
    ],

    /// Bitfields of the `APBDMA_CHANNEL_SWID_0` register.
    pub APBDMA_CHANNEL_SWID_0 [
        /// SWID for Channel 31.
        CH_31_SWID OFFSET(31) NUMBITS(1) [],

        /// SWID for Channel 30.
        CH_30_SWID OFFSET(30) NUMBITS(1) [],

        /// SWID for Channel 29.
        CH_29_SWID OFFSET(29) NUMBITS(1) [],

        /// SWID for Channel 28.
        CH_28_SWID OFFSET(28) NUMBITS(1) [],

        /// SWID for Channel 27.
        CH_27_SWID OFFSET(27) NUMBITS(1) [],

        /// SWID for Channel 26.
        CH_26_SWID OFFSET(26) NUMBITS(1) [],

        /// SWID for Channel 25.
        CH_25_SWID OFFSET(25) NUMBITS(1) [],

        /// SWID for Channel 24.
        CH_24_SWID OFFSET(24) NUMBITS(1) [],

        /// SWID for Channel 23.
        CH_23_SWID OFFSET(23) NUMBITS(1) [],

        /// SWID for Channel 22.
        CH_22_SWID OFFSET(22) NUMBITS(1) [],

        /// SWID for Channel 21.
        CH_21_SWID OFFSET(21) NUMBITS(1) [],

        /// SWID for Channel 20.
        CH_20_SWID OFFSET(20) NUMBITS(1) [],

        /// SWID for Channel 19.
        CH_19_SWID OFFSET(19) NUMBITS(1) [],

        /// SWID for Channel 18.
        CH_18_SWID OFFSET(18) NUMBITS(1) [],

        /// SWID for Channel 17.
        CH_17_SWID OFFSET(17) NUMBITS(1) [],

        /// SWID for Channel 16.
        CH_16_SWID OFFSET(16) NUMBITS(1) [],

        /// SWID for Channel 15.
        CH_15_SWID OFFSET(15) NUMBITS(1) [],

        /// SWID for Channel 14.
        CH_14_SWID OFFSET(14) NUMBITS(1) [],

        /// SWID for Channel 13.
        CH_13_SWID OFFSET(13) NUMBITS(1) [],

        /// SWID for Channel 12.
        CH_12_SWID OFFSET(12) NUMBITS(1) [],

        /// SWID for Channel 11.
        CH_11_SWID OFFSET(11) NUMBITS(1) [],

        /// SWID for Channel 10.
        CH_10_SWID OFFSET(10) NUMBITS(1) [],

        /// SWID for Channel 9.
        CH_9_SWID OFFSET(9) NUMBITS(1) [],

        /// SWID for Channel 8.
        CH_8_SWID OFFSET(8) NUMBITS(1) [],

        /// SWID for Channel 7.
        CH_7_SWID OFFSET(7) NUMBITS(1) [],

        /// SWID for Channel 6.
        CH_6_SWID OFFSET(6) NUMBITS(1) [],

        /// SWID for Channel 5.
        CH_5_SWID OFFSET(5) NUMBITS(1) [],

        /// SWID for Channel 4.
        CH_4_SWID OFFSET(4) NUMBITS(1) [],

        /// SWID for Channel 3.
        CH_3_SWID OFFSET(3) NUMBITS(1) [],

        /// SWID for Channel 2.
        CH_2_SWID OFFSET(2) NUMBITS(1) [],

        /// SWID for Channel 1.
        CH_1_SWID OFFSET(1) NUMBITS(1) [],

        /// SWID for Channel 0.
        CH_0_SWID OFFSET(0) NUMBITS(1) []
    ],

    /// Bitfields of the `APBDMA_CHAN_WT_REG0_0` register.
    pub APBDMA_CHAN_WT_REG0_0 [
        /// Weight of Channel 31.
        WT_CH31 OFFSET(28) NUMBITS(4) [],

        /// Weight of Channel 30.
        WT_CH30 OFFSET(24) NUMBITS(4) [],

        /// Weight of Channel 29.
        WT_CH29 OFFSET(20) NUMBITS(4) [],

        /// Weight of Channel 28.
        WT_CH28 OFFSET(16) NUMBITS(4) [],

        /// Weight of Channel 27.
        WT_CH27 OFFSET(12) NUMBITS(4) [],

        /// Weight of Channel 26.
        WT_CH26 OFFSET(8) NUMBITS(4) [],

        /// Weight of Channel 25.
        WT_CH25 OFFSET(4) NUMBITS(4) [],

        /// Weight of Channel 24.
        WT_CH24 OFFSET(0) NUMBITS(4) []
    ],

    /// Bitfields of the `APBDMA_CHAN_WT_REG1_0` register.
    pub APBDMA_CHAN_WT_REG1_0 [
        /// Weight of Channel 23.
        WT_CH23 OFFSET(28) NUMBITS(4) [],

        /// Weight of Channel 22.
        WT_CH22 OFFSET(24) NUMBITS(4) [],

        /// Weight of Channel 21.
        WT_CH21 OFFSET(20) NUMBITS(4) [],

        /// Weight of Channel 20.
        WT_CH20 OFFSET(16) NUMBITS(4) [],

        /// Weight of Channel 19.
        WT_CH19 OFFSET(12) NUMBITS(4) [],

        /// Weight of Channel 18.
        WT_CH18 OFFSET(8) NUMBITS(4) [],

        /// Weight of Channel 17.
        WT_CH17 OFFSET(4) NUMBITS(4) [],

        /// Weight of Channel 16.
        WT_CH16 OFFSET(0) NUMBITS(4) []
    ],

    /// Bitfields of the `APBDMA_CHAN_WT_REG2_0` register.
    pub APBDMA_CHAN_WT_REG2_0 [
        /// Weight of Channel 15.
        WT_CH15 OFFSET(28) NUMBITS(4) [],

        /// Weight of Channel 14.
        WT_CH14 OFFSET(24) NUMBITS(4) [],

        /// Weight of Channel 13.
        WT_CH13 OFFSET(20) NUMBITS(4) [],

        /// Weight of Channel 12.
        WT_CH12 OFFSET(16) NUMBITS(4) [],

        /// Weight of Channel 11.
        WT_CH11 OFFSET(12) NUMBITS(4) [],

        /// Weight of Channel 10.
        WT_CH10 OFFSET(8) NUMBITS(4) [],

        /// Weight of Channel 9.
        WT_CH9 OFFSET(4) NUMBITS(4) [],

        /// Weight of Channel 8.
        WT_CH8 OFFSET(0) NUMBITS(4) []
    ],

    /// Bitfields of the `APBDMA_CHAN_WT_REG3_0` register.
    pub APBDMA_CHAN_WT_REG3_0 [
        /// Weight of Channel 7.
        WT_CH7 OFFSET(28) NUMBITS(4) [],

        /// Weight of Channel 6.
        WT_CH6 OFFSET(24) NUMBITS(4) [],

        /// Weight of Channel 5.
        WT_CH5 OFFSET(20) NUMBITS(4) [],

        /// Weight of Channel 4.
        WT_CH4 OFFSET(16) NUMBITS(4) [],

        /// Weight of Channel 3.
        WT_CH3 OFFSET(12) NUMBITS(4) [],

        /// Weight of Channel 2.
        WT_CH2 OFFSET(8) NUMBITS(4) [],

        /// Weight of Channel 1.
        WT_CH1 OFFSET(4) NUMBITS(4) [],

        /// Weight of Channel 0.
        WT_CH0 OFFSET(0) NUMBITS(4) []
    ],

    /// Bitfields of the `APBDMA_CHANNEL_SWID1_0` register.
    pub APBDMA_CHANNEL_SWID1_0 [
        /// SWID for Channel 31.
        CH_31_SWID_1 OFFSET(31) NUMBITS(1) [],

        /// SWID for Channel 30.
        CH_30_SWID_1 OFFSET(30) NUMBITS(1) [],

        /// SWID for Channel 29.
        CH_29_SWID_1 OFFSET(29) NUMBITS(1) [],

        /// SWID for Channel 28.
        CH_28_SWID_1 OFFSET(28) NUMBITS(1) [],

        /// SWID for Channel 27.
        CH_27_SWID_1 OFFSET(27) NUMBITS(1) [],

        /// SWID for Channel 26.
        CH_26_SWID_1 OFFSET(26) NUMBITS(1) [],

        /// SWID for Channel 25.
        CH_25_SWID_1 OFFSET(25) NUMBITS(1) [],

        /// SWID for Channel 24.
        CH_24_SWID_1 OFFSET(24) NUMBITS(1) [],

        /// SWID for Channel 23.
        CH_23_SWID_1 OFFSET(23) NUMBITS(1) [],

        /// SWID for Channel 22.
        CH_22_SWID_1 OFFSET(22) NUMBITS(1) [],

        /// SWID for Channel 21.
        CH_21_SWID_1 OFFSET(21) NUMBITS(1) [],

        /// SWID for Channel 20.
        CH_20_SWID_1 OFFSET(20) NUMBITS(1) [],

        /// SWID for Channel 19.
        CH_19_SWID_1 OFFSET(19) NUMBITS(1) [],

        /// SWID for Channel 18.
        CH_18_SWID_1 OFFSET(18) NUMBITS(1) [],

        /// SWID for Channel 17.
        CH_17_SWID_1 OFFSET(17) NUMBITS(1) [],

        /// SWID for Channel 16.
        CH_16_SWID_1 OFFSET(16) NUMBITS(1) [],

        /// SWID for Channel 15.
        CH_15_SWID_1 OFFSET(15) NUMBITS(1) [],

        /// SWID for Channel 14.
        CH_14_SWID_1 OFFSET(14) NUMBITS(1) [],

        /// SWID for Channel 13.
        CH_13_SWID_1 OFFSET(13) NUMBITS(1) [],

        /// SWID for Channel 12.
        CH_12_SWID_1 OFFSET(12) NUMBITS(1) [],

        /// SWID for Channel 11.
        CH_11_SWID_1 OFFSET(11) NUMBITS(1) [],

        /// SWID for Channel 10.
        CH_10_SWID_1 OFFSET(10) NUMBITS(1) [],

        /// SWID for Channel 9.
        CH_9_SWID_1 OFFSET(9) NUMBITS(1) [],

        /// SWID for Channel 8.
        CH_8_SWID_1 OFFSET(8) NUMBITS(1) [],

        /// SWID for Channel 7.
        CH_7_SWID_1 OFFSET(7) NUMBITS(1) [],

        /// SWID for Channel 6.
        CH_6_SWID_1 OFFSET(6) NUMBITS(1) [],

        /// SWID for Channel 5.
        CH_5_SWID_1 OFFSET(5) NUMBITS(1) [],

        /// SWID for Channel 4.
        CH_4_SWID_1 OFFSET(4) NUMBITS(1) [],

        /// SWID for Channel 3.
        CH_3_SWID_1 OFFSET(3) NUMBITS(1) [],

        /// SWID for Channel 2.
        CH_2_SWID_1 OFFSET(2) NUMBITS(1) [],

        /// SWID for Channel 1.
        CH_1_SWID_1 OFFSET(1) NUMBITS(1) [],

        /// SWID for Channel 0.
        CH_0_SWID_1 OFFSET(0) NUMBITS(1) []
    ]
}

register_structs! {
    /// Representation of the APB DMA registers.
    #[allow(non_snake_case)]
    pub Registers {
        (0x00 => pub APBDMA_COMMAND_0: ReadWrite<u32, APBDMA_COMMAND_0::Register>),
        (0x04 => pub APBDMA_STATUS_0: ReadOnly<u32, APBDMA_STATUS_0::Register>),
        (0x08 => _reserved0: [ReadWrite<u8>; 0x8]),
        (0x10 => pub APBDMA_CNTRL_REG_0: ReadWrite<u32, APBDMA_CNTRL_REG_0::Register>),
        (0x14 => pub APBDMA_IRQ_STA_CPU_0: ReadOnly<u32, APBDMA_IRQ_STA_CPU_0::Register>),
        (0x18 => pub APBDMA_IRQ_STA_COP_0: ReadOnly<u32, APBDMA_IRQ_STA_COP_0::Register>),
        (0x1C => pub APBDMA_IRQ_MASK_0: ReadOnly<u32, APBDMA_IRQ_MASK_0::Register>),
        (0x20 => pub APBDMA_IRQ_MASK_SET_0: WriteOnly<u32, APBDMA_IRQ_MASK_SET_0::Register>),
        (0x24 => pub APBDMA_IRQ_MASK_CLR_0: WriteOnly<u32, APBDMA_IRQ_MASK_CLR_0::Register>),
        (0x28 => pub APBDMA_TRIG_REG_0: ReadOnly<u32, APBDMA_TRIG_REG_0::Register>),
        (0x2C => pub APBDMA_CHANNEL_TRIG_REG_0: ReadOnly<u32, APBDMA_CHANNEL_TRIG_REG_0::Register>),
        (0x30 => pub APBDMA_DMA_STATUS_0: ReadOnly<u32, APBDMA_DMA_STATUS_0::Register>),
        (0x34 => pub APBDMA_CHANNEL_EN_REG_0: ReadWrite<u32, APBDMA_CHANNEL_EN_REG_0::Register>),
        (0x38 => pub APBDMA_SECURITY_REG_0: ReadWrite<u32, APBDMA_SECURITY_REG_0::Register>),
        (0x3C => pub APBDMA_CHANNEL_SWID_0: ReadWrite<u32, APBDMA_CHANNEL_SWID_0::Register>),
        (0x40 => _reserved1: [ReadWrite<u8>; 0x4]),
        (0x44 => pub APBDMA_CHAN_WT_REG0_0: ReadWrite<u32, APBDMA_CHAN_WT_REG0_0::Register>),
        (0x48 => pub APBDMA_CHAN_WT_REG1_0: ReadWrite<u32, APBDMA_CHAN_WT_REG1_0::Register>),
        (0x4C => pub APBDMA_CHAN_WT_REG2_0: ReadWrite<u32, APBDMA_CHAN_WT_REG2_0::Register>),
        (0x50 => pub APBDMA_CHAN_WT_REG3_0: ReadWrite<u32, APBDMA_CHAN_WT_REG3_0::Register>),
        (0x54 => pub APBDMA_CHANNEL_SWID1_0: ReadWrite<u32, APBDMA_CHANNEL_SWID1_0::Register>),
        (0x58 => @END),
    }
}

assert_eq_size!(Registers, [u8; 0x58]);
