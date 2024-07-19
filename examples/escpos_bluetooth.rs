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
