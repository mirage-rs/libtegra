//! Driver for the Tegra X1 APB DMA Controller.
//!
//! See Chapter 21.2 in the Tegra X1 Technical Reference Manual
//! for details.

use ::core::cell::Cell;

pub use channel::*;
pub use self::core::*;

mod channel;
mod core;

/// Representation of the AMBA Peripheral Bus DMA Controller.
///
/// The controller manages 32 DMA [`Channel`]s, which are used
/// to transfer data over DMA. Various bus protocols support a
/// DMA mode for data transfers, which can be implemented by
/// interfacing with this device.
///
/// [`Channel`]: struct.Channel.html
#[derive(Debug)]
pub struct Controller {
    /// An array that tracks all 32 DMA [`Channel`]s the controller can access.
    ///
    /// [`Channel`]: struct.Channel.html
    channels: [Channel; 32],
}

impl Controller {
    /// Creates a new instance of the APB DMA Controller.
    ///
    /// NOTE: Please refrain from calling this method multiple times.
    /// It is advised to create a single, global instance
    /// of the [`Controller`] and stick to it.
    ///
    /// [`Controller`]: struct.Controller.html
    pub const fn new() -> Self {
        Controller {
            channels: [
                Channel::CH0,
                Channel::CH1,
                Channel::CH2,
                Channel::CH3,
                Channel::CH4,
                Channel::CH5,
                Channel::CH6,
                Channel::CH7,
                Channel::CH8,
                Channel::CH9,
                Channel::CH10,
                Channel::CH11,
                Channel::CH12,
                Channel::CH13,
                Channel::CH14,
                Channel::CH15,
                Channel::CH16,
                Channel::CH17,
                Channel::CH18,
                Channel::CH19,
                Channel::CH20,
                Channel::CH21,
                Channel::CH22,
                Channel::CH23,
                Channel::CH24,
                Channel::CH25,
                Channel::CH26,
                Channel::CH27,
                Channel::CH28,
                Channel::CH29,
                Channel::CH30,
                Channel::CH31,
            ],
        }
    }

    /// Tries to find a free [`Channel`].
    ///
    /// This method iterates over all 32 DMA [`Channel`]s
    /// and returns the first one that is not acquired.
    ///
    /// If no channels are free, this method returns `None`.
    ///
    /// [`Channel`]: struct.Channel.html
    fn find_free_channel(&self) -> Option<&Channel> {
        for channel in self.channels.iter() {
            if !channel.is_acquired() {
                return Some(channel);
            }
        }

        None
    }

