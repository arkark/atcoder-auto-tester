use colored::*;

#[derive(Debug, Eq, PartialEq)]
pub enum MessageLevel {
    Error,
    Warning,
    Info,
    Command,
}

#[derive(Debug)]
pub struct Message {
    level: MessageLevel,
    text: String,
}

impl Message {
    pub fn new(level: MessageLevel, text: &str) -> Message {
        Message {
            level,
            text: String::from(text),
        }
    }

    pub fn show(&self) {
        let msg = match self.level {
            MessageLevel::Error => format!("{} {}", "[ERROR]".red().bold(), self.text.red()),
            MessageLevel::Warning => format!("{} {}", "[WARN]".yellow().bold(), self.text.yellow()),
            MessageLevel::Info => format!("[ {} ]", self.text.normal())
                .cyan()
                .bold()
                .reversed()
                .to_string(),
            MessageLevel::Command => format!("$ {}", self.text.normal()).cyan().to_string(),
        };
        println!("{}", msg);
    }
}

pub trait Messageable {
    fn error_message(&self) -> Message;
    fn warning_message(&self) -> Message;
    fn info_message(&self) -> Message;
    fn command_message(&self) -> Message;
}

impl Messageable for &str {
    fn error_message(&self) -> Message {
        Message::new(MessageLevel::Error, self)
    }

    fn warning_message(&self) -> Message {
        Message::new(MessageLevel::Warning, self)
    }

    fn info_message(&self) -> Message {
        Message::new(MessageLevel::Info, self)
    }

    fn command_message(&self) -> Message {
        Message::new(MessageLevel::Command, self)
    }
}

impl Messageable for String {
    fn error_message(&self) -> Message {
        Message::new(MessageLevel::Error, &self)
    }

    fn warning_message(&self) -> Message {
        Message::new(MessageLevel::Warning, &self)
    }

    fn info_message(&self) -> Message {
        Message::new(MessageLevel::Info, &self)
    }

    fn command_message(&self) -> Message {
        Message::new(MessageLevel::Command, &self)
    }
}
