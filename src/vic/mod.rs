//! Driver for interfacing with the Tegra X1 Video Image Compositor (VIC).
//!
//! # Description
//!
//! The Video Image Compositor (VIC) unit implements video post-processing functions
//! needed by a video playback application to produce the final image for the player
//! window. The VIC can also perform scaling, composition, and rotation when no 3D
//! rendering is involved.
//!
//! The compositor implements much of the DirectX Video Acceleration 2.0 Enhanced
//! Video Processor specification, including de-interlacing, scaling, color conversion,
//! proc-amp, and compositing for up to 8 input surfaces. It supports advanced
//! features like gamma/de-gamma programming, color correct processing and pixel
//! decompression.

mod registers;

pub use registers::*;
