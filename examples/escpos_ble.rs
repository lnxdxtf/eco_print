use std::error::Error;
use std::str::FromStr;
use std::time::Duration;

use eco_print::escpos::commands::command::{
    ESCPOSBuilder, ESCPOSBuilderTrait, ESCPOSCommand, ESCPOSDataBuilder,
};
use eco_print::escpos::finder::ble::FinderBLE;
use eco_print::escpos::printers::{
    printer_ble::{PrinterESCPOSBLE, THERMAL_PRINTER_SERVICE},
    PrinterTrait,
};
use uuid::Uuid;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let mut commands: ESCPOSBuilder = ESCPOSBuilder::default();
    commands.add_commands(vec![
        ESCPOSDataBuilder::Command(ESCPOSCommand::LineFeed),
        ESCPOSDataBuilder::Command(ESCPOSCommand::LineFeed),
        ESCPOSDataBuilder::Command(ESCPOSCommand::LineFeed),
        ESCPOSDataBuilder::Text("SO UM TESTE PAEEE HEHEEH".into()),
        ESCPOSDataBuilder::Command(ESCPOSCommand::LineFeed),
        ESCPOSDataBuilder::Command(ESCPOSCommand::LineFeed),
        ESCPOSDataBuilder::Command(ESCPOSCommand::LineFeed),
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
    let mut printer = PrinterESCPOSBLE::new(peripheral).await?;

    println!("PRINTING ...");
    printer.print(commands.to_escpos()).await?;

    Ok(())
}
