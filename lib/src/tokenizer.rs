use crate::parsed::ParsedPrefix;
use crate::Parsed;
use std::collections::{HashMap, HashSet};
use std::error::Error;
use std::fmt::Debug;
use std::marker::PhantomData;

#[derive(Eq, PartialEq, Debug)]
pub struct Tokenizer<'a, T: State> {
    raw: &'a str,
    state: PhantomData<T>,
}

pub trait State: PartialEq + Eq + Debug {}

#[derive(Eq, PartialEq, Debug)]
pub struct Start;

impl State for Start {}

#[derive(Eq, PartialEq, Debug)]
pub struct TagsState;

impl State for TagsState {}

#[derive(Eq, PartialEq, Debug)]
pub struct PrefixState;

impl State for PrefixState {}

#[derive(Eq, PartialEq, Debug)]
pub struct CommandState;

impl State for CommandState {}

#[derive(Eq, PartialEq, Debug)]
pub struct ParamsState;

impl State for ParamsState {}

#[derive(Eq, PartialEq, Debug)]
pub struct TrailingState;

impl State for TrailingState {}

impl<'a, S: State> Tokenizer<'a, S> {
    fn skip_until_char(&mut self, ch: char) {
        if self.raw.starts_with(ch) {
            return;
        }

        let end = self
            .raw
            .find(ch)
            .map(|space_pos| space_pos + 1)
            .unwrap_or_else(|| self.raw.len());
        self.raw = &self.raw[end..];
    }

    fn skip_until_str(&mut self, s: &str) {
        if self.raw.starts_with(s) {
            return;
        }

        let end = self.raw.find(s).unwrap_or_else(|| self.raw.len());
        self.raw = &self.raw[end..];
    }

    fn skip_tags(&mut self) {
        // include ';' to also skip if tags have been partially parsed
        if self.raw.starts_with(&['@', ';'][..]) {
            self.skip_until_char(' ');
        }
    }

    fn skip_prefix(&mut self) {
        self.skip_tags();
        if self.raw.starts_with(&[':', '!', '@'][..]) {
            self.skip_until_char(' ');
        }
    }

    fn skip_command(&mut self) {
        self.skip_prefix();
        self.skip_until_char(' ');
    }

    fn skip_params(&mut self) {
        self.skip_command();
        self.skip_until_str(" :");
    }
}

impl<'a> Tokenizer<'a, Start> {
    pub fn new(raw: &'a str) -> Result<Self, ParserError> {
        if raw.is_empty() {
            Err(ParserError::NoCommand)
        } else {
            Ok(Tokenizer {
                raw,
                state: PhantomData::default(),
            })
        }
    }

    pub fn into_parsed(self, mut cfg: PartialCfg<'a>) -> Result<Parsed<'a>, ParserError> {
        let mut result = Parsed::default();

        // Parse tags
        let mut tokenizer = self.tags();
        if !cfg.tags.is_empty() {
            let mut tags = HashMap::with_capacity(cfg.tags.len());
            let mut iter = tokenizer.as_iter();
            while !cfg.tags.is_empty() {
                match iter.next() {
                    Some(Ok((key, val))) => {
                        if cfg.tags.remove(&key) {
                            tags.insert(key, val);
                        }
                    }
                    Some(Err(why)) => return Err(why),
                    None => break,
                }
            }
            result.tags = tags;
        }

        // Parse prefix
        let mut tokenizer = tokenizer.prefix();
        if let Some((name, user, host)) = cfg.prefix {
            result.prefix = Some(ParsedPrefix(
                if name { tokenizer.name()? } else { None },
                if user { tokenizer.user()? } else { None },
                if host { tokenizer.name()? } else { None },
            ));
        }

        // Command
        let mut tokenizer = tokenizer.command();
        if cfg.command {
            result.command = Some(tokenizer.command()?);
        }

        // Params
        let mut tokenizer = tokenizer.params();
        if !cfg.params.is_empty() {
            let mut params = Vec::with_capacity(cfg.params.len());
            let mut iter = tokenizer.as_iter();
            cfg.params.dedup();
            cfg.params.sort_unstable();
            let mut position = 0;
            for index in cfg.params {
                let delta = index - position;
                position = index;
                params.push(iter.nth(delta));
            }
            result.params = params;
        }

        // Trailing
        let tokenizer = tokenizer.trailing();
        if cfg.trailing {
            result.trailing = tokenizer.trailing();
        }

        Ok(result)
    }

    pub fn tags(self) -> Tokenizer<'a, TagsState> {
        Tokenizer {
            raw: self.raw,
            state: PhantomData::default(),
        }
    }

