/// Feature to control the 3d printer ender3v2
#[cfg(feature = "ender3v2")]
pub mod ender3v2;

/// Feature to control the all thermal printers based on escpos
#[cfg(feature = "escpos")]
pub mod escpos;