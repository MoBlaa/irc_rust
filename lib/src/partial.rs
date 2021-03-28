use crate::parsed::ParsedPrefix;
use crate::prefix::Prefixed;
use crate::tags::Taggable;
use crate::{InvalidIrcFormatError, Message, Parsed};
use std::collections::{HashMap, HashSet};
use std::ops::Deref;

/// Implements Zero-Copy __partial__ parsing by only parsing and extracting the desired
/// parts of the message. Implements the state pattern for a clear query structure.
///
/// TODO:
///     - Parse raw String instead of Message (increase performance as currently multiple 'find' invocations happen)
///     - Tags, Params and Prefix should return temporary references to parts of the string with lifetime 'a. (&&'a str). Currently implicit Copies happen
///
/// # Note on Parameters
///
/// While the order of the filtered elements is preserved the original index will get lost.
///
/// ```rust
/// use irc_rust::{Message, Parameterized};
///
/// let message = Message::builder("CMD").param("param0").param("param1").param("param2").build();
/// let query = message.partial()
///     // Query vor "param1" only
///     .params(vec![1], false);
/// // Note that the index of "param1" has changed from 1 to 0
/// assert_eq!(Some("param1"), query.param(0));
/// ```
#[derive(Debug)]
pub struct Partial<'a, S: State> {
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

impl<'a> Taggable<'a> for PrefixState<'a> {
    fn tag(&self, key: &str) -> Option<&'a str> {
        // TODO: Remove Copy
        self.tags.get(key).copied()
    }
}

impl<'a> State for PrefixState<'a> {}

pub struct CommandState<'a> {
    tags: HashMap<&'a str, &'a str>,
    prefix: Option<ParsedPrefix<'a>>,
    command: Option<&'a str>,
}

impl<'a> CommandState<'a> {
    pub fn prefix(&self) -> Option<&ParsedPrefix<'a>> {
        self.prefix.as_ref()
    }

    pub fn command(&self) -> Option<&'a str> {
        self.command
    }
}

impl<'a> Taggable<'a> for CommandState<'a> {
    fn tag(&self, key: &str) -> Option<&'a str> {
        // TODO: Remove Copy
        self.tags.get(key).copied()
    }
}

impl<'a> State for CommandState<'a> {}

impl<'a> State for Parsed<'a> {}

impl<'a, T: State> Partial<'a, T> {
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
        let prefix = self.message.prefix()?.map(|prefix| {
            ParsedPrefix(
                prefix.name(),
                if user { prefix.user() } else { None },
                if host { prefix.host() } else { None },
            )
        });
        Ok(prefix)
    }

    fn parse_params(
        &self,
        mut param_indexes: Vec<usize>,
        trailing: bool,
    ) -> (Vec<&'a str>, Option<&'a str>) {
        param_indexes.dedup();
        param_indexes.sort_unstable();

        match self.message.params() {
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
        }
    }
}

impl<'a> Partial<'a, Init> {
    pub(crate) fn new(message: &'a Message) -> Self {
        Partial {
            message,
            state: Init,
        }
    }

    pub fn tags<'b, I: IntoIterator<Item = &'a str>>(
        self,
        iter: I,
    ) -> Result<Partial<'a, TagsState<'a>>, InvalidIrcFormatError> {
        Ok(Partial {
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
    ) -> Result<Partial<'a, PrefixState<'a>>, InvalidIrcFormatError> {
        let prefix = self.parse_prefix(user, host)?;
        Ok(Partial {
            message: self.message,
            state: PrefixState {
                tags: HashMap::with_capacity(0),
                prefix,
            },
        })
    }

    pub fn command(self) -> Partial<'a, CommandState<'a>> {
        let command = self.message.command();
        Partial {
            message: self.message,
            state: CommandState {
                tags: HashMap::new(),
                prefix: None,
                command: Some(command),
            },
        }
    }

    pub fn params(self, indexes: Vec<usize>, trailing: bool) -> Partial<'a, Parsed<'a>> {
        let (params, trailing) = self.parse_params(indexes, trailing);

        Partial {
            message: self.message,
            state: Parsed::new(HashMap::new(), None, None, params, trailing),
        }
    }
}

impl<'a> Partial<'a, TagsState<'a>> {
    pub fn prefix(
        self,
        user: bool,
        host: bool,
    ) -> Result<Partial<'a, PrefixState<'a>>, InvalidIrcFormatError> {
        let prefix = self.parse_prefix(user, host)?;
        Ok(Partial {
            message: self.message,
            state: PrefixState {
                tags: self.state.tags,
                prefix,
            },
        })
    }

    pub fn command(self) -> Partial<'a, CommandState<'a>> {
        let command = self.message.command();
        Partial {
            message: self.message,
            state: CommandState {
                tags: self.state.tags,
                prefix: None,
                command: Some(command),
            },
        }
    }

    pub fn params(self, indexes: Vec<usize>, trailing: bool) -> Partial<'a, Parsed<'a>> {
        let (params, trailing) = self.parse_params(indexes, trailing);

        Partial {
            message: self.message,
            state: Parsed::new(self.state.tags, None, None, params, trailing),
        }
    }
}

impl<'a> Partial<'a, PrefixState<'a>> {
    pub fn command(self) -> Partial<'a, CommandState<'a>> {
        let command = self.message.command();
        Partial {
            message: self.message,
            state: CommandState {
                tags: self.state.tags,
                prefix: self.state.prefix,
                command: Some(command),
            },
        }
    }

