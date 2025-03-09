use std::process::Command;

#[derive(Debug)]
pub struct AiderCommand {
    pub message: String,
}

impl AiderCommand {
    pub fn to_command(&self) -> Command {
        let mut cmd = Command::new("aider");
        
        cmd.arg("-m").arg(&self.message.trim());
        
        cmd
    }
}

