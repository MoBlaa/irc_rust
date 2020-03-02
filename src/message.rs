use crate::params::Params;
use crate::prefix::{Prefix, PrefixBuilder};
use crate::tags::Tags;

/// A simple irc message containing tags, prefix, command, parameters and a trailing parameter.
pub struct Message {
    raw: String
}

impl Message {
    /// Create a new Message from the given string. Expects the string to be in a valid irc format.
    pub fn new(value: &str) -> Message {
        Message {
            raw: value.to_string()
        }
    }

    /// Creates a message builder with the given command.
    pub fn builder() -> MessageBuilder {
        MessageBuilder {
            tags: Vec::new(),
            prefix: None,
            command: None,
            params: Vec::new(),
            trailing: None,
        }
    }

    ///
    pub fn tags(&self) -> Option<Tags> {
        if self.raw.starts_with('@') {
            self.raw.find(' ').and_then(|index| Some(Tags::new(&self.raw[1..index])))
        } else {
            None
        }
    }

    pub fn prefix(&self) -> Option<Prefix> {
        let offset = self.tags()
            // Set offset if tags exist
            .and_then(|tags| {
                // + '@' + ' '
                Some(tags.len() + 2)
            }).unwrap_or(0);
        match self.raw.chars().nth(offset) {
            Some(':') => {
                match self.raw[offset..].find(' ') {
                    Some(index) => Some(Prefix::new(&self.raw[offset + 1..offset + index])),
                    None => Some(Prefix::new(&self.raw[offset + 1..]))
                }
            }
            _ => None
        }
    }

    pub fn command(&self) -> &str {
        let without_tags = match self.raw.find(' ') {
            Some(start) => {
                if self.raw.starts_with("@") {
                    &self.raw[start + 1..]
                } else {
                    &self.raw
                }
            }
            None => &self.raw
        };
        let without_prefix = match without_tags.find(' ') {
            Some(start) => {
                if without_tags.starts_with(":") {
                    &without_tags[start + 1..]
                } else {
                    without_tags
                }
            }
            None => &self.raw
        };
        match without_prefix.find(' ') {
            Some(end) => &without_prefix[..end],
            None => without_prefix
        }
    }

    pub fn params(&self) -> Option<Params> {
        let command = self.command();
        let cmd_start = self.raw.find(command).unwrap();
        self.raw[cmd_start..].find(' ')
            .and_then(|param_start| Some(Params::new(&self.raw[cmd_start + param_start..])))
    }
}

impl ToString for Message {
    fn to_string(&self) -> String {
        self.raw.to_string()
    }
}

pub struct MessageBuilder {
    tags: Vec<String>,
    prefix: Option<Prefix>,
    command: Option<String>,
    params: Vec<String>,
    trailing: Option<String>,
}

impl MessageBuilder {
    pub fn command(mut self, cmd: &str) -> MessageBuilder {
        self.command = Some(cmd.to_string());
        self
    }

    pub fn tag(mut self, key: &str, value: &str) -> MessageBuilder {
        self.tags.push(format!("{}={}", key, value));
        self
    }

    pub fn prefix(mut self, prefix_builder: PrefixBuilder) -> MessageBuilder {
        self.prefix = Some(prefix_builder.build().unwrap());
        self
    }

    pub fn param(mut self, param: &str) -> MessageBuilder {
        self.params.push(param.to_string());
        self
    }

    pub fn trailing(mut self, trailing: &str) -> MessageBuilder {
        self.trailing = Some(trailing.to_string());
        self
    }

    pub fn build(self) -> Result<Message, &'static str> {
        let mut str = if !self.tags.is_empty() {
            format!("@{} ", self.tags.join(";"))
        } else {
            String::new()
        };
        if let Some(prefix) = self.prefix {
            str = format!("{}:{} ", str, prefix.to_string())
        }
        if let Some(command) = self.command {
            str = format!("{}{}", str, command);
        } else {
            return Err("message requires a command")
        }
        if !self.params.is_empty() {
            str = format!("{} {}", str, self.params.join(" "));
        }
        if let Some(trailing) = self.trailing {
            str = format!("{} :{}", str, trailing);
        }
        Ok(Message {
            raw: str
        })
    }
}