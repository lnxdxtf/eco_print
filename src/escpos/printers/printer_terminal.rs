use std::error::Error;

use crate::escpos::commands::{command::ESCPOSCommandList, image::ESCPOSImage};

pub struct PrinterESCPOSBTerminal {}

impl PrinterESCPOSBTerminal {
    pub fn new() -> Self {
        Self {}
    }

    pub fn print_text(&self, command: ESCPOSCommandList) -> Result<(), Box<dyn Error>> {
        println!("{}", command.to_string());
        Ok(())
    }

    pub fn print_image(&self, image: ESCPOSImage) -> Result<(), Box<dyn Error>> {
        println!("{}", image.to_ascii_art());
        Ok(())
    }
}