    pub fn prefix(mut self) -> Tokenizer<'a, PrefixState> {
        self.skip_tags();
        Tokenizer {
            raw: self.raw,
            state: PhantomData::default(),
        }
    }

    pub fn command(mut self) -> Tokenizer<'a, CommandState> {
        self.skip_prefix();
        Tokenizer {
            raw: self.raw,
            state: PhantomData::default(),
        }
    }

    pub fn params(mut self) -> Tokenizer<'a, ParamsState> {
        self.skip_command();
        Tokenizer {
            raw: self.raw,
            state: PhantomData::default(),
        }
    }

    pub fn trailing(mut self) -> Tokenizer<'a, TrailingState> {
        self.skip_params();
        Tokenizer {
            raw: self.raw,
            state: PhantomData::default(),
        }
    }
}

impl<'a> Tokenizer<'a, TagsState> {
    pub fn as_iter<'b>(&'b mut self) -> IntoTagsIter<'b, 'a> {
        IntoTagsIter(self)
    }

    pub fn prefix(mut self) -> Tokenizer<'a, PrefixState> {
        self.skip_tags();
        Tokenizer {
            raw: self.raw,
            state: PhantomData::default(),
        }
    }

    pub fn command(mut self) -> Tokenizer<'a, CommandState> {
        self.skip_prefix();
        Tokenizer {
            raw: self.raw,
            state: PhantomData::default(),
        }
    }

    pub fn params(mut self) -> Tokenizer<'a, ParamsState> {
        self.skip_command();
        Tokenizer {
            raw: self.raw,
            state: PhantomData::default(),
        }
    }

    pub fn trailing(mut self) -> Tokenizer<'a, TrailingState> {
        self.skip_params();
        Tokenizer {
            raw: self.raw,
            state: PhantomData::default(),
        }
    }
}

pub type Prefix<'a> = (&'a str, Option<&'a str>, Option<&'a str>);

impl<'a> Tokenizer<'a, PrefixState> {
    pub fn name(&mut self) -> Result<Option<&'a str>, ParserError> {
        if self.raw.starts_with(' ') {
            self.raw = &self.raw[1..];
        }
        let mut name = None;
        if self.raw.starts_with(':') {
            let end = self
                .raw
                .find(&['!', '@', ' '][..])
                .ok_or(ParserError::NoCommand)?;
            let split = self.raw.split_at(end);
            name = Some(&split.0[1..]);
            self.raw = split.1;
        }
        Ok(name)
    }

    pub fn user(&mut self) -> Result<Option<&'a str>, ParserError> {
        let mut user = None;
        if self.raw.starts_with('!') {
            let end = self
                .raw
                .find('@')
                .ok_or(ParserError::PrefixUserWithoutHost)?;
            let split = self.raw.split_at(end);
            user = Some(&split.0[1..]);
            self.raw = split.1;
        }
        Ok(user)
    }

    pub fn host(&mut self) -> Result<Option<&'a str>, ParserError> {
        let mut host = None;
        if self.raw.starts_with('@') {
            let end = self.raw.find(' ').ok_or(ParserError::NoCommand)?;
            let split = self.raw.split_at(end);
            host = Some(&split.0[1..]);
            self.raw = split.1;
        }
        Ok(host)
    }

    /// Returns [None] if the prefix is badly formatted or no prefix is present.
    pub fn parts(&mut self) -> Result<Option<Prefix<'a>>, ParserError> {
        if let Some(raw) = self.raw.strip_prefix(' ') {
            self.raw = raw;
        }
        if !self.raw.starts_with(':') {
            return Ok(None);
        }
        let (name, user, host) = (self.name()?, self.user()?, self.host()?);
        if name.is_none() && (user.is_some() || host.is_some()) {
            Err(ParserError::PrefixWithoutName)
        } else {
            Ok(Some((name.unwrap(), user, host)))
        }
    }

    pub fn command(mut self) -> Tokenizer<'a, CommandState> {
        self.skip_prefix();
        Tokenizer {
            raw: self.raw,
            state: PhantomData::default(),
        }
    }

    pub fn params(mut self) -> Tokenizer<'a, ParamsState> {
        self.skip_command();
        Tokenizer {
            raw: self.raw,
            state: PhantomData::default(),
        }
    }

    pub fn trailing(mut self) -> Tokenizer<'a, TrailingState> {
        self.skip_params();
        Tokenizer {
            raw: self.raw,
            state: PhantomData::default(),
        }
    }
}

