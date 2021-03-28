use crate::{InvalidIrcFormatError, Message};
use std::collections::{HashMap, HashSet};

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
    names: Option<HashSet<&'a str>>,
}

impl<'a> State for TagsState<'a> {}

type Prefix<'a> = (&'a str, Option<&'a str>, Option<&'a str>);

pub struct PrefixState<'a> {
    tags: HashMap<&'a str, &'a str>,
    prefix: Option<(bool, bool, bool)>,
}

impl<'a> State for PrefixState<'a> {}

impl<'a> Query<'a, Init> {
    pub fn new(message: &'a Message) -> Self {
        Query {
            message,
            state: Init,
        }
    }

    pub fn tags<'b, I: IntoIterator<Item = &'a str>>(self, iter: I) -> Query<'a, TagsState<'a>> {
        Query {
            message: self.message,
            state: TagsState {
                names: Some(iter.into_iter().collect::<HashSet<_>>()),
            },
        }
    }
}

impl<'a, 'b> Query<'a, TagsState<'b>> {
    fn parse_tags(&mut self) -> Result<HashMap<&'a str, &'a str>, InvalidIrcFormatError> {
        if self.state.names.is_none() {
            return Ok(HashMap::with_capacity(0));
        }

        let mut searched = self.state.names.take().unwrap();
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

    pub fn prefix(
        mut self,
        name: bool,
        user: bool,
        host: bool,
    ) -> Result<Query<'a, PrefixState<'a>>, InvalidIrcFormatError> {
        let tags = self.parse_tags()?;

        Ok(Query {
            message: self.message,
            state: PrefixState {
                tags,
                prefix: Some((name, user, host)),
            },
        })
    }
}

impl<'a> Query<'a, PrefixState<'a>> {
    fn parse_prefix(self) -> Result<Prefix<'a>, InvalidIrcFormatError> {
        unimplemented!()
    }
}