    /// Reserves a [`Channel`] for use and passes its reference through the supplied closure.
    ///
    /// This method gives users the possibility to correctly acquire and release
    /// channels respectively, without having to worry about unwanted side effects,
    /// such as race conditions. If a specific channel is desired, it can be passed
    /// directly to this method, otherwise an unclaimed channel will be randomly
    /// picked.
    ///
    /// The supplied closure takes the [`Channel`] as argument and is expected to
    /// return a `Result`, which is forwarded to the direct return value of this
    /// method.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use libtegra::apb;
    ///
    /// /// The global instance of the APB DMA Controller.
    /// const APB_DMA_CONTROLLER: apb::dma::Controller = apb::dma::Controller::new();
    ///
    /// // Do something with DMA Channel 0...
    ///
    /// APB_DMA_CONTROLLER.with_channel(Some(apb::dma::Channel::CH0), |channel| {
    ///     // Within this context, we exclusively own Channel 0, which can
    ///     // be accessed through `channel` of type `&apb::dma::Channel`.
    ///
    ///     // Do something with channel...
    ///
    ///     Ok(())
    /// })
    /// .unwrap();
    ///
    /// // DMA Channel 0 was released, it can be re-used now.
    /// ```
    ///
    /// [`Channel`]: struct.Channel.html
    pub fn with_channel<C>(&self, channel: Option<Channel>, exec: C) -> Result<(), ()>
    where
        C: FnOnce(&Channel) -> Result<(), ()>,
    {
        let exec_channel = if let Some(ref ch) = channel {
            ch
        } else {
            self.find_free_channel()
                .expect("No free DMA Channel available!")
        };

        exec_channel.acquire();
        exec(exec_channel)?;
        exec_channel.release();

        Ok(())
    }
}

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
/// [`Controller`]: struct.Controller.html
#[derive(Clone, Debug, PartialEq, Eq)]
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
    /// [`Controller`]: struct.Controller.html
    pub(super) fn acquire(&self) {
        while self.claimed.get() {
            // Wait until the channel is free.
        }

        self.claimed.set(true);
    }

    /// Releases the current channel.
    ///
    /// NOTE: To be used by the [`Controller`] only.
    ///
    /// [`Controller`]: struct.Controller.html
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

    /// Prepares for data to be queried over the channel.
    ///
    /// This function doesn't actually trigger transfers, it only
    /// prepares the data output buffer and configures the channel. The
    /// actual transfer process can be initiated through
    /// [`Channel::start`] and terminated through [`Channel::finish`].
    /// For transfer status details, see [`Channel::is_busy`].
    ///
    /// NOTE: This method has a strong low-level approach and
    /// shouldn't be called directly.
    ///
    /// [`Channel::start`]: struct.Channel.html#method.start
    /// [`Channel::finish`]: struct.Channel.html#method.finish
    /// [`Channel::is_busy`]: struct.Channel.html#method.is_busy
    pub(super) fn query(
        &self,
        slave: u32,
        ahb_address: u32,
        apb_address: u32,
        size: u32,
    ) -> Result<(), ()> {
        let register_base = unsafe { &*self.registers };

        if size == 0 {
            return Err(());
        }

        // Program AHB and APB Starting addresses.
        register_base.APBDMACHAN_CHANNEL_AHB_PTR_0.set(ahb_address);
        register_base.APBDMACHAN_CHANNEL_APB_PTR_0.set(apb_address);

        // Set AHB 1 word burst, and no address wrapping.
        register_base.APBDMACHAN_CHANNEL_AHB_SEQ_0.modify(
            APBDMACHAN_CHANNEL_AHB_SEQ_0::AHB_BURST::DmaBurst1Words
            + APBDMACHAN_CHANNEL_AHB_SEQ_0::AHB_ADDR_WRAP::NoWrap
        );

        // Set APB bus width, and address wrap for each word.
        register_base.APBDMACHAN_CHANNEL_APB_SEQ_0.modify(
            APBDMACHAN_CHANNEL_APB_SEQ_0::APB_BUS_WIDTH::BusWidth32
            + APBDMACHAN_CHANNEL_APB_SEQ_0::APB_ADDR_WRAP::WrapOn1Words
        );

        // Set the amount of words to be transferred.
        register_base.APBDMACHAN_CHANNEL_WCOUNT_0.set((size - 1) as u32);

        // Set transfer mode to one block at a time (64kB),
        // set DMA direction for AHB to read,
        // and set up flow control.
        register_base.APBDMACHAN_CHANNEL_CSR_0.modify(
            APBDMACHAN_CHANNEL_CSR_0::ONCE::SingleBlock
            + APBDMACHAN_CHANNEL_CSR_0::DIR::AhbRead
            + APBDMACHAN_CHANNEL_CSR_0::REQ_SEL.val(slave)
            + APBDMACHAN_CHANNEL_CSR_0::FLOW::SET
        );

        Ok(())
    }

    /// Prepares data to be written over the channel.
    ///
    /// This method doesn't actually trigger transfers, it only
    /// loads in the data and configures the channel. The
    /// actual transfer process can be initiated through
    /// [`Channel::start`] and terminated through [`Channel::finish`].
    /// For transfer status details, see [`Channel::is_busy`].
    ///
    /// NOTE: This method has a strong low-level approach and
    /// shouldn't be called directly.
    ///
    /// [`Channel::start`]: struct.Channel.html#method.start
    /// [`Channel::finish`]: struct.Channel.html#method.finish
    /// [`Channel::is_busy`]: struct.Channel.html#method.is_busy
    pub(super) fn write(
        &self,
        slave: u32,
        ahb_address: u32,
        apb_address: u32,
        size: u32,
    ) -> Result<(), ()> {
        let register_base = unsafe { &*self.registers };

        if size == 0 {
            return Err(());
        }

        // Program AHB and APB Starting addresses.
        register_base.APBDMACHAN_CHANNEL_AHB_PTR_0.set(ahb_address);
        register_base.APBDMACHAN_CHANNEL_APB_PTR_0.set(apb_address);

        // Set AHB 1 word burst, and no address wrapping.
        register_base.APBDMACHAN_CHANNEL_AHB_SEQ_0.modify(
            APBDMACHAN_CHANNEL_AHB_SEQ_0::AHB_BURST::DmaBurst1Words
            + APBDMACHAN_CHANNEL_AHB_SEQ_0::AHB_ADDR_WRAP::NoWrap
        );

        // Set APB bus width, and address wrap for each word.
        register_base.APBDMACHAN_CHANNEL_APB_SEQ_0.modify(
            APBDMACHAN_CHANNEL_APB_SEQ_0::APB_BUS_WIDTH::BusWidth32
            + APBDMACHAN_CHANNEL_APB_SEQ_0::APB_ADDR_WRAP::WrapOn1Words
        );

        // Set the amount of words to be transferred.
        register_base.APBDMACHAN_CHANNEL_WCOUNT_0.set((size - 1) as u32);

        // Set transfer mode to one block at a time (64kB),
        // set DMA direction for AHB to read,
        // and set up flow control.
        register_base.APBDMACHAN_CHANNEL_CSR_0.modify(
            APBDMACHAN_CHANNEL_CSR_0::ONCE::SingleBlock
            + APBDMACHAN_CHANNEL_CSR_0::DIR::AhbWrite
            + APBDMACHAN_CHANNEL_CSR_0::REQ_SEL.val(slave)
            + APBDMACHAN_CHANNEL_CSR_0::FLOW::SET
        );

        Ok(())
    }
}
