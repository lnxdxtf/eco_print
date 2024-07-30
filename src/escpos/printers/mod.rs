#[cfg(feature = "ble")]
pub mod printer_bluetooth;
#[cfg(feature = "usb")]
pub mod printer_usb;

pub mod printer_terminal;
