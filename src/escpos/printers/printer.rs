use super::{printer_bluetooth::PrinterESCPOSBluetooth, printer_usb::PrinterESCPOSUSB};

pub enum PrinterESCPOS {
    USB(PrinterESCPOSUSB),
    Bluetooth(PrinterESCPOSBluetooth),
}

impl PrinterESCPOS {
    pub async fn create_printer(
        &self,
        vendor_id: Option<u16>,
        device_name: Option<String>,
    ) -> Self {
        match self {
            PrinterESCPOS::USB(_) => {
                PrinterESCPOS::USB(PrinterESCPOSUSB::new(vendor_id.unwrap()).unwrap())
            }
            PrinterESCPOS::Bluetooth(_) => PrinterESCPOS::Bluetooth(
                PrinterESCPOSBluetooth::new(device_name.unwrap())
                    .await
                    .unwrap(),
            ),
        }
    }
}
