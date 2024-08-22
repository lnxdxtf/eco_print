pub trait ESCPOSBuilderTrait {
    fn to_escpos(&self) -> String;

    // Not Required
    fn add_command(&mut self, _cmd: ESCPOSDataBuilder) {}
    
    // Not Required
    fn add_commands(&mut self, _cmds: Vec<ESCPOSDataBuilder>) {}
}

#[derive(Debug, Clone, Copy)]
pub enum ESCPOSCommand {
    LineFeed,
    FontBold,
    FontNormal,
    Underline,
    Cut,
    AlignLeft,
    AlignCenter,
    AlignRight,
    FontA,
    FontB,
    EmphasizeOn,
    EmphasizeOff,
    DoubleHeightOn,
    DoubleHeightOff,
    DoubleWidthOn,
    DoubleWidthOff,
    UpsideDownOn,
    UpsideDownOff,
}

impl ESCPOSBuilderTrait for ESCPOSCommand {
    fn to_escpos(&self) -> String {
        let escpos = match *self {
            ESCPOSCommand::LineFeed => "\n",
            ESCPOSCommand::FontBold => "\x1B\x45\x01",
            ESCPOSCommand::FontNormal => "\x1B\x45\x00",
            ESCPOSCommand::Underline => "\x1B\x2D\x01",
            ESCPOSCommand::Cut => "\x1D\x56\x00",
            ESCPOSCommand::AlignLeft => "\x1B\x61\x00",
            ESCPOSCommand::AlignCenter => "\x1B\x61\x01",
            ESCPOSCommand::AlignRight => "\x1B\x61\x02",
            ESCPOSCommand::FontA => "\x1B\x4D\x00",
            ESCPOSCommand::FontB => "\x1B\x4D\x01",
            ESCPOSCommand::EmphasizeOn => "\x1B\x45\x01",
            ESCPOSCommand::EmphasizeOff => "\x1B\x45\x00",
            ESCPOSCommand::DoubleHeightOn => "\x1B\x21\x10",
            ESCPOSCommand::DoubleHeightOff => "\x1B\x21\x00",
            ESCPOSCommand::DoubleWidthOn => "\x1B\x21\x20",
            ESCPOSCommand::DoubleWidthOff => "\x1B\x21\x00",
            ESCPOSCommand::UpsideDownOn => "\x1B\x7B\x01",
            ESCPOSCommand::UpsideDownOff => "\x1B\x7B\x00",
        };
        escpos.to_string()
    }
}

/// Use this enum to build ESCPOS commands
/// Command(ESCPOSCommand) - Add ESCPOSCommand
/// Text(String) - Add Text
/// Image(path/byte) - Add Image
/// QrCode(String) - Add QR Code
#[derive(Debug, Clone)]
pub enum ESCPOSDataBuilder {
    Command(ESCPOSCommand),
    Text(String),
    Image,
    QrCode,
}

/// Use this struct to build ESCPOS commands
/// That implements ESCPOSBuilderTrait, which is a trait that has methods to add commands and convert to ESCPOS string
/// e.g. 
/// ```rust
/// let mut builder = ESCPOSBuilder::new();
/// builder.add_command(ESCPOSDataBuilder::Command(ESCPOSCommand::LineFeed));
/// builder.add_command(ESCPOSDataBuilder::Command(ESCPOSCommand::FontBold));
/// builder.add_command(ESCPOSDataBuilder::Text("Hello World".to_string()));
/// let cmd_escpos = builder.to_escpos();
/// ```
#[derive(Default, Debug, Clone)]
pub struct ESCPOSBuilder(Vec<ESCPOSDataBuilder>);

impl ESCPOSBuilderTrait for ESCPOSBuilder {
    fn add_command(&mut self, _cmd: ESCPOSDataBuilder) {
        self.0.push(_cmd);
    }
    fn add_commands(&mut self, _cmds: Vec<ESCPOSDataBuilder>) {
        self.0.extend(_cmds);
    }
    fn to_escpos(&self) -> String {
        let mut escpos = String::new();
        for cmd in &self.0 {
            match cmd {
                ESCPOSDataBuilder::Command(cmd) => {
                    escpos.push_str(cmd.to_escpos().as_str());
                },
                ESCPOSDataBuilder::Text(str) => {
                    escpos.push_str(str);
                },
                ESCPOSDataBuilder::Image => todo!(),
                ESCPOSDataBuilder::QrCode => todo!(),
            }
        }
        escpos
    }
}