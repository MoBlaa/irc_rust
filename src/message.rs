use std::collections::HashMap;
use std::fmt::{Display, Formatter};
use std::fmt;
use std::iter::FromIterator;

use crate::params::Params;
use crate::prefix::Prefix;
use crate::tags::Tags;

/// A simple irc message containing tags, prefix, command, parameters and a trailing parameter.
///
/// # Examples
/// ```
/// use irc_rust::message::Message;
///
/// let message = Message::from("@key1=value1;key2=value2 :name!user@host CMD param1 param2 :trailing");
///
/// assert_eq!(message.to_string(), "@key1=value1;key2=value2 :name!user@host CMD param1 param2 :trailing");
/// ```
///
/// ```
/// use irc_rust::message::Message;
///
/// let message = Message::builder()
///         .tag("key1", "value1")
///         .tag("key2", "value2")
///         .prefix_name("name")
///         .prefix_user("user")
///         .prefix_host("host")
///         .command("CMD")
///         .param("param1").param("param2")
///         .trailing("trailing")
///         .build();
///
/// let tags = message.tags().unwrap();
/// println!("key1={}", &tags["key1"]) // Prints 'key1=value1'
///
/// ```
pub struct Message {
    raw: String
}

impl Message {
    /// Create a new Message from the given string. Expects the string to be in a valid irc format.
    pub fn new(raw: String) -> Message {
        Message {
            raw
        }
    }

    // Create a new Message from the given String view. Expects the string to be in a valid irc format.
    pub fn from(raw: &str) -> Message {
        Message {
            raw: raw.to_string()
        }
    }

    /// Creates a message builder as alternative to building an irc string before creating the message.
    pub fn builder<'a>() -> MessageBuilder<'a> {
        MessageBuilder {
            tags: HashMap::new(),
            prefix_name: None,
            prefix_user: None,
            prefix_host: None,
            command: None,
            params: Vec::new(),
            trailing: None,
        }
    }

    /// Creates a builder from this message. Only initializes fields already present in the message.
    /// By using this method a whole new Message will be created.
    pub fn to_builder(&self) -> MessageBuilder<'_> {
        MessageBuilder {
            tags: if let Some(tags) = self.tags() {
                HashMap::from_iter(tags.iter())
            } else {
                HashMap::new()
            },
            prefix_name: self.prefix().map(|prefix| prefix.name()),
            prefix_user: self.prefix().and_then(|prefix| prefix.user()),
            prefix_host: self.prefix().and_then(|prefix| prefix.host()),
            command: Some(self.command()),
            params: if let Some(params) = self.params() {
                Vec::from_iter(params.iter())
            } else {
                Vec::new()
            },
            trailing: self.params().and_then(|params| params.trailing),
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

impl Display for Message {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.raw)
    }
}

//// A MessageBuilder for a simpler generation of a message instead of building an string first.
pub struct MessageBuilder<'a> {
    tags: HashMap<&'a str, &'a str>,
    prefix_name: Option<&'a str>,
    prefix_user: Option<&'a str>,
    prefix_host: Option<&'a str>,
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

    /// Set a tag.
    pub fn tag(mut self, key: &'a str, value: &'a str) -> MessageBuilder<'a> {
        self.tags.insert(key, value);
        self
    }

    /// Set a prefix.
    pub fn prefix_name(mut self, name: &'a str) -> MessageBuilder<'a> {
        self.prefix_name = Some(name);
        self
    }

    /// Set a prefix.
    pub fn prefix_user(mut self, user: &'a str) -> MessageBuilder<'a> {
        self.prefix_user = Some(user);
        self
    }

    /// Set a prefix.
    pub fn prefix_host(mut self, host: &'a str) -> MessageBuilder<'a> {
        self.prefix_host = Some(host);
        self
    }

    /// Add a param.
    pub fn param(mut self, param: &'a str) -> MessageBuilder<'a> {
        self.params.push(param);
        self
    }

    /// Set a param at the given index. If the index is below 0, it won't be set.
    /// If index >= length of the existing parameters it will be added to the end but not set as trailing.
    /// This doesn't allow to set the trailing parameter.
    pub fn set_param(mut self, index: usize, param: &'a str) -> MessageBuilder<'a> {
        if index >= self.params.len() {
            self.params.push(param);
        }
        self.params[index] = param;
        self
    }

    //( Add a trailing param;
    pub fn trailing(mut self, trailing: &'a str) -> MessageBuilder<'a> {
        self.trailing = Some(trailing);
        self
    }

    /// Create a Message instance and return if valid.
    pub fn build<'b>(self) -> Message {
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
        if let Some(prefix_name) = self.prefix_name {
            str.push(':');
            str.push_str(prefix_name);
            if self.prefix_user.is_some() && self.prefix_host.is_none() {
                panic!("irc prefix can only contain a user if host is also present");
            }
            if let Some(user) = self.prefix_user {
                str.push('!');
                str.push_str(user);
            }
            if let Some(host) = self.prefix_host {
                str.push('@');
                str.push_str(host);
            }
            str.push(' ')
        }
        if let Some(command) = self.command {
            str.push_str(command);
        } else {
            panic!("irc message requires an command");
        }
        if !self.params.is_empty() {
            str.push(' ');
            str.push_str(&self.params.join(" "));
        }
        if let Some(trailing) = self.trailing {
            str.push_str(" :");
            str.push_str(trailing);
        }
        Message {
            raw: str
        }
    }
}