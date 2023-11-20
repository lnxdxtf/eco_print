use std::error::Error;

use bluest::{Adapter, AdvertisingDevice, Characteristic, Device, Uuid};
use futures_lite::StreamExt;

use crate::escpos::commands::command::ESCPOSCommandList;

pub struct PrinterESCPOSBluetooth {
    adapter: Adapter,
    device_name: String,
    device: Option<Device>,
    device_writer: Option<Characteristic>,
}

impl PrinterESCPOSBluetooth {
    pub async fn new(device_name: String) -> Result<Self, Box<dyn Error>> {
        let adapter = Adapter::default()
            .await
            .ok_or("Bluetooth adapter not found")?;
        adapter.wait_available().await?;
        Ok(Self {
            adapter,
            device_name,
            device: None,
            device_writer: None,
        })
    }

    pub async fn print_text(&mut self, command: ESCPOSCommandList) -> Result<(), Box<dyn Error>> {
        self.device_writer = Some(self.get_device_writer().await?);

        self.device_writer
            .clone()
            .unwrap()
            .write_without_response(command.to_string().as_bytes())
            .await?;
        Ok(())
    }

    pub async fn scan_and_connect(&mut self) -> Result<(), Box<dyn Error>> {
        let mut scan = self.adapter.scan(&[]).await?;
        while self.device.is_none() {
            if let Some(discovered_device) = scan.next().await {
                if discovered_device.device.name().as_deref().unwrap() == self.device_name {
                    let services = self.get_device_services(&discovered_device).await?;

                    let device_printer = self
                        .adapter
                        .discover_devices(&services)
                        .await?
                        .next()
                        .await
                        .ok_or("failed to discover device")
                        .unwrap()
                        .unwrap();

                    self.device = Some(device_printer);
                }
            }
        }
        Ok(())
    }

    async fn get_device_services(
        &self,
        device: &AdvertisingDevice,
    ) -> Result<Vec<Uuid>, Box<dyn Error>> {
        let services: Vec<Uuid> = device
            .device
            .services()
            .await?
            .iter()
            .map(|x: &bluest::Service| x.uuid())
            .collect();
        Ok(services)
    }

    async fn get_device_writer(&self) -> Result<Characteristic, Box<dyn Error>> {
        let mut characteristic_writer: Option<Characteristic> = None;
        for service in self.device.clone().unwrap().services().await?.iter() {
            let characteristics = service.characteristics().await?;
            for characteristic in characteristics.iter() {
                let props = characteristic.properties().await?;
                if props.write {
                    characteristic_writer = Some(characteristic.clone());
                    break;
                }
            }
        }
        Ok(characteristic_writer.unwrap())
    }
}
