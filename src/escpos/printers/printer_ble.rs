#[allow(unused_imports)]
use btleplug::api::{bleuuid::uuid_from_u16, Characteristic, Manager as _, Peripheral, WriteType};
use std::error::Error;
use uuid::Uuid;

use super::PrinterTrait;

pub const THERMAL_PRINTER_SERVICE: &'static str = "000018f0-0000-1000-8000-00805f9b34fb";
pub const THERMAL_PRINTER_CHR_0: Uuid = uuid_from_u16(0x2af0);
pub const THERMAL_PRINTER_CHR_1: Uuid = uuid_from_u16(0x2af1);

pub struct PrinterESCPOSBLE {
    _peripheral: btleplug::platform::Peripheral,
}

impl PrinterESCPOSBLE {
    pub async fn new(peripheral: btleplug::platform::Peripheral) -> Result<Self, Box<dyn Error>> {
        #[cfg(feature = "debug")]
        log::debug!("eco_print::PrinterESCPOSBLE::new() -> start");

        if peripheral.is_connected().await? {
            #[cfg(feature = "debug")]
            log::debug!(
                "eco_print::PrinterESCPOSBLE::new() -> peripheral connected: {:?}",
                peripheral
            );
            peripheral.discover_services().await?;
            Ok(Self {
                _peripheral: peripheral,
            })
        } else {
            #[cfg(feature = "debug")]
            log::error!(
                "eco_print::PrinterESCPOSBLE::new() -> peripheral not connected: {:?}",
                peripheral
            );
            Err("Peripheral is not connected".into())
        }
    }

    fn get_print_chr(&self) -> Result<Characteristic, Box<dyn Error>> {
        #[cfg(feature = "debug")]
        log::debug!("eco_print::PrinterESCPOSBLE::get_print_chr() -> start");

        let chr = self
            ._peripheral
            .characteristics()
            .iter()
            .find(|chr| chr.uuid == THERMAL_PRINTER_CHR_1)
            .expect("Characteristicnot found")
            .clone();
        #[cfg(feature = "debug")]
        log::debug!(
            "eco_print::PrinterESCPOSBLE::get_print_chr() -> characteristic found: {:?}",
            chr
        );
        Ok(chr)
    }
}

impl PrinterTrait for PrinterESCPOSBLE {
    async fn print(&mut self, data: String) -> Result<(), Box<dyn Error>> {
        #[cfg(feature = "debug")]
        log::debug!(
            "eco_print::PrinterESCPOSBLE::print() -> start with data: {:?}",
            data
        );
        let chr = self.get_print_chr()?;
        self._peripheral
            .write(&chr, data.as_bytes(), WriteType::WithoutResponse)
            .await?;
        #[cfg(feature = "debug")]
        log::debug!("eco_print::PrinterESCPOSBLE::print() -> done");
        Ok(())
    }
}