    pub fn params(self, indexes: Vec<usize>, trailing: bool) -> Partial<'a, Parsed<'a>> {
        let (params, trailing) = self.parse_params(indexes, trailing);

        Partial {
            message: self.message,
            state: Parsed::new(self.state.tags, self.state.prefix, None, params, trailing),
        }
    }
}

impl<'a> Partial<'a, CommandState<'a>> {
    pub fn params(self, indexes: Vec<usize>, trailing: bool) -> Partial<'a, Parsed<'a>> {
        let (params, trailing) = self.parse_params(indexes, trailing);

        Partial {
            message: self.message,
            state: Parsed::new(
                self.state.tags,
                self.state.prefix,
                self.state.command,
                params,
                trailing,
            ),
        }
    }
}

impl<'a, T: State> Deref for Partial<'a, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.state
    }
}

#[cfg(test)]
mod tests {
    use crate::params::Parameterized;
    use crate::prefix::Prefixed;
    use crate::tags::Taggable;
    use crate::Message;
    use std::error::Error;

    #[test]
    fn test_overall() -> Result<(), Box<dyn Error>> {
        let message = Message::builder("CMD")
            .tag("tag1", "value1")
            .tag("tag2", "value2")
            .prefix("name", Some("user"), Some("host"))
            .param("param0")
            .param("param1")
            .trailing("Trailing parameter!")
            .build();
        let query = message
            .partial()
            .tags(vec!["tag1"])?
            .prefix(true, false)?
            .command()
            .params(vec![1], true);
        assert_eq!(Some("CMD"), query.command());
        assert_eq!(Some("value1"), query.tag("tag1"));
        assert_eq!(None, query.tag("tag2"));
        assert_eq!(
            Some(("name", Some("user"), None)),
            query.prefix().map(|prefix| prefix.as_parts())
        );
        assert_eq!(Some("param1"), query.param(0));
        assert_eq!(None, query.param(1));
        assert_eq!(Some("Trailing parameter!"), query.trailing());

        Ok(())
    }

    #[test]
    fn test_tags() -> Result<(), Box<dyn Error>> {
        let message = Message::builder("CMD").build();
        assert_eq!(Some("CMD"), message.partial().command().command());
        let tags = message.partial().tags(vec!["test"])?;
        assert_eq!(None, tags.tag("test"));

        let message = Message::builder("CMD")
            .tag("test", "value")
            .tag("test1", "value1")
            .build();
        assert_eq!(Some("CMD"), message.partial().command().command());
        let tags = message.partial().tags(vec!["test"])?;
        assert_eq!(Some("value"), tags.tag("test"));
        assert_eq!(None, tags.tag("test1"));

        let tags = message.partial().tags(vec!["test", "test1"])?;
        assert_eq!(Some("value"), tags.tag("test"));
        assert_eq!(Some("value1"), tags.tag("test1"));

        let tags = message.partial().tags(vec!["test", "test1", "test2"])?;
        assert_eq!(Some("value"), tags.tag("test"));
        assert_eq!(Some("value1"), tags.tag("test1"));
        assert_eq!(None, tags.tag("test2"));

        Ok(())
    }

    #[test]
    fn test_prefix() -> Result<(), Box<dyn Error>> {
        let message = Message::builder("CMD").build();
        assert_eq!("CMD", message.command());
        let prefix = message.partial().prefix(false, false)?;
        assert_eq!(None, prefix.prefix());

        let message = Message::builder("CMD")
            .prefix("name", Some("user"), Some("host"))
            .build();
        assert_eq!("CMD", message.command());
        let prefix = message.partial().prefix(false, false)?;
        assert_eq!(Some("name"), prefix.prefix().map(|prefix| prefix.name()));
        assert_eq!(None, prefix.prefix().and_then(|prefix| prefix.user()));
        assert_eq!(None, prefix.prefix().and_then(|prefix| prefix.host()));

        let prefix = message.partial().prefix(true, false)?;
        assert_eq!(Some("name"), prefix.prefix().map(|prefix| prefix.name()));
        assert_eq!(
            Some("user"),
            prefix.prefix().and_then(|prefix| prefix.user())
        );
        assert_eq!(None, prefix.prefix().and_then(|prefix| prefix.host()));

        let prefix = message.partial().prefix(false, true)?;
        assert_eq!(Some("name"), prefix.prefix().map(|prefix| prefix.name()));
        assert_eq!(None, prefix.prefix().and_then(|prefix| prefix.user()));
        assert_eq!(
            Some("host"),
            prefix.prefix().and_then(|prefix| prefix.host())
        );

        let prefix = message.partial().prefix(true, true)?;
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
