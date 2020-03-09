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
    pub fn is_idle(&self) -> bool {
        !self.is_busy()
    }

    /// Indicates whether the channel is currently claimed.
    pub fn is_acquired(&self) -> bool {
        self.claimed.get()
    }
}
