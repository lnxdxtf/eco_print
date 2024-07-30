use crate::escpos::commands::command::ESCPOSCommandList;
use btleplug::{
    api::{bleuuid::uuid_from_u16, Central, Characteristic, Manager as _, Peripheral, WriteType},
    platform::{Adapter, Manager},
};
use std::{error::Error, str::FromStr};
use uuid::Uuid;

pub const THERMAL_PRINTER_SERVICE: &'static str = "000018f0-0000-1000-8000-00805f9b34fb";
pub const THERMAL_PRINTER_CHR_0: Uuid = uuid_from_u16(0x2af0);
pub const THERMAL_PRINTER_CHR_1: Uuid = uuid_from_u16(0x2af1);

pub struct PrinterESCPOSBluetooth {
    _peripheral: btleplug::platform::Peripheral,
}

impl PrinterESCPOSBluetooth {
    pub async fn new(peripheral: btleplug::platform::Peripheral) -> Result<Self, Box<dyn Error>> {
        if peripheral.is_connected().await? {
            peripheral.discover_services().await?;
            Ok(Self {
                _peripheral: peripheral,
            })
        } else {
            Err("Peripheral is not connected".into())
        }
    }

    fn get_print_chr(&self) -> Result<Characteristic, Box<dyn Error>> {
        let chr = self
            ._peripheral
            .characteristics()
            .iter()
            .find(|chr| chr.uuid == THERMAL_PRINTER_CHR_1)
            .expect("Characteristicnot found")
            .clone();
        Ok(chr)
    }

    pub async fn print_text(&mut self, command: ESCPOSCommandList) -> Result<(), Box<dyn Error>> {
        let chr = self.get_print_chr()?;
        let data = command.to_string();
        self._peripheral
            .write(&chr, data.as_bytes(), WriteType::WithoutResponse)
            .await?;
        Ok(())
    }

    #[cfg(feature = "img")]
    pub async fn print_image(
        &mut self,
        image: crate::escpos::commands::command::image::ESCPOSImage,
    ) -> Result<(), Box<dyn Error>> {
        use image::EncodableLayout;

        let chr = self.get_print_chr()?;
        let data = image.to_escpos();
        println!("{:?}", data);
        self._peripheral
            .write(&chr, data.as_bytes(), WriteType::WithoutResponse)
            .await?;

        Ok(())
    }
}
