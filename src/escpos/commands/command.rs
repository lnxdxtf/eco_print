use super::{basic::ESCPOSCommandsBasic, image::ESCPOSImage};

pub enum ESCPOSCommand {
    Command(ESCPOSCommandsBasic),
    Text(String),
}

pub struct ESCPOSCommandList(Vec<ESCPOSCommand>);

impl Default for ESCPOSCommandList {
    fn default() -> Self {
        Self::new()
    }
}

impl ESCPOSCommandList {
    pub fn new() -> Self {
        ESCPOSCommandList(Vec::new())
    }

    pub fn add_command(&mut self, command: ESCPOSCommand) {
        self.0.push(command);
    }

    pub fn add_list(&mut self, commands: Vec<ESCPOSCommand>) {
        for command in commands {
            self.add_command(command);
        }
    }
}

impl ToString for ESCPOSCommandList {
    fn to_string(&self) -> String {
        let mut command_string = String::new();
        for command in &self.0 {
            match command {
                ESCPOSCommand::Command(cmd) => {
                    command_string.push_str(cmd.to_escpos());
                }
                ESCPOSCommand::Text(text) => {
                    command_string.push_str(text);
                }
            }
        }
        command_string
    }
}
