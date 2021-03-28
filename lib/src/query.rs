use crate::parsed::ParsedPrefix;
use crate::prefix::Prefixed;
use crate::tags::Taggable;
use crate::{InvalidIrcFormatError, Message, Parsed};
use std::collections::{HashMap, HashSet};
use std::ops::Deref;

/// Implements Zero-Copy __partial__ parsing by only parsing and extracting the desired
/// parts of the message. Implements the state pattern for a clear query structure.
pub struct Query<'a, S: State> {
    message: &'a Message,
    state: S,
}

pub trait State {}

pub struct Init;

impl State for Init {}

pub struct TagsState<'a> {
    tags: HashMap<&'a str, &'a str>,
}

impl<'a> State for TagsState<'a> {}

impl<'a> Taggable<'a> for TagsState<'a> {
    fn tag(&self, key: &str) -> Option<&'a str> {
        // TODO: Remove Copy
        self.tags.get(key).copied()
    }
}

pub struct PrefixState<'a> {
    tags: HashMap<&'a str, &'a str>,
    prefix: Option<ParsedPrefix<'a>>,
}

impl<'a> PrefixState<'a> {
    pub fn prefix(&self) -> Option<&ParsedPrefix<'a>> {
        self.prefix.as_ref()
    }
}

impl<'a> State for PrefixState<'a> {}

impl<'a> State for Parsed<'a> {}

impl<'a, T: State> Query<'a, T> {
    pub fn command(&self) -> &'a str {
        self.message.command()
    }

    fn parse_tags(
        &self,
        mut searched: HashSet<&'a str>,
    ) -> Result<HashMap<&'a str, &'a str>, InvalidIrcFormatError> {
        let parsed = self
            .message
            .tags()?
            .map(|tags| {
                let mut result = HashMap::with_capacity(searched.len());
                for (key, value) in tags.iter() {
                    if searched.contains(&key) {
                        result.insert(key, value);
                        searched.remove(&key);
                        if searched.is_empty() {
                            // Early abortion if all tags have been found to not parse all tags
                            break;
                        }
                    }
                }
                result
            })
            .unwrap_or_default();
        Ok(parsed)
    }

    fn parse_prefix(
        &self,
        user: bool,
        host: bool,
    ) -> Result<Option<ParsedPrefix<'a>>, InvalidIrcFormatError> {
        let prefix = match self.message.prefix()? {
            None => None,
            Some(prefix) => Some(ParsedPrefix(
                prefix.name(),
                if user { prefix.user() } else { None },
                if host { prefix.host() } else { None },
            )),
        };
        Ok(prefix)
    }

    fn parse_params(
        &self,
        mut param_indexes: Vec<usize>,
        trailing: bool,
    ) -> Result<(Vec<&'a str>, Option<&'a str>), InvalidIrcFormatError> {
        param_indexes.dedup();
        // Sort in revere order to be able to pop
        param_indexes.sort_by(|a, b| Ord::cmp(b, a));

        let result = match self.message.params() {
            None => (Vec::with_capacity(0), None),
            Some(params) => {
                let (mut params_iter, trailing_parsed) = params.into_parts();
                let mut filtered_params = Vec::with_capacity(param_indexes.len());
                let mut position = 0;
                for param_index in param_indexes {
                    match params_iter.nth(param_index - position) {
                        Some(next) => filtered_params.push(next),
                        None => break,
                    }
                    position = param_index;
                }

                (
                    filtered_params,
                    if trailing { trailing_parsed } else { None },
                )
            }
        };
        Ok(result)
    }
}

impl<'a> Query<'a, Init> {
    pub(crate) fn new(message: &'a Message) -> Self {
        Query {
            message,
            state: Init,
        }
    }

    pub fn tags<'b, I: IntoIterator<Item = &'a str>>(
        self,
        iter: I,
    ) -> Result<Query<'a, TagsState<'a>>, InvalidIrcFormatError> {
        Ok(Query {
            message: self.message,
            state: TagsState {
                tags: self.parse_tags(iter.into_iter().collect::<HashSet<_>>())?,
            },
        })
    }

