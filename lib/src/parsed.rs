use crate::errors::ParserError;
use crate::params::Parameterized;
use crate::prefix::{Prefix, Prefixed};
use crate::tags::Taggable;
use crate::tokenizer::Tokenizer;
use std::collections::HashMap;
use std::convert::TryFrom;

/// Fully parsed Message instead of parsing on demand. Instead of
/// zero-allocation this struct implements zero-copy parsing.
///
/// Implements a partially or fully parsed message.
///
/// Doesn't implement the [std::convert::FromStr] trait as its lifetime
/// depends on its source string.
#[derive(Debug, Clone, Eq, PartialEq, Default)]
pub struct Parsed<'a> {
    tags: HashMap<&'a str, &'a str>,
    prefix: Option<Prefix<'a>>,
    command: Option<&'a str>,
    params: Vec<Option<&'a str>>,
    trailing: Option<&'a str>,
}

impl<'a> Parsed<'a> {
    pub(crate) fn new(
        tags: HashMap<&'a str, &'a str>,
        prefix: Option<Prefix<'a>>,
        command: Option<&'a str>,
        params: Vec<Option<&'a str>>,
        trailing: Option<&'a str>,
    ) -> Self {
        Self {
            tags,
            prefix,
            command,
            params,
            trailing,
        }
    }

    pub fn command(&self) -> Option<&'a str> {
        self.command
    }

    pub fn prefix(&self) -> Option<&Prefix<'a>> {
        self.prefix.as_ref()
    }

    pub fn params(&self) -> impl Iterator<Item = &Option<&'a str>> {
        self.params.iter()
    }

    pub fn tags(&self) -> impl Iterator<Item = (&&'a str, &&'a str)> {
        self.tags.iter()
    }
}

impl<'a> Taggable<'a> for Parsed<'a> {
    fn tag(&self, key: &str) -> Option<&'a str> {
        self.tags.get(key).copied()
    }
}

impl<'a> Parameterized<'a> for Parsed<'a> {
    fn param(&self, index: usize) -> Option<&'a str> {
        match self.params.get(index) {
            Some(Some(st)) => Some(*st),
            _ => None,
        }
    }

    fn trailing(&self) -> Option<&'a str> {
        self.trailing
    }
}

impl<'a> Prefixed<'a> for Parsed<'a> {
    fn name(&self) -> Option<&'a str> {
        self.prefix.as_ref().map(|&(name, _user, _host)| name)
    }

    fn user(&self) -> Option<&'a str> {
        self.prefix.as_ref().and_then(|&(_name, user, _host)| user)
    }

    fn host(&self) -> Option<&'a str> {
        self.prefix.as_ref().and_then(|&(_name, _user, host)| host)
    }
}

impl<'a> TryFrom<&'a str> for Parsed<'a> {
    type Error = ParserError;

    fn try_from(value: &'a str) -> Result<Self, Self::Error> {
        let mut tokenizer = Tokenizer::new(value)?.tags();
        let iter = tokenizer.as_iter();
        let mut tags = HashMap::new();
        for res in iter {
            let (key, value) = res?;
            tags.insert(key, value);
        }
        let mut tokenizer = tokenizer.prefix();
        let prefix = tokenizer
            .parts()?
            .map(|(name, user, host)| (name, user, host));
        let mut tokenizer = tokenizer.command();
        let command = tokenizer.command()?;
        let mut tokenizer = tokenizer.params();
        let params = tokenizer.as_iter().map(Some).collect::<Vec<_>>();
        let trailing = tokenizer.trailing().trailing();

        Ok(Self {
            tags,
            prefix,
            command: Some(command),
            params,
            trailing,
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::params::Parameterized;
    use crate::tags::Taggable;
    use crate::Message;
    use std::error::Error;

    #[test]
    fn test_parsed() -> Result<(), Box<dyn Error>> {
        let message = Message::builder("CMD")
            .tag("tag1", "value1")
            .tag("tag2", "value2")
            .prefix("name", Some("user"), Some("host"))
            .param("param0")
            .trailing("Trailing Parameter!")
            .build();
        let parsed = message.parsed()?;

        assert_eq!(Some("value1"), parsed.tag("tag1"));
        assert_eq!(Some("value2"), parsed.tag("tag2"));
        assert_eq!(
            Some(("name", Some("user"), Some("host"))),
            parsed.prefix().map(|prefix| (prefix.0, prefix.1, prefix.2))
        );
        assert_eq!(Some("param0"), parsed.param(0));
        assert_eq!(Some("Trailing Parameter!"), parsed.trailing());

        Ok(())
    }
}
