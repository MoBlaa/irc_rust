use std::collections::HashMap;

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

    /// Creates a message builder as alternative to building an irc string before creating the message.
    pub fn builder() -> MessageBuilder<'static> {
        MessageBuilder {
            tags: HashMap::new(),
            prefix: None,
            command: None,
            params: Vec::new(),
            trailing: None,
        }
    }

    /// Returns tags if any are present.
    pub fn tags(&self) -> Option<Tags> {
        if self.raw.starts_with('@') {
            self.raw.find(' ').and_then(|index| Some(Tags::new(&self.raw[1..index])))
        } else {
            None
        }
    }

    /// Returns the Prefix if present.
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

    /// Returns the command the message represents.
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

    /// Returns the params if any are present.
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

/// A MessageBuilder for a simpler generation of a message instead of building an string first.
pub struct MessageBuilder<'a> {
    tags: HashMap<&'a str, &'a str>,
    prefix: Option<Prefix>,
    command: Option<&'a str>,
    params: Vec<&'a str>,
    trailing: Option<&'a str>,
}

impl<'a> MessageBuilder<'a> {
    /// Set the command.
    pub fn command(mut self, cmd: &'a str) -> MessageBuilder<'a> {
        self.command = Some(cmd);
        self
    }

    /// Add a tag and ignore if the tag has already been present. Duplicate keys are possible.
    pub fn tag(mut self, key: &'a str, value: &'a str) -> MessageBuilder<'a> {
        self.tags.insert(key, value);
        self
    }

    ///
    pub fn prefix(mut self, prefix_builder: PrefixBuilder) -> MessageBuilder<'a> {
        self.prefix = Some(prefix_builder.build().unwrap());
        self
    }

    pub fn param(mut self, param: &'a str) -> MessageBuilder<'a> {
        self.params.push(param);
        self
    }

    pub fn trailing(mut self, trailing: &'a str) -> MessageBuilder<'a> {
        self.trailing = Some(trailing);
        self
    }

    pub fn build(self) -> Result<Message, &'static str> {
        let mut str = String::new();
        if !self.tags.is_empty() {
            str.push('@');
            for (key, val) in self.tags {
                str.push_str(key);
                str.push_str("=");
                str.push_str(val);
                str.push_str(";")
            }
            str.pop();
            str.push(' ');
        }
        if let Some(prefix) = self.prefix {
            str = format!("{}:{} ", str, prefix.to_string())
        }
        if let Some(command) = self.command {
            str = format!("{}{}", str, command);
        } else {
            return Err("message requires a command");
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