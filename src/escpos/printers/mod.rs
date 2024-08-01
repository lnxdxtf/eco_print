#[cfg(feature = "ble")]
pub mod printer_ble;
#[cfg(feature = "usb")]
pub mod printer_usb;

pub mod printer_terminal;

#[allow(async_fn_in_trait)]
pub trait PrinterTrait {
    /// Print data to the printer
    /// the data parameter is a string that contains the data to be printed, which can be text, commands, etc.
    /// the data parameter contains the escpos commands already formatted.
    /// Use ESCPOSBuilder to generate the data parameter.
    /// e.g.:
    /// ```rust
    /// let mut builder = ESCPOSBuilder::new();
    /// builder.add_command(ESCPOSDataBuilder::Command(ESCPOSCommand::LineFeed));
    /// builder.add_command(ESCPOSDataBuilder::Command(ESCPOSCommand::FontBold));
    /// builder.add_command(ESCPOSDataBuilder::Text("Hello World".to_string()));
    /// let data = builder.to_escpos();
    /// //printer implements PrinterTrait
    /// let printer = Printer::new();
    /// printer.print(data);
    /// ```
    async fn print(&mut self, data: String) -> Result<(), Box<dyn std::error::Error>>;
}
