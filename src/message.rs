use crate::params::Params;
use crate::prefix::{Prefix, PrefixBuilder};
use crate::tags::Tags;

pub struct Message {
    raw: String
}

impl Message {
    pub fn new(value: String) -> Message {
        Message {
            raw: value
        }
    }

    pub fn from(str: &str) -> Message {
        Message {
            raw: str.to_string()
        }
    }

    pub fn builder(command: &str) -> MessageBuilder {
        MessageBuilder {
            tags: Vec::new(),
            prefix: None,
            command,
            params: Vec::new(),
            trailing: None,
        }
    }

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
                Some(tags.raw.len() + 2)
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
        let without_prefix = match self.raw.find(' ') {
            Some(start) => {
                if self.raw.starts_with(":") {
                    &self.raw[start + 1..]
                } else {
                    &self.raw
                }
            }
            None => self.raw.as_str()
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

impl<'a> ToString for Message {
    fn to_string(&self) -> String {
        self.raw.to_string()
    }
}

pub struct MessageBuilder<'a> {
    tags: Vec<String>,
    prefix: Option<Prefix>,
    command: &'a str,
    params: Vec<String>,
    trailing: Option<String>,
}

impl<'a> MessageBuilder<'a> {
    pub fn tag(mut self, key: &'a str, value: &'a str) -> MessageBuilder<'a> {
        self.tags.push(format!("{}={}", key, value));
        self
    }

    pub fn prefix(mut self, prefix_builder: PrefixBuilder) -> MessageBuilder<'a> {
        self.prefix = Some(prefix_builder.build());
        self
    }

    pub fn param(mut self, param: &'a str) -> MessageBuilder<'a> {
        self.params.push(param.to_string());
        self
    }

    pub fn trailing(mut self, trailing: &'a str) -> MessageBuilder<'a> {
        self.trailing = Some(trailing.to_string());
        self
    }

    pub fn build(self) -> Message {
        let mut str = if !self.tags.is_empty() {
            format!("@{} ", self.tags.join("="))
        } else {
            String::new()
        };
        if let Some(prefix) = self.prefix {
            str = format!("{}:{} ", str, prefix.to_string())
        }
        str = format!("{}{}", str, self.command);
        if !self.params.is_empty() {
            str = format!("{} {}", str, self.params.join(" "));
        }
        if let Some(trailing) = self.trailing {
            str = format!("{} :{}", str, trailing);
        }
        Message {
            raw: str
        }
    }
}