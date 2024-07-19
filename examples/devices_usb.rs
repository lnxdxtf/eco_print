use std::error::Error;

use eco_print::escpos::finder::usb::FinderUSB;

fn main() -> Result<(), Box<dyn Error>> {
    let devices = FinderUSB::devices()?;
    for device in devices.iter() {
        println!(
            "Device {:#?}",
            device.device_descriptor().unwrap().vendor_id()
        );
    }
    Ok(())
}
