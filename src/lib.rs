#[allow(unused)]
#[macro_use]
extern crate log;
#[cfg(feature = "ble")]
pub mod ble;
#[cfg(feature = "usb")]
pub mod usb;

pub mod commands;

#[cfg(feature = "ble")]
pub use btleplug;

pub use tokio;
pub use uuid;

#[cfg(feature = "ble")]
#[cfg(target_os = "android")]
mod android_ble;

use serde::{ser::Serializer, Serialize};
pub type EcoPrintResult<T> = Result<T, EcoPrintError>;

#[derive(Debug, thiserror::Error)]
pub enum EcoPrintError {
    // USB
    #[error("USB feature not enabled on Cargo.toml")]
    USBFeatureNotEnabled,
    #[error("Error scanning for devices on USB: {0}")]
    USBScan(String),
    #[error("Error unknown usb: {0}")]
    USBUnknown(String),
    // BLE
    #[error("Bluetooth/BLE feature not enabled on Cargo.toml")]
    BLEFeatureNotEnabled,
    #[error("Adapter Bluetooth/BLE: {0}")]
    BLEAdapter(String),
    #[error("Bluetooth/BLE don't enabled")]
    BLENotEnabled,
    #[error("Error scanning for devices on Bluetooth/BLE: {0}")]
    BLEScan(String),
    #[error("Error connecting to device: {0}")]
    BLEConnect(String),
    #[error("Error on services peripheral: {0}")]
    BLEServices(String),
    #[error("Error unknown: {0}")]
    BLEUnknown(String),
    #[error("Error on peripheral: {0}")]
    BLEPeripheral(String),

    // Printer
    #[error("Error printer: {0}")]
    Printer(String),
    #[error("Error on printing: {0}")]
    Printing(String),
    #[error("Send to printer error: {0}")]
    Send(String),

    // Android JNI
    #[cfg(target_os = "android")]
    #[error("Error On Android: {0}")]
    Android(crate::android_ble::AndroidError),

    #[error("Error on runtime: {0}")]
    Runtime(String),
}

impl Serialize for EcoPrintError {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let msg = format!("{}", self.to_string());
        log::error!("{}", msg);
        serializer.serialize_str(&msg)
    }
}

unsafe impl Send for EcoPrintError {}
unsafe impl Sync for EcoPrintError {}

#[allow(async_fn_in_trait)]
pub trait PrinterTrait {
    /// Print data to the printer
    /// the data parameter is a string that contains the data to be printed, which can be text, commands, etc.
    /// the data parameter contains the escpos commands already formatted.
    /// Use ESCPOSBuilder to generate the data parameter.
    /// e.g.:
    /// ```rust
    /// let mut builder = ESCPOSBuilder::new();
    /// builder.add_command(ESCPOSDataBuilder::Command(ESCPOSCommand::LineFeed));
    /// builder.add_command(ESCPOSDataBuilder::Command(ESCPOSCommand::FontBold));
    /// builder.add_command(ESCPOSDataBuilder::Text("Hello World".to_string()));
    /// let data = builder.to_escpos();
    /// //printer implements PrinterTrait
    /// let printer = Printer::new();
    /// printer.print(data);
    /// ```
    async fn print(&mut self, data: String) -> crate::EcoPrintResult<()>;
}

#[allow(async_fn_in_trait)]
/// This trait is used to interact with the devices that can be found nearby.
/// Can be use for USB, Bluetooth, BLE, etc.
pub trait FinderTrait<Device> {
    /// Start the finder
    async fn start(&mut self) -> crate::EcoPrintResult<()> {
        Ok(())
    }

    /// Get the devices found or access the field that contains the devices found (self.devices)
    async fn get_devices(&self) -> Vec<Device>;

    /// Return the stream of devices found
    async fn scan(&mut self) -> crate::EcoPrintResult<()>;
    /// Connect to a device and save the connection on the struct
    async fn connect(&mut self, device: Device) -> crate::EcoPrintResult<()>;
    /// Disconnect from the device that is connected on the struct
    async fn disconnect(&mut self) -> crate::EcoPrintResult<()>;
}
