//! Driver for the Tegra X1 APB DMA Controller.
//!
//! See Chapter 21.2 in the Tegra X1 Technical Reference Manual
//! for details.

use ::core::cell::Cell;

pub use channel::*;
pub use self::core::*;

mod channel;
mod core;

/// Representation of an APB DMA Channel.
///
/// Channels are used for data transfers over DMA by the DMA
/// [`Controller`] and need to be claimed and released upon
/// usage.
///
/// NOTE: Instances of this struct should never be created manually.
/// Refer to the public constants this struct holds, which represent
/// the channels 0 through 31.
///
/// [`Controller`]:
pub struct Channel {
    /// A pointer to the [`Register`] base of this channel.
    ///
    /// [`Register`]: struct.ChannelRegisters.html
    registers: *const ChannelRegisters,
    /// A cell which holds the state of whether this channel is claimed.
    ///
    /// NOTE: When initializing a new instance of this structure, always
    /// initialize this with `false` as modifying this value should never
    /// be done manually. It is exposed through the [`Channel::is_acquired`]
    /// method.
    ///
    /// [`Channel::is_acquired`]: struct.Channel.html#method.is_acquired
    claimed: Cell<bool>,
}

// Definitions of known Channels.

impl Channel {
    /// Representation of the DMA Channel 0.
    pub const CH0: Self = Channel {
        registers: CHANNEL_0,
        claimed: Cell::new(false),
    };

    /// Representation of the DMA Channel 1.
    pub const CH1: Self = Channel {
        registers: CHANNEL_1,
        claimed: Cell::new(false),
    };

    /// Representation of the DMA Channel 2.
    pub const CH2: Self = Channel {
        registers: CHANNEL_2,
        claimed: Cell::new(false),
    };

    /// Representation of the DMA Channel 3.
    pub const CH3: Self = Channel {
        registers: CHANNEL_3,
        claimed: Cell::new(false),
    };

    /// Representation of the DMA Channel 4.
    pub const CH4: Self = Channel {
        registers: CHANNEL_4,
        claimed: Cell::new(false),
    };

    /// Representation of the DMA Channel 5.
    pub const CH5: Self = Channel {
        registers: CHANNEL_5,
        claimed: Cell::new(false),
    };

    /// Representation of the DMA Channel 6.
    pub const CH6: Self = Channel {
        registers: CHANNEL_6,
        claimed: Cell::new(false),
    };

    /// Representation of the DMA Channel 7.
    pub const CH7: Self = Channel {
        registers: CHANNEL_7,
        claimed: Cell::new(false),
    };

    /// Representation of the DMA Channel 8.
    pub const CH8: Self = Channel {
        registers: CHANNEL_8,
        claimed: Cell::new(false),
    };

    /// Representation of the DMA Channel 9.
    pub const CH9: Self = Channel {
        registers: CHANNEL_9,
        claimed: Cell::new(false),
    };

    /// Representation of the DMA Channel 10.
    pub const CH10: Self = Channel {
        registers: CHANNEL_10,
        claimed: Cell::new(false),
    };

    /// Representation of the DMA Channel 11.
    pub const CH11: Self = Channel {
        registers: CHANNEL_11,
        claimed: Cell::new(false),
    };

    /// Representation of the DMA Channel 12.
    pub const CH12: Self = Channel {
        registers: CHANNEL_12,
        claimed: Cell::new(false),
    };

    /// Representation of the DMA Channel 13.
    pub const CH13: Self = Channel {
        registers: CHANNEL_13,
        claimed: Cell::new(false),
    };

    /// Representation of the DMA Channel 14.
    pub const CH14: Self = Channel {
        registers: CHANNEL_14,
        claimed: Cell::new(false),
    };

    /// Representation of the DMA Channel 15.
    pub const CH15: Self = Channel {
        registers: CHANNEL_15,
        claimed: Cell::new(false),
    };

    /// Representation of the DMA Channel 16.
    pub const CH16: Self = Channel {
        registers: CHANNEL_16,
        claimed: Cell::new(false),
    };

    /// Representation of the DMA Channel 17.
    pub const CH17: Self = Channel {
        registers: CHANNEL_17,
        claimed: Cell::new(false),
    };

    /// Representation of the DMA Channel 18.
    pub const CH18: Self = Channel {
        registers: CHANNEL_18,
        claimed: Cell::new(false),
    };

    /// Representation of the DMA Channel 19.
    pub const CH19: Self = Channel {
        registers: CHANNEL_19,
        claimed: Cell::new(false),
    };

