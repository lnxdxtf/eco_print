use std::error::Error;

use bluest::{Adapter, AdvertisingDevice, Characteristic, Uuid};
use futures_lite::StreamExt;

use crate::escpos::commands::command::ESCPOSCommandList;
pub struct PrinterESCPOSBluetooth {
    adapter: Adapter,
    device_name: String,
    device: Option<AdvertisingDevice>,
    device_services: Option<Vec<Uuid>>,
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
            device_services: None,
            device: None,
            device_writer: None,
        })
    }

    pub async fn print_text(&mut self, command: ESCPOSCommandList) -> Result<(), Box<dyn Error>> {
        self.device_writer
            .clone()
            .unwrap()
            .write(command.to_string().as_bytes())
            .await?;
        Ok(())
    }

    pub async fn scan_and_connect(&mut self) -> Result<(), Box<dyn Error>> {
        let mut scan = self.adapter.scan(&[]).await?;
        while self.device.is_none() {
            if let Some(discovered_device) = scan.next().await {
                if discovered_device.device.name().as_deref().unwrap() == self.device_name {
                    self.device_services =
                        Some(self.get_device_services(&discovered_device).await?);
                    self.device_writer = Some(self.get_device_writer().await?);
                    self.device = Some(discovered_device);
                    println!("Connected to: {}", self.device_name);
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
        while let Some(service) = self
            .device
            .clone()
            .unwrap()
            .device
            .services()
            .await?
            .iter()
            .next()
        {
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
