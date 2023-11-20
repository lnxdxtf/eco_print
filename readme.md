# eco_print

The eco_print library is a Rust-developed tool designed to facilitate the connection and execution of commands in thermal printers and, in the future, in the Ender 3 V2 3D printer. With this library, users can connect to thermal printers via both Bluetooth and USB, offering flexibility and convenience for various use cases.

Currently, eco_print only supports the Windows operating system, making it an ideal choice for developers and users in this environment. The library was created with the goal of integrating with a User Interface (UI), where users can connect their device and gain full control over the printer's functionalities.


### [FEATURE] ESCPOS
One of the main highlights of eco_print is its ability to intuitively and simply translate ESC/POS commands, making programming for thermal printers more accessible.

| Print    |  Method       | Status |
|----------|---------------|--------|
| Text     | .print_text() |   ✅   |
| BitImage | None          |   🚧   |
| BarCode  | None          |   🚧   |
| QRCode   | None          |   🚧   |


#### <a href="/examples/">Examples</a>

#### Usage
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

use eco_print::escpos::commands::basic::ESCPOSCommandsBasic;
use eco_print::escpos::commands::command::{ESCPOSCommand, ESCPOSCommandList};
use eco_print::escpos::printers::printer_bluetooth::PrinterESCPOSBluetooth;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let mut commands: ESCPOSCommandList = ESCPOSCommandList::new();
    commands.add_list(vec![
        ESCPOSCommand::Command(ESCPOSCommandsBasic::LineFeed),
        ESCPOSCommand::Command(ESCPOSCommandsBasic::AlignCenter),
        ESCPOSCommand::Text("X BLUETOOTH TEST CENTER X".to_string()),
        ESCPOSCommand::Command(ESCPOSCommandsBasic::LineFeed),
        ESCPOSCommand::Command(ESCPOSCommandsBasic::AlignLeft),
        ESCPOSCommand::Text("X BLUETOOTH TEST LEFT X".to_string()),
        ESCPOSCommand::Command(ESCPOSCommandsBasic::LineFeed),
        ESCPOSCommand::Command(ESCPOSCommandsBasic::AlignRight),
        ESCPOSCommand::Text("X BLUETOOTH TEST RIGHT X".to_string()),
        ESCPOSCommand::Command(ESCPOSCommandsBasic::LineFeed),
        ESCPOSCommand::Command(ESCPOSCommandsBasic::LineFeed),
        ESCPOSCommand::Command(ESCPOSCommandsBasic::LineFeed),
        ESCPOSCommand::Command(ESCPOSCommandsBasic::LineFeed),
    ]);

    let device_printer_bluetooth_name = "KP-1020".to_string();

    let mut printer: PrinterESCPOSBluetooth =
        PrinterESCPOSBluetooth::new(device_printer_bluetooth_name).await?;
    printer.scan_and_connect().await?;
    printer.print_text(commands).await?;

    Ok(())
}

```