use crate::errors::ParserError;
use crate::parsed::Parsed;
use std::collections::HashMap;
use std::convert::TryFrom;
use std::str::FromStr;

/// A Message Builder for a simpler generation of a message instead of building a string first.
///
/// # Examples
///
/// To build a simple message from scratch:
///
/// ```rust
/// use irc_rust::Message;
/// use std::error::Error;
///
/// # fn main() -> Result<(), irc_rust::errors::ParserError> {
/// let message = Message::builder("CMD")
///     .tag("key1", "value1")
///     .tag("key2", "value2")
///     .prefix("name", Some("user"), Some("host"))
///     .param("param1").param("param2")
///     .trailing("trailing")
///     .build();
///
/// let mut tags = message.tags()?;
/// let (key, value) = tags.next().unwrap()?;
/// println!("{}={}", key, value); // Prints 'key1=value1'
/// # Ok(())
/// # }
/// ```
///
/// To alter an existing message:
///
/// ```rust
/// use irc_rust::Message;
/// use std::error::Error;
/// use irc_rust::builder::Builder;
///
/// # fn main() -> Result<(), Box<dyn Error>> {
/// let message = Message::from("@key=value :name!user@host CMD param1 :trailing!").to_builder()?
///     .tag("key", "value2")
///     .param("param2")
///     .param("param4")
///     .set_param(1, "param3")
///     .build();
///
/// // Or
/// let message: Message = "@key=value :name!user@host CMD param1 :trailing!".parse::<Builder>()?
///     .tag("key", "value2")
///     .param("param2")
///     .param("param4")
///     .set_param(1, "param3")
///     .build();
///
/// assert_eq!(message.to_string(), "@key=value2 :name!user@host CMD param1 param3 param4 :trailing!");
/// Ok(())
/// # }
/// ```
#[derive(Debug, Clone, Eq, PartialEq, Default)]
pub struct Builder {
    tags: HashMap<String, String>,
    prefix_name: Option<String>,
    prefix_user: Option<String>,
    prefix_host: Option<String>,
    command: String,
    params: Vec<String>,
    trailing: Option<String>,
}

impl Builder {
    /// Creates a new empty builder.
    pub fn new<S: ToString>(command: S) -> Self {
        Builder {
            tags: HashMap::new(),
            prefix_name: None,
            prefix_user: None,
            prefix_host: None,
            command: "".to_string(),
            params: Vec::new(),
            trailing: None,
        }
        .command(command.to_string())
    }

    /// Set the command.
    ///
    /// # Panics
    ///
    /// Panics if **cmd** is empty.
    pub fn command<S: ToString>(mut self, cmd: S) -> Builder {
        let cmd = cmd.to_string();
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
    pub fn tag<SK: ToString, SV: ToString>(mut self, key: SK, value: SV) -> Builder {
        let key = key.to_string();
        if key.is_empty() {
            panic!("tried to set tag with empty key");
        }
        self.tags.insert(key, value.to_string());
        self
    }

    /// Set a prefix name.
    ///
    /// # Panics
    ///
    /// Panics if **name** is empty, **user or host** == **Some("")** or **user** is some and **host** is none.
    pub fn prefix<SN, SU, SH>(mut self, name: SN, user: Option<SU>, host: Option<SH>) -> Builder
    where
        SN: ToString,
        SU: ToString,
        SH: ToString,
    {
        let name = name.to_string();
        if name.is_empty() {
            panic!("tried to set empty prefix name");
        }
        let user = user.map(|user| user.to_string());
        if user.is_some() && user.as_ref().unwrap().is_empty() {
            panic!("tried to set empty prefix user");
        }
        let host = host.map(|host| host.to_string());
        if host.is_some() && host.as_ref().unwrap().is_empty() {
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
    pub fn param<S: ToString>(mut self, param: S) -> Builder {
        let param = param.to_string();
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
    pub fn set_param<S: ToString>(mut self, index: usize, param: S) -> Builder {
        let param = param.to_string();
        if param.is_empty() {
            panic!("tried to set empty param");
        }
        if index >= self.params.len() {
            self.params.push(param);
        } else {
            self.params[index] = param;
        }
        self
    }

    pub fn remove_param(mut self, index: usize) -> Builder {
        if index < self.params.len() {
            self.params.remove(index);
        }
        self
    }

    //( Add a trailing param;
    pub fn trailing<S: ToString>(mut self, trailing: S) -> Builder {
        self.trailing = Some(trailing.to_string());
        self
    }

    /// Create a Message instance and return if valid.
    pub fn build(self) -> crate::message::Message {
        let mut str = String::new();
        if !self.tags.is_empty() {
            str.push('@');
            for (key, val) in self.tags {
                str.push_str(key.as_str());
                str.push('=');
                str.push_str(val.as_str());
                str.push(';')
            }
            str.pop();
            str.push(' ');
        }
        if let Some(prefix_name) = self.prefix_name {
            str.push(':');
            str.push_str(prefix_name.as_str());
            // Asserting as checked in setters.
            assert!(self.prefix_user.is_none() || self.prefix_host.is_some());
            if let Some(user) = self.prefix_user {
                str.push('!');
                str.push_str(user.as_str());
            }
            if let Some(host) = self.prefix_host {
                str.push('@');
                str.push_str(host.as_str());
            }
            str.push(' ')
        }
        str.push_str(self.command.as_str());
        if !self.params.is_empty() {
            str.push(' ');
            str.push_str(&self.params.join(" "));
        }
        if let Some(trailing) = self.trailing {
            str.push_str(" :");
            str.push_str(trailing.as_str());
        }
        crate::message::Message::from(str)
    }
}

impl FromStr for Builder {
    type Err = ParserError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parsed = Parsed::try_from(s)?;

        let mut builder = Builder::new(parsed.command().ok_or(ParserError::NoCommand)?);
        for (key, value) in parsed.tags() {
            builder = builder.tag(key, value)
        }
        if let Some(&(name, user, host)) = parsed.prefix() {
            builder = builder.prefix(name, user, host);
        }
        // Flatten to remove empty params
        for param in parsed.params().flatten() {
            builder = builder.param(param);
        }
        if let Some(trailing) = parsed.trailing() {
            builder = builder.trailing(trailing);
        }

        Ok(builder)
    }
}
