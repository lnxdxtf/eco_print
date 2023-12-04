use eco_print::escpos::{
    commands::{
        basic::ESCPOSCommandsBasic,
        command::{ESCPOSCommand, ESCPOSCommandList},
        image::ESCPOSImage,
    },
    printers::printer_terminal::PrinterESCPOSBTerminal,
};
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let printer = PrinterESCPOSBTerminal::new();
    let img_path = "./assets/imgs/img_01.jpeg";
    let img = ESCPOSImage::new(img_path, 384).unwrap();

    let mut commands: ESCPOSCommandList = ESCPOSCommandList::new();
    commands.add_list(vec![
        ESCPOSCommand::Command(ESCPOSCommandsBasic::LineFeed),
        ESCPOSCommand::Command(ESCPOSCommandsBasic::AlignCenter),
        ESCPOSCommand::Text("X".to_string()),
        ESCPOSCommand::Command(ESCPOSCommandsBasic::LineFeed),
        ESCPOSCommand::Command(ESCPOSCommandsBasic::AlignLeft),
        ESCPOSCommand::Text("X".to_string()),
        ESCPOSCommand::Command(ESCPOSCommandsBasic::LineFeed),
        ESCPOSCommand::Command(ESCPOSCommandsBasic::AlignRight),
        ESCPOSCommand::Text("X".to_string()),
        ESCPOSCommand::Command(ESCPOSCommandsBasic::LineFeed),
        ESCPOSCommand::Command(ESCPOSCommandsBasic::LineFeed),
    ]);

    printer.print_text(commands)?;
    printer.print_image(img)?;

    Ok(())
}
