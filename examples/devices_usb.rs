use std::error::Error;

use eco_print::escpos::finder::usb::FinderUSB;

fn main() -> Result<(), Box<dyn Error>> {
    let devices = FinderUSB::devices()?;
    for device in devices.iter() {
        let descriptor = device.device_descriptor()?;
        println!("Device | Vendor_id {:?}", descriptor.vendor_id());
    }
    Ok(())
}