impl<'a> Tokenizer<'a, CommandState> {
    pub fn command(&mut self) -> Result<&'a str, ParserError> {
        if self.raw.starts_with(' ') {
            self.raw = &self.raw[1..];
        }

        let end = self.raw.find(' ').unwrap_or_else(|| self.raw.len());
        let (command, rest) = self.raw.split_at(end);
        if command.is_empty() {
            return Err(ParserError::NoCommand);
        }
        self.raw = rest;
        Ok(command)
    }

    pub fn params(mut self) -> Tokenizer<'a, ParamsState> {
        self.skip_command();
        Tokenizer {
            raw: self.raw,
            state: PhantomData::default(),
        }
    }

    pub fn trailing(mut self) -> Tokenizer<'a, TrailingState> {
        self.skip_params();
        Tokenizer {
            raw: self.raw,
            state: PhantomData::default(),
        }
    }
}

impl<'a> Tokenizer<'a, ParamsState> {
    pub fn trailing(mut self) -> Tokenizer<'a, TrailingState> {
        self.skip_params();
        Tokenizer {
            raw: self.raw,
            state: PhantomData::default(),
        }
    }

    pub fn as_iter<'b>(&'b mut self) -> IntoParamsIter<'b, 'a> {
        IntoParamsIter(self)
    }
}

pub struct IntoParamsIter<'a, 'b>(&'a mut Tokenizer<'b, ParamsState>);

impl<'a, 'b> Iterator for IntoParamsIter<'b, 'a> {
    type Item = &'a str;

    fn next(&mut self) -> Option<Self::Item> {
        if !self.0.raw.starts_with(' ') || self.0.raw.starts_with(" :") {
            return None;
        }
        self.0.raw = &self.0.raw[1..];
        let end = self
            .0
            .raw
            .find(' ')
            .or_else(|| self.0.raw.find(" :"))
            .unwrap_or_else(|| self.0.raw.len());
        let (param, rest) = self.0.raw.split_at(end);
        self.0.raw = rest;
        Some(param)
    }
}

impl<'a> Tokenizer<'a, TrailingState> {
    pub fn trailing(&self) -> Option<&'a str> {
        if self.raw.starts_with(" :") {
            Some(&self.raw[2..])
        } else {
            None
        }
    }
}

pub struct IntoTagsIter<'a, 'b>(&'a mut Tokenizer<'b, TagsState>);

impl<'a, 'b> Iterator for IntoTagsIter<'b, 'a> {
    type Item = Result<(&'a str, &'a str), ParserError>;

    fn next(&mut self) -> Option<Self::Item> {
        match &self.0.raw[..1] {
            "@" | ";" => {
                let key_start = 1;
                let key_end = self.0.raw[key_start..].find(&['='][..]);
                if key_end.is_none() {
                    // Skip to next entry
                    self.0.raw = &self.0.raw[1..];
                    let end = self
                        .0
                        .raw
                        .find(&[' ', ';'][..])
                        .unwrap_or_else(|| self.0.raw.len());
                    self.0.raw = &self.0.raw[end..];
                    return Some(Err(ParserError::NoTagKeyEnd));
                }
                let key_end = key_start + key_end.unwrap();
                let val_start = key_end + 1;
                let val_end = self.0.raw[val_start..].find(&[';', ' '][..]);
                if val_end.is_none() {
                    // Skip till the end as only tags seem to be present
                    self.0.raw = &self.0.raw[self.0.raw.len()..];
                    return Some(Err(ParserError::NoTagValueEnd));
                }
                let val_end = val_start + val_end.unwrap();
                let key_val = (
                    &self.0.raw[key_start..key_end],
                    &self.0.raw[val_start..val_end],
                );
                self.0.raw = &self.0.raw[val_end..];
                Some(Ok(key_val))
            }
            _ => None,
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
pub enum ParserError {
    NoTagKeyEnd,
    NoTagValueEnd,
    NoCommand,
    PrefixWithoutName,
    PrefixUserWithoutHost,
}

impl std::fmt::Display for ParserError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ParserError::NoTagKeyEnd => write!(f, "Tag Key has no ending '='"),
            ParserError::NoTagValueEnd => write!(f, "Tag Value has no ending ';' or ' '"),
            ParserError::NoCommand => write!(f, "Missing command in message"),
            ParserError::PrefixWithoutName => write!(f, "Prefix has to have name included"),
            ParserError::PrefixUserWithoutHost => {
                write!(f, "Prefix user is not allowed without host")
            }
        }
    }
}

impl Error for ParserError {}

#[derive(Default, Clone)]
pub struct PartialCfg<'a> {
    pub tags: HashSet<&'a str>,
    pub prefix: Option<(bool, bool, bool)>,
    pub command: bool,
    pub params: Vec<usize>,
    pub trailing: bool,
}

