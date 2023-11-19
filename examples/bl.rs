use std::error::Error;
use std::time::Duration;

use eco_print::escpos::commands::basic::ESCPOSCommandsBasic;
use eco_print::escpos::commands::command::{ESCPOSCommandList, ESCPOSCommand};
use eco_print::escpos::printers::printer_bluetooth::PrinterESCPOSBluetooth;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // let adapter = Adapter::default()
    //     .await
    //     .ok_or("Bluetooth adapter not found")?;
    // adapter.wait_available().await?;

    // let mut scan = adapter.scan(&[]).await?;
    // let mut found_device = false;
    // let mut device: Option<AdvertisingDevice> = None; // Initialize device as None
    // while !found_device {
    //     if let Some(discovered_device) = scan.next().await {
    //         if discovered_device.device.name().as_deref().unwrap() == "KP-1020" {
    //             found_device = true;
    //             device = Some(discovered_device);
    //         }
    //     }
    // }

    // if let Some(device) = device {
    //     let cmd = "teste do pai x5\n\n\n".as_bytes();

    //     let services: Vec<Uuid> = device
    //         .device
    //         .services()
    //         .await?
    //         .iter()
    //         .map(|x| x.uuid())
    //         .collect();

    //     let device_printer = adapter
    //         .discover_devices(&services)
    //         .await?
    //         .next()
    //         .await
    //         .ok_or("failed to discover device")
    //         .unwrap()
    //         .unwrap();

    //     println!(
    //         "{:?} pair: {:?} connected: {:?}",
    //         device_printer.name(),
    //         device_printer.is_paired().await,
    //         device_printer.is_connected().await
    //     );
    //     let mut characteristic_writer: Option<bluest::Characteristic> = None;

    //     for service in device_printer.services().await?.iter() {
    //         let characteristics = service.characteristics().await?;
    //         for characteristic in characteristics.iter() {
    //             let props = characteristic.properties().await?;
    //             if props.write {
    //                 characteristic_writer = Some(characteristic.clone());
    //                 break;
    //             }
    //         }
    //     }
    //     if let Some(writer) = characteristic_writer {
    //         writer.write(cmd).await?;
    //     }
    // }
    let mut commands: ESCPOSCommandList = ESCPOSCommandList::new();
    commands.add_list(vec![
        ESCPOSCommand::Command(ESCPOSCommandsBasic::LineFeed),
        ESCPOSCommand::Command(ESCPOSCommandsBasic::AlignCenter),
        ESCPOSCommand::Text("X BLUETOOTH".to_string()),
        ESCPOSCommand::Command(ESCPOSCommandsBasic::LineFeed),
        ESCPOSCommand::Command(ESCPOSCommandsBasic::AlignLeft),
        ESCPOSCommand::Text("X BLUETOOTH DO PAI".to_string()),
        ESCPOSCommand::Command(ESCPOSCommandsBasic::LineFeed),
        ESCPOSCommand::Command(ESCPOSCommandsBasic::AlignRight),
        ESCPOSCommand::Text("X OLOKO".to_string()),
        ESCPOSCommand::Command(ESCPOSCommandsBasic::LineFeed),
        ESCPOSCommand::Command(ESCPOSCommandsBasic::LineFeed),
        ESCPOSCommand::Command(ESCPOSCommandsBasic::LineFeed),
        ESCPOSCommand::Command(ESCPOSCommandsBasic::LineFeed),
    ]);
    let mut printer: PrinterESCPOSBluetooth = PrinterESCPOSBluetooth::new("KP-1020".to_string()).await?;
    printer.scan_and_connect().await?;
    printer.print_text(commands).await?;
    Ok(())
}
