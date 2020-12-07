//! Driver for the Tegra X1 AMBA Peripheral Bus Controller.
//!
//! See Chapter 21 in the Tegra X1 Technical Reference Manual for details.
//!
//! # Miscellaneous
//!
//! A number of system control registers that are grouped together in the
//! aptly named miscellaneous section are exposed here.
//!
//! There's no higher-level wrapper from Rust code other than MMIO abstractions
//! because these registers vastly differ in their purpose and should rather
//! be programmed manually be experienced users where needed.
//!
//! # DMA
//!
//! The APB DMA Controller is placed between the AHB Bus and the APB Bus
//! and is master on both buses.
//!
//! The APB DMA Controller is used for block data transfers where the source
//! may be DRAM or IRAM and the destination some device placed on the APB Bus,
//! or vice versa. DMA transfers are done without any processor intervention
//! other than register writes needed to program the parameters for a particular
//! transfer and accesses needed to handle any interrupts.
//!
//! ## Functionality
//!
//! The controller manages and exposes 32 channels where each channel can transfer
//! specified portions of data from an AHB address space to an APB address space.
//! It follows a simple round robin arbitration scheme, starting with channel 0.
//! Alternatively, a user may select a channel manually and let the driver detect
//! whether that channel is currently in use when another DMA transfer should be
//! executed.
//!
//! Each channel can have independent burst transfer sizes programmed to one word,
//! four words or eight words.

pub mod dma;
pub mod misc;