#[cfg(test)]
mod tests {
    use crate::tokenizer::{ParserError, Tokenizer};
    use std::error::Error;

    #[test]
    fn test_empty() {
        assert_eq!(Err(ParserError::NoCommand), Tokenizer::new(""));
    }

    #[test]
    fn test_command_only() -> Result<(), Box<dyn Error>> {
        let mut tokenizer = Tokenizer::new("CMD")?.tags();
        assert_eq!(None, tokenizer.as_iter().next());
        let mut tokenizer = tokenizer.prefix();
        assert_eq!(None, tokenizer.parts()?);
        let mut tokenizer = tokenizer.command();
        assert_eq!("CMD", tokenizer.command()?);
        let mut tokenizer = tokenizer.params();
        assert_eq!(None, tokenizer.as_iter().next());
        assert_eq!(None, tokenizer.trailing().trailing());

        Ok(())
    }

    #[test]
    fn test_tag() -> Result<(), Box<dyn Error>> {
        let mut tokenizer = Tokenizer::new("@key1=value1 CMD")?.tags();
        let mut iter = tokenizer.as_iter();
        assert_eq!(Some(Ok(("key1", "value1"))), iter.next());
        assert_eq!(None, iter.next());
        let mut tokenizer = tokenizer.prefix();
        assert_eq!(None, tokenizer.parts()?);
        let mut tokenizer = tokenizer.command();
        assert_eq!("CMD", tokenizer.command()?);
        let mut tokenizer = tokenizer.params();
        let mut iter = tokenizer.as_iter();
        assert_eq!(None, iter.next());
        assert_eq!(None, tokenizer.trailing().trailing());

        Ok(())
    }

    #[test]
    fn test_tags() -> Result<(), Box<dyn Error>> {
        let mut tokenizer = Tokenizer::new("@key1=value1;key2=value2 CMD")?.tags();
        let mut iter = tokenizer.as_iter();
        assert_eq!(Some(Ok(("key1", "value1"))), iter.next());
        assert_eq!(Some(Ok(("key2", "value2"))), iter.next());
        assert_eq!(None, iter.next());
        let mut tokenizer = tokenizer.prefix();
        assert_eq!(None, tokenizer.parts()?);
        let mut tokenizer = tokenizer.command();
        assert_eq!("CMD", tokenizer.command()?);
        let mut tokenizer = tokenizer.params();
        let mut iter = tokenizer.as_iter();
        assert_eq!(None, iter.next());
        assert_eq!(None, tokenizer.trailing().trailing());

        Ok(())
    }

    #[test]
    fn test_prefix() -> Result<(), Box<dyn Error>> {
        let mut tokenizer = Tokenizer::new(":name!user@host CMD")?.tags();
        let mut iter = tokenizer.as_iter();
        assert_eq!(None, iter.next());
        let mut tokenizer = tokenizer.prefix();
        assert_eq!(
            Some(("name", Some("user"), Some("host"))),
            tokenizer.parts()?
        );
        let mut tokenizer = tokenizer.command();
        assert_eq!("CMD", tokenizer.command()?);
        let mut tokenizer = tokenizer.params();
        let mut iter = tokenizer.as_iter();
        assert_eq!(None, iter.next());
        assert_eq!(None, tokenizer.trailing().trailing());

        Ok(())
    }

    #[test]
    fn test_prefix_name() -> Result<(), Box<dyn Error>> {
        let mut tokenizer = Tokenizer::new(":name CMD")?.tags();
        let mut iter = tokenizer.as_iter();
        assert_eq!(None, iter.next());
        let mut tokenizer = tokenizer.prefix();
        assert_eq!(Some(("name", None, None)), tokenizer.parts()?);
        let mut tokenizer = tokenizer.command();
        assert_eq!("CMD", tokenizer.command()?);
        let mut tokenizer = tokenizer.params();
        let mut iter = tokenizer.as_iter();
        assert_eq!(None, iter.next());
        assert_eq!(None, tokenizer.trailing().trailing());

        Ok(())
    }

    #[test]
    fn test_prefix_user() -> Result<(), Box<dyn Error>> {
        let mut tokenizer = Tokenizer::new(":name!user CMD")?.tags();
        let mut iter = tokenizer.as_iter();
        assert_eq!(None, iter.next());
        let mut tokenizer = tokenizer.prefix();
        assert_eq!(Err(ParserError::PrefixUserWithoutHost), tokenizer.parts());

        Ok(())
    }

