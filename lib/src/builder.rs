use std::collections::HashMap;

/// A MessageBuilder for a simpler generation of a message instead of building an string first.
#[derive(Debug, Clone, Eq, PartialEq, Default)]
pub struct Message<'a> {
    tags: HashMap<&'a str, &'a str>,
    prefix_name: Option<&'a str>,
    prefix_user: Option<&'a str>,
    prefix_host: Option<&'a str>,
    command: &'a str,
    params: Vec<&'a str>,
    trailing: Option<&'a str>,
}

impl<'a> Message<'a> {
    /// Creates a new empty builder.
    pub fn new(command: &'a str) -> Self {
        Message {
            tags: HashMap::new(),
            prefix_name: None,
            prefix_user: None,
            prefix_host: None,
            command: "",
            params: Vec::new(),
            trailing: None,
        }
        .command(command)
    }

    /// Set the command.
    ///
    /// # Panics
    ///
    /// Panics if **cmd** is empty.
    pub fn command(mut self, cmd: &'a str) -> Message<'a> {
        if cmd.is_empty() {
            panic!("tried to set empty command");
        }
        self.command = cmd;
        self
    }

    /// Set a tag.
    ///
    /// # Panics
    ///
    /// Panics if **key** is empty. **value** is allowed to be empty.
    pub fn tag(mut self, key: &'a str, value: &'a str) -> Message<'a> {
        if key.is_empty() {
            panic!("tried to set tag with empty key");
        }
        self.tags.insert(key, value);
        self
    }

    /// Set a prefix name.
    ///
    /// # Panics
    ///
    /// Panics if **name** is empty, **user or host** == **Some("")** or **user** is some and **host** is none.
    pub fn prefix(
        mut self,
        name: &'a str,
        user: Option<&'a str>,
        host: Option<&'a str>,
    ) -> Message<'a> {
        if name.is_empty() {
            panic!("tried to set empty prefix name");
        }
        if user.is_some() && user.unwrap().is_empty() {
            panic!("tried to set empty prefix user");
        }
        if host.is_some() && host.unwrap().is_empty() {
            panic!("tried to set empty prefix host");
        }
        if user.is_some() && host.is_none() {
            panic!("tried to set prefix user without host");
        }
        self.prefix_name = Some(name);
        self.prefix_user = user;
        self.prefix_host = host;
        self
    }

    /// Add a param.
    ///
    /// # Panics
    ///
    /// Panics if **param** is empty.
    pub fn param(mut self, param: &'a str) -> Message<'a> {
        if param.is_empty() {
            panic!("tried to add empty param");
        }
        self.params.push(param);
        self
    }

    /// Set a param at the given index. If the index is below 0, it won't be set.
    /// If index >= length of the existing parameters it will be added to the end but not set as trailing.
    /// This doesn't allow to set the trailing parameter.
    ///
    /// # Panics
    ///
    /// Panics if **param** is empty.
    pub fn set_param(mut self, index: usize, param: &'a str) -> Message<'a> {
        if param.is_empty() {
            panic!("tried to set empty param");
        }
        if index >= self.params.len() {
            self.params.push(param);
        }
        self.params[index] = param;
        self
    }

    pub fn remove_param(mut self, index: usize) -> Message<'a> {
        if index < self.params.len() {
            self.params.remove(index);
        }
        self
    }

    //( Add a trailing param;
    pub fn trailing(mut self, trailing: &'a str) -> Message<'a> {
        self.trailing = Some(trailing);
        self
    }

    /// Create a Message instance and return if valid.
    pub fn build(self) -> crate::message::Message {
        let mut str = String::new();
        if !self.tags.is_empty() {
            str.push('@');
            for (key, val) in self.tags {
                str.push_str(key);
                str.push('=');
                str.push_str(val);
                str.push(';')
            }
            str.pop();
            str.push(' ');
        }
        if let Some(prefix_name) = self.prefix_name {
            str.push(':');
            str.push_str(prefix_name);
            // Asserting as checked in setters.
            assert!(self.prefix_user.is_none() || self.prefix_host.is_some());
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
        str.push_str(self.command);
        if !self.params.is_empty() {
            str.push(' ');
            str.push_str(&self.params.join(" "));
        }
        if let Some(trailing) = self.trailing {
            str.push_str(" :");
            str.push_str(trailing);
        }
        crate::message::Message::from(str)
    }
}