    pub fn prefix(
        self,
        user: bool,
        host: bool,
    ) -> Result<Query<'a, PrefixState<'a>>, InvalidIrcFormatError> {
        let prefix = self.parse_prefix(user, host)?;
        Ok(Query {
            message: self.message,
            state: PrefixState {
                tags: HashMap::with_capacity(0),
                prefix,
            },
        })
    }
}

impl<'a> Query<'a, TagsState<'a>> {
    pub fn prefix(
        self,
        user: bool,
        host: bool,
    ) -> Result<Query<'a, PrefixState<'a>>, InvalidIrcFormatError> {
        let prefix = self.parse_prefix(user, host)?;
        Ok(Query {
            message: self.message,
            state: PrefixState {
                tags: self.state.tags,
                prefix,
            },
        })
    }
}

impl<'a> Query<'a, PrefixState<'a>> {
    pub fn params(
        self,
        indexes: Vec<usize>,
        trailing: bool,
    ) -> Result<Query<'a, Parsed<'a>>, InvalidIrcFormatError> {
        let (params, trailing) = self.parse_params(indexes, trailing)?;

        Ok(Query {
            message: self.message,
            state: Parsed::new(
                self.state.tags,
                self.state.prefix,
                self.message.command(),
                params,
                trailing,
            ),
        })
    }
}

impl<'a, T: State> Deref for Query<'a, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.state
    }
}

#[cfg(test)]
mod tests {
    use crate::prefix::Prefixed;
    use crate::tags::Taggable;
    use crate::Message;
    use std::error::Error;

    #[test]
    fn test_tags() -> Result<(), Box<dyn Error>> {
        let message = Message::builder("CMD").build();
        assert_eq!("CMD", message.query().command());
        let tags = message.query().tags(vec!["test"])?;
        assert_eq!(None, tags.tag("test"));

        let message = Message::builder("CMD")
            .tag("test", "value")
            .tag("test1", "value1")
            .build();
        assert_eq!("CMD", message.query().command());
        let tags = message.query().tags(vec!["test"])?;
        assert_eq!(Some("value"), tags.tag("test"));
        assert_eq!(None, tags.tag("test1"));

        let tags = message.query().tags(vec!["test", "test1"])?;
        assert_eq!(Some("value"), tags.tag("test"));
        assert_eq!(Some("value1"), tags.tag("test1"));

        let tags = message.query().tags(vec!["test", "test1", "test2"])?;
        assert_eq!(Some("value"), tags.tag("test"));
        assert_eq!(Some("value1"), tags.tag("test1"));
        assert_eq!(None, tags.tag("test2"));

        Ok(())
    }

    #[test]
    fn test_prefix() -> Result<(), Box<dyn Error>> {
        let message = Message::builder("CMD").build();
        assert_eq!("CMD", message.command());
        let prefix = message.query().prefix(false, false)?;
        assert_eq!(None, prefix.prefix());

        let message = Message::builder("CMD")
            .prefix("name", Some("user"), Some("host"))
            .build();
        assert_eq!("CMD", message.command());
        let prefix = message.query().prefix(false, false)?;
        assert_eq!(Some("name"), prefix.prefix().map(|prefix| prefix.name()));
        assert_eq!(None, prefix.prefix().and_then(|prefix| prefix.user()));
        assert_eq!(None, prefix.prefix().and_then(|prefix| prefix.host()));

        let prefix = message.query().prefix(true, false)?;
        assert_eq!(Some("name"), prefix.prefix().map(|prefix| prefix.name()));
        assert_eq!(
            Some("user"),
            prefix.prefix().and_then(|prefix| prefix.user())
        );
        assert_eq!(None, prefix.prefix().and_then(|prefix| prefix.host()));

        let prefix = message.query().prefix(false, true)?;
        assert_eq!(Some("name"), prefix.prefix().map(|prefix| prefix.name()));
        assert_eq!(None, prefix.prefix().and_then(|prefix| prefix.user()));
        assert_eq!(
            Some("host"),
            prefix.prefix().and_then(|prefix| prefix.host())
        );

        let prefix = message.query().prefix(true, true)?;
        assert_eq!(Some("name"), prefix.prefix().map(|prefix| prefix.name()));
        assert_eq!(
            Some("user"),
            prefix.prefix().and_then(|prefix| prefix.user())
        );
        assert_eq!(
            Some("host"),
            prefix.prefix().and_then(|prefix| prefix.host())
        );

        Ok(())
    }
}
