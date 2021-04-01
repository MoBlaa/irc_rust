use std::fmt;
use std::fmt::{Display, Formatter};

use crate::builder::Message as MessageBuilder;
use crate::errors::ParserError;
use crate::params::Parameterized;
use crate::parsed::Parsed;
use crate::tokenizer::{PartialCfg, Prefix, Start, Tokenizer};
use std::convert::TryFrom;

/// A simple irc message containing tags, prefix, command, parameters and a trailing parameter.
///
/// All types returned from getters of this type ([Prefix, Params, Tags]) are owned types. So they are tied to the [Message] instance they are retrieved from and don't own their part of the message.
///
/// Parses its part lazily on method invokations.
///
/// # Examples
///
/// Create a Message from a plain string.
///
/// ```rust
/// use irc_rust::Message;
///
/// let message = Message::from("@key1=value1;key2=value2 :name!user@host CMD param1 param2 :trailing");
///
/// assert_eq!(message.to_string(), "@key1=value1;key2=value2 :name!user@host CMD param1 param2 :trailing");
/// ```
///
/// To build a message in a verbose and easy to read way you can use the `Message::builder` method and the `MessageBuilder`.
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
/// You can create a new message from an existing message by calling the `to_builder` method.
/// To alter existing parameters the `set_param` method can be used.
///
/// ```rust
/// use irc_rust::Message;
/// use std::error::Error;
///
/// fn main() -> Result<(), Box<dyn Error>> {
///     let message = Message::from("@key=value :name!user@host CMD param1 :trailing!").to_builder()?
///     .tag("key", "value2")
///     .param("param2")
///     .param("param4")
///     .set_param(1, "param3")
///     .build();
///
///     assert_eq!(message.to_string(), "@key=value2 :name!user@host CMD param1 param3 param4 :trailing!");
///     Ok(())
/// }
/// ```
#[derive(Debug, Clone, Eq, Ord, PartialOrd, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Message {
    raw: String,
}

impl Message {
    /// Returns a fully parsed but zero-copy struct referencing the struct.
    pub fn parsed(&self) -> Result<Parsed, ParserError> {
        Parsed::try_from(self.raw.as_str())
    }

    /// Returns a query instance to partially parse the message.
    pub fn partial<'a>(&'a self, cfg: PartialCfg<'a>) -> Result<Parsed<'a>, ParserError> {
        Tokenizer::new(self.raw.as_str())?.into_parsed(cfg)
    }

    pub fn tokenizer(&self) -> Result<Tokenizer<Start>, ParserError> {
        Tokenizer::new(self.raw.as_str())
    }

    /// Creates a message builder as alternative to building an irc string before creating the message.
    pub fn builder(command: &str) -> MessageBuilder {
        MessageBuilder::new(command)
    }

    /// Creates a builder from this message. Only initializes fields already present in the message.
    /// By using this method a whole new Message will be created.
    pub fn to_builder(&self) -> Result<MessageBuilder<'_>, ParserError> {
        let parsed = Parsed::try_from(self.raw.as_str())?;

        let mut builder = MessageBuilder::new(parsed.command().ok_or(ParserError::NoCommand)?);
        for (key, value) in parsed.tags() {
            builder = builder.tag(key, value)
        }
        if let Some(parsed) = parsed.prefix() {
            builder = builder.prefix(
                parsed.name().ok_or(ParserError::PrefixWithoutName)?,
                parsed.user(),
                parsed.host(),
            );
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

    /// Returns tags if any are present.
    pub fn tags(
        &self,
    ) -> Result<impl Iterator<Item = Result<(&str, &str), ParserError>>, ParserError> {
        Tokenizer::new(self.raw.as_str()).map(|tokenizer| tokenizer.tags().into_iter())
    }

    /// Returns the Prefix if present.
    pub fn prefix(&self) -> Result<Option<Prefix>, ParserError> {
        Tokenizer::new(self.raw.as_str()).and_then(|tokenizer| tokenizer.prefix().parts())
    }

    /// Returns the command the message represents.
    pub fn command(&self) -> Result<&str, ParserError> {
        Tokenizer::new(self.raw.as_str()).and_then(|tokenizer| tokenizer.command().command())
    }

    /// Returns the params if any are present.
    pub fn params(&self) -> Result<impl Iterator<Item = &str>, ParserError> {
        Tokenizer::new(self.raw.as_str()).map(|tokenizer| tokenizer.params().into_iter())
    }

    /// Returns the trailing parameter if any is present.
    pub fn trailing(&self) -> Result<Option<&str>, ParserError> {
        Tokenizer::new(self.raw.as_str()).map(|tokenizer| tokenizer.trailing().trailing())
    }
}

impl Display for Message {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        self.raw.fmt(f)
    }
}

impl From<String> for Message {
    fn from(raw: String) -> Self {
        Message { raw }
    }
}

impl From<&str> for Message {
    fn from(raw: &str) -> Self {
        Message {
            raw: raw.to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::message::Message;

    #[test]
    #[cfg(feature = "serde")]
    fn test_serde() {
        let message =
            Message::from("@test=test :user@prefix!host COMMAND param :trailing".to_string());
        let serialized = serde_json::to_string(&message).unwrap();
        println!("Ser: {}", serialized);
        let deserialized: Message = serde_json::from_str(serialized.as_str()).unwrap();
        assert_eq!(deserialized.to_string(), message.to_string());
    }

    #[test]
    fn test_tags() {
        let message =
            Message::from("@test=test :user@prefix!host COMMAND param :trailing".to_string());
        let tags = message.tags();
        assert!(tags.is_ok(), "{:?}", tags.err());
        let mut tags = tags.unwrap().into_iter();
        let tag = tags.next();
        assert!(tag.is_some(), "{:?}", tag);
    }

    #[test]
    fn test_prefix() {
        let message =
            Message::from("@test=test :user@prefix!host COMMAND param :trailing".to_string());
        let prefix = message.prefix();
        assert!(prefix.is_ok(), "{:?}", prefix);
        let prefix = prefix.unwrap();
        assert!(prefix.is_some(), "{:?}", prefix);
    }
}
