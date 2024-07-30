# eco_print

The eco_print library is a Rust-developed tool designed to facilitate the
connection and execution of commands in thermal printers. With this library, users can connect to thermal
printers via both Bluetooth and USB, offering flexibility and convenience for
various use cases.

Currently, eco_print only supports the Windows operating system, making it an
ideal choice for developers and users in this environment. The library was
created with the goal of integrating with a User Interface (UI), where users can
connect their device and gain full control over the printer's functionalities.

# ESCPOS

One of the main highlights of eco_print is its ability to intuitively and simply
translate ESC/POS commands, making programming for thermal printers more
accessible.

| Print    | Method        | Status |
| -------- | ------------- | ------ |
| Text     | .print_text() | ✅     |
| BitImage | None          | 🚧     |
| BarCode  | None          | 🚧     |
| QRCode   | None          | 🚧     |

#### <a href="/examples/">Examples</a>


## Status
| Feature             | Status |
| --------            | --|
| Android             |🚧|
| IOS                 |❌|
| Image/QrCode/BitMap |🚧|
| USB-Windows         |✅|
| BLE-Windows         |✅|
| USB-MacOS           |❌|
| BLE-MacOS           |❌|
| USB-Linux           |❌|
| BLE-Linux           |❌|

NOTE: Mobile(IOS/ANDROID) only supports BLE



## Usage

- USB

```rust
    use eco_print::escpos::{
    commands::{
        basic::ESCPOSCommandsBasic,
        command::{ESCPOSCommand, ESCPOSCommandList},
    },
    printers::printer_usb::PrinterESCPOSUSB,
};
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    // to show all devices and vendor_id from each usb_device
    let devices = FinderUSB::devices()?;
    for device in devices.iter() {
        let descriptor = device.device_descriptor()?;
        println!("Device | Vendor_id {:?}", descriptor.vendor_id());
    }
    // You need know the vendor_id of your device
    let vendod_id_printer = 1155;
    let mut printer = PrinterESCPOSUSB::new(1155)?;

    let mut commands: ESCPOSCommandList = ESCPOSCommandList::new();
    commands.add_list(vec![
        ESCPOSCommand::Command(ESCPOSCommandsBasic::LineFeed),
        ESCPOSCommand::Command(ESCPOSCommandsBasic::AlignCenter),
        ESCPOSCommand::Text("X".to_string()),
        ESCPOSCommand::Command(ESCPOSCommandsBasic::LineFeed),
        ESCPOSCommand::Command(ESCPOSCommandsBasic::AlignLeft),
        ESCPOSCommand::Text("X".to_string()),
        ESCPOSCommand::Command(ESCPOSCommandsBasic::LineFeed),
        ESCPOSCommand::Command(ESCPOSCommandsBasic::AlignRight),
        ESCPOSCommand::Text("X".to_string()),
        ESCPOSCommand::Command(ESCPOSCommandsBasic::LineFeed),
        ESCPOSCommand::Command(ESCPOSCommandsBasic::LineFeed),
        ESCPOSCommand::Command(ESCPOSCommandsBasic::LineFeed),
        ESCPOSCommand::Command(ESCPOSCommandsBasic::LineFeed),
    ]);

    printer.print_text(commands)?;

    Ok(())
}
```

- Bluetooth

```rust
use std::error::Error;
use std::str::FromStr;
use std::time::Duration;

use eco_print::escpos::commands::basic::ESCPOSCommandsBasic;
use eco_print::escpos::commands::command::{ESCPOSCommand, ESCPOSCommandList};
use eco_print::escpos::commands::image::ESCPOSImage;
use eco_print::escpos::finder::ble::FinderBLE;
use eco_print::escpos::printers::printer_bluetooth::{
    PrinterESCPOSBluetooth, THERMAL_PRINTER_SERVICE,
};
use uuid::Uuid;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let img_path = "assets/imgs/square.png";
    let img = ESCPOSImage::new(img_path, 384).unwrap();

    let mut commands: ESCPOSCommandList = ESCPOSCommandList::new();
    commands.add_list(vec![
        ESCPOSCommand::Command(ESCPOSCommandsBasic::LineFeed),
        ESCPOSCommand::Command(ESCPOSCommandsBasic::AlignRight),
        ESCPOSCommand::Text("X".to_string()),
        ESCPOSCommand::Command(ESCPOSCommandsBasic::LineFeed),
        ESCPOSCommand::Command(ESCPOSCommandsBasic::AlignCenter),
        ESCPOSCommand::Text("X".to_string()),
        ESCPOSCommand::Command(ESCPOSCommandsBasic::LineFeed),
        ESCPOSCommand::Command(ESCPOSCommandsBasic::AlignLeft),
        ESCPOSCommand::Text("X".to_string()),
        ESCPOSCommand::Command(ESCPOSCommandsBasic::LineFeed),
        ESCPOSCommand::Command(ESCPOSCommandsBasic::LineFeed),
        ESCPOSCommand::Command(ESCPOSCommandsBasic::LineFeed),
        ESCPOSCommand::Command(ESCPOSCommandsBasic::LineFeed),
        ESCPOSCommand::Command(ESCPOSCommandsBasic::LineFeed),
    ]);
    let adapter = FinderBLE::get_adapter().await?;
    let filter = vec![Uuid::from_str(THERMAL_PRINTER_SERVICE)?];
    let devices = FinderBLE::scan(&adapter, filter, Duration::from_secs(5)).await?;
    if devices.is_empty() {
        println!("No devices found");
        return Ok(());
    }
    println!("{:#?}", devices);
    let peripheral = FinderBLE::connect(devices[0].clone()).await?;
    let mut printer = PrinterESCPOSBluetooth::new(peripheral).await?;

    println!("PRINTING ...");
    printer.print_text(commands).await?;

    Ok(())
}
```
