use std::time::Duration;

use eco_print::{
    ble::ESCPOSPrinterBLE,
    commands::command::{ESCPOSBuilder, ESCPOSBuilderTrait, ESCPOSCommand, ESCPOSDataBuilder},
    EcoPrintResult, FinderTrait,
};
use tokio::time::sleep;

#[tokio::main]
async fn main() -> EcoPrintResult<()> {
    pretty_env_logger::init();
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

    let mut printer = ESCPOSPrinterBLE::new()?;
    printer.start().await?;
    printer.scan().await?;
    sleep(Duration::from_secs(5)).await;
    let devices = printer.get_devices().await;
    println!("Devices: {:?}", devices);

    Ok(())
}
