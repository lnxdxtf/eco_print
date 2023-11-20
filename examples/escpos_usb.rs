use eco_print::escpos::{
    commands::{
        basic::ESCPOSCommandsBasic,
        command::{ESCPOSCommand, ESCPOSCommandList},
    },
    printers::printer_usb::PrinterESCPOSUSB,
};
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let mut printer = PrinterESCPOSUSB::new(1155)?;
    
    let mut commands: ESCPOSCommandList = ESCPOSCommandList::new();
    commands.add_list(vec![
        ESCPOSCommand::Command(ESCPOSCommandsBasic::LineFeed),
        ESCPOSCommand::Command(ESCPOSCommandsBasic::AlignCenter),
        ESCPOSCommand::Text("X BLUETOOTH TEST CENTER X".to_string()),
        ESCPOSCommand::Command(ESCPOSCommandsBasic::LineFeed),
        ESCPOSCommand::Command(ESCPOSCommandsBasic::AlignLeft),
        ESCPOSCommand::Text("X BLUETOOTH TEST LEFT X".to_string()),
        ESCPOSCommand::Command(ESCPOSCommandsBasic::LineFeed),
        ESCPOSCommand::Command(ESCPOSCommandsBasic::AlignRight),
        ESCPOSCommand::Text("X BLUETOOTH TEST RIGHT X".to_string()),
        ESCPOSCommand::Command(ESCPOSCommandsBasic::LineFeed),
        ESCPOSCommand::Command(ESCPOSCommandsBasic::LineFeed),
        ESCPOSCommand::Command(ESCPOSCommandsBasic::LineFeed),
        ESCPOSCommand::Command(ESCPOSCommandsBasic::LineFeed),
    ]);

    printer.print_text(commands)?;

    Ok(())
}
