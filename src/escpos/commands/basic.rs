pub enum ESCPOSCommandsBasic {
    LineFeed,        // "\n"
    FontBold,        // "\x1B\x45\x01" (ESC E 1)
    FontNormal,      // "\x1B\x45\x00" (ESC E 0)
    Underline,       // "\x1B\x2D\x01" (ESC - 1)
    Cut,             // "\x1D\x56\x00" (GS V 0)
    AlignLeft,       // "\x1B\x61\x00" (ESC a 0)
    AlignCenter,     // "\x1B\x61\x01" (ESC a 1)
    AlignRight,      // "\x1B\x61\x02" (ESC a 2)
    FontA,           // "\x1B\x4D\x00" (ESC M 0)
    FontB,           // "\x1B\x4D\x01" (ESC M 1)
    EmphasizeOn,     // "\x1B\x45\x01" (ESC E 1)
    EmphasizeOff,    // "\x1B\x45\x00" (ESC E 0)
    DoubleHeightOn,  // "\x1B\x21\x10" (ESC ! 16)
    DoubleHeightOff, // "\x1B\x21\x00" (ESC ! 0)
    DoubleWidthOn,   // "\x1B\x21\x20" (ESC ! 32)
    DoubleWidthOff,  // "\x1B\x21\x00" (ESC ! 0)
    UpsideDownOn,    // "\x1B\x7B\x01" (ESC { 1)
    UpsideDownOff,   //  "\x1B\x7B\x00" (ESC { 0)
}

impl ESCPOSCommandsBasic {
    pub fn to_escpos(&self) -> &str {
        match *self {
            ESCPOSCommandsBasic::LineFeed => "\n",
            ESCPOSCommandsBasic::FontBold => "\x1B\x45\x01",
            ESCPOSCommandsBasic::FontNormal => "\x1B\x45\x00",
            ESCPOSCommandsBasic::Underline => "\x1B\x2D\x01",
            ESCPOSCommandsBasic::Cut => "\x1D\x56\x00",
            ESCPOSCommandsBasic::AlignLeft => "\x1B\x61\x00",
            ESCPOSCommandsBasic::AlignCenter => "\x1B\x61\x01",
            ESCPOSCommandsBasic::AlignRight => "\x1B\x61\x02",
            ESCPOSCommandsBasic::FontA => "\x1B\x4D\x00",
            ESCPOSCommandsBasic::FontB => "\x1B\x4D\x01",
            ESCPOSCommandsBasic::EmphasizeOn => "\x1B\x45\x01",
            ESCPOSCommandsBasic::EmphasizeOff => "\x1B\x45\x00",
            ESCPOSCommandsBasic::DoubleHeightOn => "\x1B\x21\x10",
            ESCPOSCommandsBasic::DoubleHeightOff => "\x1B\x21\x00",
            ESCPOSCommandsBasic::DoubleWidthOn => "\x1B\x21\x20",
            ESCPOSCommandsBasic::DoubleWidthOff => "\x1B\x21\x00",
            ESCPOSCommandsBasic::UpsideDownOn => "\x1B\x7B\x01",
            ESCPOSCommandsBasic::UpsideDownOff => "\x1B\x7B\x00",
        }
    }
}