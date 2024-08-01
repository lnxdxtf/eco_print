use std::error::Error;

use super::PrinterTrait;

pub struct PrinterESCPOSTerminal;

impl PrinterTrait for PrinterESCPOSTerminal {
    async fn print(&mut self, data: String) -> Result<(), Box<dyn Error>> {
        println!("{}", data);
        Ok(())
    }
}
