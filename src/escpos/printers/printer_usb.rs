use crate::escpos::{commands::command::ESCPOSCommandList, finder::usb::FinderUSB};
use rusb::{Device, DeviceHandle, GlobalContext};
use std::error::Error;
use std::time::Duration;

pub struct PrinterESCPOSUSB {
    _device: Device<GlobalContext>,
    device_handle: DeviceHandle<GlobalContext>,
    endpoint: u8,
}

impl PrinterESCPOSUSB {
    pub fn find_device(vendor_id: u16) -> Result<Device<GlobalContext>, Box<dyn Error>> {
        let devices = FinderUSB::devices()?;
        let device = devices
            .iter()
            .find_map(|device| {
                let descriptor = device.device_descriptor().ok()?;
                (descriptor.vendor_id() == vendor_id).then(|| device.clone())
            })
            .ok_or("Device not found")?;
        Ok(device)
    }

    pub fn open_device(device: &Device<GlobalContext>) -> Result<DeviceHandle<GlobalContext>, rusb::Error> {
        let mut device_handle = device.open()?;
        device_handle.claim_interface(0)?;
        Ok(device_handle)
    }

    pub fn new(vendor_id: u16) -> Result<Self, Box<dyn Error>> {
        let _device = Self::find_device(vendor_id)?;
        let device_handle = Self::open_device(&_device)?;
        let endpoint = _device.port_number().swap_bytes();
        Ok(Self {
            _device,
            device_handle,
            endpoint,
        })
    }

    pub fn print_text(&mut self, command: ESCPOSCommandList) -> Result<usize, rusb::Error> {
        self.device_handle.write_bulk(
            self.endpoint,
            command.to_string().as_bytes(),
            Duration::from_secs(1),
        )
    }
}