use std::error::Error;

use eco_print::escpos::commands::basic::ESCPOSCommandsBasic;
use eco_print::escpos::commands::command::{ESCPOSCommand, ESCPOSCommandList};
use eco_print::escpos::commands::image::ESCPOSImage;
use eco_print::escpos::printers::printer_bluetooth::PrinterESCPOSBluetooth;

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
        ESCPOSCommand::Text("X".to_string()),
        ESCPOSCommand::Command(ESCPOSCommandsBasic::AlignRight),
        ESCPOSCommand::Text("X".to_string()),
        ESCPOSCommand::Command(ESCPOSCommandsBasic::LineFeed),
        ESCPOSCommand::Text("X".to_string()),
        ESCPOSCommand::Command(ESCPOSCommandsBasic::AlignRight),
        ESCPOSCommand::Text("X".to_string()),
        ESCPOSCommand::Command(ESCPOSCommandsBasic::LineFeed),
        ESCPOSCommand::Text("X".to_string()),
        ESCPOSCommand::Command(ESCPOSCommandsBasic::LineFeed),
        ESCPOSCommand::Command(ESCPOSCommandsBasic::LineFeed),
    ]);

    let device_printer_bluetooth_name = "KP-1020".to_string();

    let mut printer: PrinterESCPOSBluetooth =
        PrinterESCPOSBluetooth::new(device_printer_bluetooth_name).await?;
    printer.scan_and_connect().await?;
    // printer.print_text(commands).await?;
    printer.print_image(img).await?;

    Ok(())
}