    /// Representation of the DMA Channel 20.
    pub const CH20: Self = Channel {
        registers: CHANNEL_20,
        claimed: Cell::new(false),
    };

    /// Representation of the DMA Channel 21.
    pub const CH21: Self = Channel {
        registers: CHANNEL_21,
        claimed: Cell::new(false),
    };

    /// Representation of the DMA Channel 22.
    pub const CH22: Self = Channel {
        registers: CHANNEL_22,
        claimed: Cell::new(false),
    };

    /// Representation of the DMA Channel 23.
    pub const CH23: Self = Channel {
        registers: CHANNEL_23,
        claimed: Cell::new(false),
    };

    /// Representation of the DMA Channel 24.
    pub const CH24: Self = Channel {
        registers: CHANNEL_24,
        claimed: Cell::new(false),
    };

    /// Representation of the DMA Channel 25.
    pub const CH25: Self = Channel {
        registers: CHANNEL_25,
        claimed: Cell::new(false),
    };

    /// Representation of the DMA Channel 26.
    pub const CH26: Self = Channel {
        registers: CHANNEL_26,
        claimed: Cell::new(false),
    };

    /// Representation of the DMA Channel 27.
    pub const CH27: Self = Channel {
        registers: CHANNEL_27,
        claimed: Cell::new(false),
    };

    /// Representation of the DMA Channel 28.
    pub const CH28: Self = Channel {
        registers: CHANNEL_28,
        claimed: Cell::new(false),
    };

    /// Representation of the DMA Channel 29.
    pub const CH29: Self = Channel {
        registers: CHANNEL_29,
        claimed: Cell::new(false),
    };

    /// Representation of the DMA Channel 30.
    pub const CH30: Self = Channel {
        registers: CHANNEL_30,
        claimed: Cell::new(false),
    };

    /// Representation of the DMA Channel 31.
    pub const CH31: Self = Channel {
        registers: CHANNEL_31,
        claimed: Cell::new(false),
    };
}

impl Channel {
    /// Acquires the current channel.
    ///
    /// NOTE: To be used by the [`Controller`] only.
    ///
    /// [`Controller`]:
    pub(super) fn acquire(&self) {
        if self.claimed.get() {
            panic!("Channel is already acquired!");
        }

        self.claimed.set(true);
    }

    /// Releases the current channel.
    ///
    /// NOTE: To be used by the [`Controller`] only.
    ///
    /// [`Controller`]:
    pub(super) fn release(&self) {
        self.claimed.set(false);
    }

    /// Starts DMA transfers for the current channel.
    pub fn start(&self) {
        let register_base = unsafe { &*self.registers };

        register_base.APBDMACHAN_CHANNEL_CSR_0.modify(APBDMACHAN_CHANNEL_CSR_0::ENB::SET);
    }

    /// Finishes DMA transfers for the current channel.
    pub fn finish(&self) {
        let register_base = unsafe { &*self.registers };

        register_base.APBDMACHAN_CHANNEL_CSR_0.modify(
            APBDMACHAN_CHANNEL_CSR_0::HOLD::SET
            + APBDMACHAN_CHANNEL_CSR_0::ENB::CLEAR
        );
    }

    /// Indicates whether the channel is currently busy doing transfers.
    ///
    /// NOTE: Channels need to be acquired before being able to transfer
    /// data. If this function returns `true`, [`Channel::is_acquired`]
    /// is guaranteed to return `true` as well.
    ///
    /// [`Channel::is_acquired`]: struct.Channel.html#method.is_acquired
    pub fn is_busy(&self) -> bool {
        let register_base = unsafe { &*self.registers };

        if register_base.APBDMACHAN_CHANNEL_STA_0
            .is_set(APBDMACHAN_CHANNEL_STA_0::DMA_ACTIVITY)
        {
            true
        } else {
            false
        }
    }

    /// Indicates whether the channel is in idle state and ready for transfers.
    ///
    /// This is the counterpart to [`Channel::is_busy`].
    ///
    /// [`Channel::is_busy`]: struct.Channel.html#method.is_busy
    pub fn is_idle(&self) -> bool {
        !self.is_busy()
    }

    /// Indicates whether the channel is currently claimed.
    ///
    /// NOTE: Channels need to be acquired before being able to
    /// transfer data. Even though this function returns `true`,
    /// that doesn't necessarily mean that data is actually being
    /// processed at the moment. See [`Channel::is_ready`] for
    /// further details.
    ///
    /// [`Channel::is_ready`]: struct.Channel.html#method.is_ready
    pub fn is_acquired(&self) -> bool {
        self.claimed.get()
    }
}
