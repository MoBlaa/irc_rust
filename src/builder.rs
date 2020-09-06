use std::collections::HashMap;
use crate::Message;

/// A MessageBuilder for a simpler generation of a message instead of building an string first.
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
    /// Creates a new empty builder.
    pub fn new() -> Self {
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
    pub fn build(self) -> Message {
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
        Message::from(str)
    }
}
