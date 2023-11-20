use rusb::{devices, DeviceList, GlobalContext};

pub struct FinderUSB;

impl FinderUSB {
    pub fn devices() -> Result<DeviceList<GlobalContext>, rusb::Error> {
        devices()
    }
}
