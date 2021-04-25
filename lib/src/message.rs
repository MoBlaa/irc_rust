use std::fmt;
use std::fmt::{Display, Formatter};

use crate::builder::Builder as MessageBuilder;
use crate::errors::ParserError;
use crate::parsed::Parsed;
use crate::prefix::Prefix;
use crate::tokenizer::{PartialCfg, Start, Tokenizer};
use std::convert::TryFrom;
use std::str::FromStr;

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
/// # fn main() -> Result<(), irc_rust::errors::ParserError> {
/// let message = Message::from("@key1=value1;key2=value2 :name!user@host CMD param1 param2 :trailing");
///
/// // Or
///
/// let message = "@key1=value1;key2=value2 :name!user@host CMD param1 param2 :trailing"
///     .parse::<Message>()?;
///
/// assert_eq!(message.to_string(), "@key1=value1;key2=value2 :name!user@host CMD param1 param2 :trailing");
/// # Ok(())
/// # }
/// ```
///
/// To build a message in a verbose and easy to read way you can use the `Message::builder` method and the `MessageBuilder`.
#[derive(Debug, Clone, Eq, Ord, PartialOrd, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Message {
    raw: String,
}

impl Message {
    /// Returns a fully parsed but zero-copy struct referencing the parsed message.
    pub fn parse(&self) -> Result<Parsed, ParserError> {
        Parsed::try_from(self.raw.as_str())
    }

    /// Returns a query instance to partially parse the message.
    ///
    /// # Usage
    ///
    /// ```rust
    /// use irc_rust::Message;
    /// use irc_rust::tokenizer::PartialCfg;
    /// use std::collections::HashSet;
    /// use std::iter::FromIterator;
    /// # fn main() -> Result<(), irc_rust::errors::ParserError> {
    /// let message = "@tag1=value1;tag2=value2 CMD param0 param1 :trailing"
    ///     .parse::<Message>()?;
    /// let parsed = message.parse_partial(PartialCfg {
    ///         tags: HashSet::from_iter(vec!["tag2"]),
    ///         params: vec![1],
    ///         trailing: true,
    ///         ..PartialCfg::default()
    ///     })?;
    /// assert_eq!(Some("CMD"), parsed.command());
    /// assert!(parsed.tag("tag1").is_none());
    /// assert!(parsed.prefix().is_none());
    /// assert_eq!(Some("value2"), parsed.tag("tag2"));
    /// assert_eq!(Some("param1"), parsed.param(1));
    /// # Ok(())
    /// # }
    /// ```
    pub fn parse_partial<'a>(&'a self, cfg: PartialCfg<'a>) -> Result<Parsed<'a>, ParserError> {
        Tokenizer::new(self.raw.as_str())?.parse_partial(cfg)
    }

    /// Returns a tokenizer over the message. Can be used to implement a custom parsing algorithm.
    pub fn tokenizer(&self) -> Result<Tokenizer<Start>, ParserError> {
        Tokenizer::new(self.raw.as_str())
    }

    /// Creates a message builder as alternative to building an irc string before creating the message.
    pub fn builder(command: &str) -> MessageBuilder {
        MessageBuilder::new(command)
    }

    /// Creates a builder from this message. Only initializes fields already present in the message.
    /// By using this method a whole new Message will be created.
    pub fn to_builder(&self) -> Result<MessageBuilder, ParserError> {
        MessageBuilder::from_str(self.raw.as_str())
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

impl FromStr for Message {
    type Err = ParserError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Message::from(s.to_string()))
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
