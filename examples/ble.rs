use std::str::FromStr;

use btleplug::{api::Peripheral as PeripheralTrait, platform::Peripheral};
use eco_print::{
    ble::{ESCPOSPrinterBLE, THERMAL_PRINTER_SERVICE},
    commands::command::{ESCPOSBuilder, ESCPOSBuilderTrait, ESCPOSCommand, ESCPOSDataBuilder},
    EcoPrintResult, FinderTrait, PrinterTrait,
};

#[tokio::main]
async fn main() -> EcoPrintResult<()> {
    env_logger::builder()
        .filter_level(log::LevelFilter::Trace)
        .init();

    let repeated_commands = std::iter::repeat(ESCPOSDataBuilder::Command(ESCPOSCommand::LineFeed))
        .take(5)
        .collect::<Vec<_>>();

    let mut commands: ESCPOSBuilder = ESCPOSBuilder::default();
    commands.add_commands(vec![
        ESCPOSDataBuilder::Command(ESCPOSCommand::LineFeed),
        ESCPOSDataBuilder::Command(ESCPOSCommand::LineFeed),
        ESCPOSDataBuilder::Command(ESCPOSCommand::LineFeed),
        ESCPOSDataBuilder::Text("Only a Test LOL ".into()),
    ]);

    // Add repeated commands
    commands.add_commands(repeated_commands);


    let mut printer = ESCPOSPrinterBLE::new()?;
    printer.start().await?;
    printer.scan().await?;
    let mut device_printer: Option<Peripheral> = None;

    while device_printer.is_none() {
        let devices = printer.get_devices().await;
        for d in devices {
            let prop = d.properties().await.unwrap().unwrap();
            if prop.services.contains(
                uuid::Uuid::from_str(THERMAL_PRINTER_SERVICE)
                    .unwrap()
                    .as_ref(),
            ) {
                device_printer = Some(d);
            }
        }
    }
    printer.connect(device_printer.clone().unwrap()).await?;
    printer.print(commands.to_escpos()).await?;
    Ok(())
}
