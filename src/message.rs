use crate::params::Params;
use crate::prefix::{Prefix};
use crate::tags::Tags;

pub struct Message<'a> {
    raw: &'a str
}

impl<'a> Message<'a> {
    pub fn new(value: &'a str) -> Message<'a> {
        Message {
            raw: value
        }
    }

    pub fn from(str: &'a str) -> Message<'a> {
        Message {
            raw: str
        }
    }

    pub fn tags(&self) -> Option<Tags> {
        if self.raw.starts_with('@') {
            self.raw.find(' ').and_then(|index| Some(Tags::new(&self.raw[1..index])))
        } else {
            None
        }
    }

    pub fn prefix(&self) -> Option<Prefix<'a>> {
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

    pub fn command(&self) -> &'a str {
        let without_tags = match self.raw.find(' ') {
            Some(start) => {
                if self.raw.starts_with("@") {
                    &self.raw[start + 1..]
                } else {
                    self.raw
                }
            }
            None => self.raw
        };
        let without_prefix = match without_tags.find(' ') {
            Some(start) => {
                if without_tags.starts_with(":") {
                    &without_tags[start + 1..]
                } else {
                    without_tags
                }
            }
            None => self.raw
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

impl<'a> ToString for Message<'a> {
    fn to_string(&self) -> String {
        self.raw.to_string()
    }
}