use std::{error::Error, str::FromStr, time::Duration};

use eco_print::escpos::{
    finder::ble::FinderBLE, printers::printer_ble::THERMAL_PRINTER_SERVICE,
};

use uuid::Uuid;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let adapter = FinderBLE::get_adapter().await?;
    let filter = vec![Uuid::from_str(THERMAL_PRINTER_SERVICE)?];

    let devices = FinderBLE::scan(&adapter, filter, Duration::from_secs(5)).await?;
    println!("{:#?}", devices);

    let device = FinderBLE::connect(devices[0].clone()).await?;

    Ok(())
}
