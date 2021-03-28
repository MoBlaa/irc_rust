use crate::prefix::Prefixed;
use crate::tags::Taggable;
use crate::{InvalidIrcFormatError, Message};
use std::collections::HashMap;
use std::convert::TryFrom;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct ParsedPrefix<'a>(pub &'a str, pub Option<&'a str>, pub Option<&'a str>);

impl<'a> Prefixed<'a> for ParsedPrefix<'a> {
    fn name(&self) -> &'a str {
        self.0
    }

    fn user(&self) -> Option<&'a str> {
        self.1
    }

    fn host(&self) -> Option<&'a str> {
        self.2
    }
}

impl<'a> From<(&'a str, Option<&'a str>, Option<&'a str>)> for ParsedPrefix<'a> {
    fn from((name, user, host): (&'a str, Option<&'a str>, Option<&'a str>)) -> Self {
        Self(name, user, host)
    }
}

/// Fully parsed Message instead of parsing on demand. Instead of
/// zero-allocation this struct implements zero-copy parsing.
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Parsed<'a> {
    tags: HashMap<&'a str, &'a str>,
    prefix: Option<ParsedPrefix<'a>>,
    command: &'a str,
    params: Vec<&'a str>,
    trailing: Option<&'a str>,
}

impl<'a> Parsed<'a> {
    pub(crate) fn new(
        tags: HashMap<&'a str, &'a str>,
        prefix: Option<ParsedPrefix<'a>>,
        command: &'a str,
        params: Vec<&'a str>,
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

    pub fn command(&self) -> &'a str {
        self.command
    }

    pub fn prefix(&self) -> Option<&ParsedPrefix<'a>> {
        self.prefix.as_ref()
    }

    pub fn trailing(&self) -> Option<&&'a str> {
        self.trailing.as_ref()
    }

    pub fn param(&self, index: usize) -> Option<&&'a str> {
        self.params.get(index)
    }

    pub fn params(&self) -> impl Iterator<Item = &&'a str> {
        self.params.iter()
    }

    pub fn tag(&self, key: &str) -> Option<&&'a str> {
        self.tags.get(key)
    }

    pub fn tags(&self) -> impl Iterator<Item = (&&'a str, &&'a str)> {
        self.tags.iter()
    }
}

impl<'a> Taggable<'a> for Parsed<'a> {
    fn tag(&self, key: &str) -> Option<&'a str> {
        // TODO: Remove copy
        self.tags.get(key).copied()
    }
}

impl<'a> TryFrom<&'a Message> for Parsed<'a> {
    type Error = InvalidIrcFormatError;

    fn try_from(value: &'a Message) -> Result<Self, Self::Error> {
        let tags = value
            .tags()?
            .map(|tags| tags.iter().collect::<HashMap<_, _>>())
            .unwrap_or_default();

        let prefix = value.prefix()?.map(|prefix| prefix.into_parts().into());

        let (params, trailing) = value
            .params()
            .map(|param| param.into_parts())
            .map(|(params, trailing)| (params.collect::<Vec<_>>(), trailing))
            .unwrap_or_default();

        Ok(Self {
            tags,
            prefix,
            command: value.command(),
            params,
            trailing,
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::prefix::Prefixed;
    use crate::{InvalidIrcFormatError, Message};

    #[test]
    fn test_parsed() -> Result<(), InvalidIrcFormatError> {
        let message = Message::builder("CMD")
            .tag("tag1", "value1")
            .tag("tag2", "value2")
            .prefix("name", Some("user"), Some("host"))
            .param("param0")
            .trailing("Trailing Parameter!")
            .build();
        let parsed = message.parsed()?;

        assert_eq!(Some(&"value1"), parsed.tag("tag1"));
        assert_eq!(Some(&"value2"), parsed.tag("tag2"));
        assert_eq!(
            Some(("name", Some("user"), Some("host"))),
            parsed.prefix().map(|prefix| prefix.as_parts())
        );
        assert_eq!(Some(&"param0"), parsed.param(0));
        assert_eq!(Some(&"Trailing Parameter!"), parsed.trailing());

        Ok(())
    }
}