    #[test]
    fn test_prefix_host() -> Result<(), Box<dyn Error>> {
        let mut tokenizer = Tokenizer::new(":name@host CMD")?.tags();
        let mut iter = tokenizer.as_iter();
        assert_eq!(None, iter.next());
        let mut tokenizer = tokenizer.prefix();
        assert_eq!(Some(("name", None, Some("host"))), tokenizer.parts()?);
        let mut tokenizer = tokenizer.command();
        assert_eq!("CMD", tokenizer.command()?);
        let mut tokenizer = tokenizer.params();
        let mut iter = tokenizer.as_iter();
        assert_eq!(None, iter.next());
        assert_eq!(None, tokenizer.trailing().trailing());

        Ok(())
    }

    #[test]
    fn test_params() -> Result<(), Box<dyn Error>> {
        let mut tokenizer = Tokenizer::new("CMD param0 param1")?.tags();
        let mut iter = tokenizer.as_iter();
        assert_eq!(None, iter.next());
        let mut tokenizer = tokenizer.prefix();
        assert_eq!(None, tokenizer.parts()?);
        let mut tokenizer = tokenizer.command();
        assert_eq!("CMD", tokenizer.command()?);
        let mut tokenizer = tokenizer.params();
        let mut iter = tokenizer.as_iter();
        assert_eq!(Some("param0"), iter.next());
        assert_eq!(Some("param1"), iter.next());
        assert_eq!(None, iter.next());
        assert_eq!(None, tokenizer.trailing().trailing());

        Ok(())
    }

    #[test]
    fn test_params_trailing() -> Result<(), Box<dyn Error>> {
        let mut tokenizer = Tokenizer::new("CMD param0 param1 :Trailing parameter!")?.tags();
        let mut iter = tokenizer.as_iter();
        assert_eq!(None, iter.next());
        let mut tokenizer = tokenizer.prefix();
        assert_eq!(None, tokenizer.parts()?);
        let mut tokenizer = tokenizer.command();
        assert_eq!("CMD", tokenizer.command()?);
        let mut tokenizer = tokenizer.params();
        let mut iter = tokenizer.as_iter();
        assert_eq!(Some("param0"), iter.next());
        assert_eq!(Some("param1"), iter.next());
        assert_eq!(None, iter.next());
        assert_eq!(Some("Trailing parameter!"), tokenizer.trailing().trailing());

        Ok(())
    }

    #[test]
    fn test_trailing() -> Result<(), Box<dyn Error>> {
        let mut tokenizer = Tokenizer::new("CMD :Trailing parameter!")?.tags();
        let mut iter = tokenizer.as_iter();
        assert_eq!(None, iter.next());
        let mut tokenizer = tokenizer.prefix();
        assert_eq!(None, tokenizer.parts()?);
        let mut tokenizer = tokenizer.command();
        assert_eq!("CMD", tokenizer.command()?);
        let mut tokenizer = tokenizer.params();
        let mut iter = tokenizer.as_iter();
        assert_eq!(None, iter.next());
        assert_eq!(Some("Trailing parameter!"), tokenizer.trailing().trailing());

        Ok(())
    }

    #[test]
    fn test_all() -> Result<(), Box<dyn Error>> {
        let mut tokenizer = Tokenizer::new(
            "@key1=value1;key2=value2 :name!user@host CMD param0 param1 :Trailing parameter!@:=;",
        )?
        .tags();
        let mut iter = tokenizer.as_iter();
        assert_eq!(Some(Ok(("key1", "value1"))), iter.next());
        assert_eq!(Some(Ok(("key2", "value2"))), iter.next());
        let mut tokenizer = tokenizer.prefix();
        assert_eq!(
            Some(("name", Some("user"), Some("host"))),
            tokenizer.parts()?
        );
        let mut tokenizer = tokenizer.command();
        assert_eq!("CMD", tokenizer.command()?);
        let mut tokenizer = tokenizer.params();
        let mut iter = tokenizer.as_iter();
        assert_eq!(Some("param0"), iter.next());
        assert_eq!(Some("param1"), iter.next());
        assert_eq!(
            Some("Trailing parameter!@:=;"),
            tokenizer.trailing().trailing()
        );

        Ok(())
    }

    #[test]
    fn test_only_trailing() -> Result<(), Box<dyn Error>> {
        let tokenizer = Tokenizer::new(
            "@key1=value1;key2=value2 :name!user@host CMD param0 param1 :Trailing parameter!@:=;",
        )?;
        assert_eq!(
            Some("Trailing parameter!@:=;"),
            tokenizer.trailing().trailing()
        );

        Ok(())
    }
}
