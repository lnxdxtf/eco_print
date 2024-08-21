use std::error::Error;

use qrcode::QrCode;

pub struct ESCPOSQRCode {
    pub qrcode: QrCode,
}

impl ESCPOSQRCode {
    pub fn new(data: String) -> Result<Self, Box<dyn Error>> {
        let qrcode = QrCode::new(data)?;
        Ok(Self { qrcode })
    }

    
}
