use std::marker::PhantomData;

pub struct Tokenizer<'a, T: State> {
    raw: &'a str,
    state: PhantomData<T>,
}

pub trait State {}

pub struct Start;

impl State for Start {}

pub struct Tags;

impl State for Tags {}

pub struct Prefix;

impl State for Prefix {}

pub struct Command;

impl State for Command {}

pub struct Params;

impl State for Params {}

pub struct Trailing;

impl State for Trailing {}

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

        let end = self
            .raw
            .find(s)
            .map(|space_pos| space_pos + 1)
            .unwrap_or_else(|| self.raw.len());
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
    pub fn new(raw: &'a str) -> Self {
        Tokenizer {
            raw,
            state: PhantomData::default(),
        }
    }

    pub fn tags(self) -> Tokenizer<'a, Tags> {
        Tokenizer {
            raw: self.raw,
            state: PhantomData::default(),
        }
    }

    pub fn prefix(mut self) -> Tokenizer<'a, Prefix> {
        self.skip_tags();
        Tokenizer {
            raw: self.raw,
            state: PhantomData::default(),
        }
    }

    pub fn command(mut self) -> Tokenizer<'a, Command> {
        self.skip_prefix();
        Tokenizer {
            raw: self.raw,
            state: PhantomData::default(),
        }
    }

    pub fn params(mut self) -> Tokenizer<'a, Params> {
        self.skip_command();
        Tokenizer {
            raw: self.raw,
            state: PhantomData::default(),
        }
    }

    pub fn trailing(mut self) -> Tokenizer<'a, Trailing> {
        self.skip_params();
        Tokenizer {
            raw: self.raw,
            state: PhantomData::default(),
        }
    }
}

impl<'a> Tokenizer<'a, Tags> {
    pub fn prefix(mut self) -> Tokenizer<'a, Prefix> {
        self.skip_tags();
        Tokenizer {
            raw: self.raw,
            state: PhantomData::default(),
        }
    }

    pub fn command(mut self) -> Tokenizer<'a, Command> {
        self.skip_prefix();
        Tokenizer {
            raw: self.raw,
            state: PhantomData::default(),
        }
    }

    pub fn params(mut self) -> Tokenizer<'a, Params> {
        self.skip_command();
        Tokenizer {
            raw: self.raw,
            state: PhantomData::default(),
        }
    }

    pub fn trailing(mut self) -> Tokenizer<'a, Trailing> {
        self.skip_params();
        Tokenizer {
            raw: self.raw,
            state: PhantomData::default(),
        }
    }
}

impl<'a> Tokenizer<'a, Prefix> {
    /// Returns [None] if the prefix is badly formatted or no prefix is present.
    pub fn parts(&mut self) -> (Option<&'a str>, Option<&'a str>, Option<&'a str>) {
        if self.raw.starts_with(" ") {
            self.raw = &self.raw[1..];
        }
        let mut name = None;
        let mut user = None;
        let mut host = None;

        if self.raw.starts_with(':') {
            let end = self
                .raw
                .find(&['!', '@', ' '][..])
                .unwrap_or_else(|| self.raw.len());
            let split = self.raw.split_at(end);
            name = Some(&split.0[1..]);
            self.raw = split.1;
        }
        if self.raw.starts_with('!') {
            let end = self
                .raw
                .find(&['@', ' '][..])
                .unwrap_or_else(|| self.raw.len());
            let split = self.raw.split_at(end);
            user = Some(&split.0[1..]);
            self.raw = split.1;
        }
        if self.raw.starts_with('@') {
            let end = self.raw.find(' ').unwrap_or_else(|| self.raw.len());
            let split = self.raw.split_at(end);
            host = Some(&split.0[1..]);
            self.raw = split.1;
        }

        (name, user, host)
    }

    pub fn command(mut self) -> Tokenizer<'a, Command> {
        self.skip_prefix();
        Tokenizer {
            raw: self.raw,
            state: PhantomData::default(),
        }
    }

    pub fn params(mut self) -> Tokenizer<'a, Params> {
        self.skip_command();
        Tokenizer {
            raw: self.raw,
            state: PhantomData::default(),
        }
    }

    pub fn trailing(mut self) -> Tokenizer<'a, Trailing> {
        self.skip_params();
        Tokenizer {
            raw: self.raw,
            state: PhantomData::default(),
        }
    }
}

impl<'a> Tokenizer<'a, Command> {
    pub fn command(&mut self) -> Option<&'a str> {
        if self.raw.starts_with(' ') {
            self.raw = &self.raw[1..];
        }

        let end = self.raw.find(' ').unwrap_or_else(|| self.raw.len());
        let (command, rest) = self.raw.split_at(end);
        if command.is_empty() {
            return None;
        }
        self.raw = rest;
        Some(command)
    }

    pub fn params(mut self) -> Tokenizer<'a, Params> {
        self.skip_command();
        Tokenizer {
            raw: self.raw,
            state: PhantomData::default(),
        }
    }

    pub fn trailing(mut self) -> Tokenizer<'a, Trailing> {
        self.skip_params();
        Tokenizer {
            raw: self.raw,
            state: PhantomData::default(),
        }
    }
}

impl<'a> Tokenizer<'a, Params> {
    pub fn trailing(mut self) -> Tokenizer<'a, Trailing> {
        self.skip_params();
        Tokenizer {
            raw: self.raw,
            state: PhantomData::default(),
        }
    }
}

impl<'a> Iterator for Tokenizer<'a, Params> {
    type Item = &'a str;

    fn next(&mut self) -> Option<Self::Item> {
        if !self.raw.starts_with(' ') || self.raw.starts_with(" :") {
            return None;
        }
        self.raw = &self.raw[1..];
        let end = self
            .raw
            .find(' ')
            .or_else(|| self.raw.find(" :"))
            .unwrap_or_else(|| self.raw.len());
        let (param, rest) = self.raw.split_at(end);
        self.raw = rest;
        Some(param)
    }
}

impl<'a> Tokenizer<'a, Trailing> {
    pub fn trailing(&self) -> Option<&'a str> {
        if self.raw.starts_with(" :") {
            Some(&self.raw[2..])
        } else {
            None
        }
    }
}

impl<'a> Iterator for Tokenizer<'a, Tags> {
    type Item = (&'a str, &'a str);

    fn next(&mut self) -> Option<Self::Item> {
        if self.raw.is_empty() {
            return None;
        }
        match &self.raw[..1] {
            "@" | ";" => {
                let key_start = 1;
                let key_end = key_start + self.raw[key_start..].find(&['='][..])?;
                let val_start = key_end + 1;
                let val_end = val_start + self.raw[val_start..].find(&[';', ' '][..])?;
                let key_val = (&self.raw[key_start..key_end], &self.raw[val_start..val_end]);
                self.raw = &self.raw[val_end..];
                Some(key_val)
            }
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::tokenizer::Tokenizer;

    #[test]
    fn test_empty() {
        let mut tokenizer = Tokenizer::new("").tags();
        assert_eq!(None, tokenizer.next());
        let mut tokenizer = tokenizer.prefix();
        assert_eq!((None, None, None), tokenizer.parts());
        let mut tokenizer = tokenizer.command();
        assert_eq!(None, tokenizer.command());
        let mut tokenizer = tokenizer.params();
        assert_eq!(None, tokenizer.next());
        assert_eq!(None, tokenizer.trailing().trailing())
    }

    #[test]
    fn test_command_only() {
        let mut tokenizer = Tokenizer::new("CMD").tags();
        assert_eq!(None, tokenizer.next());
        let mut tokenizer = tokenizer.prefix();
        assert_eq!((None, None, None), tokenizer.parts());
        let mut tokenizer = tokenizer.command();
        assert_eq!(Some("CMD"), tokenizer.command());
        let mut tokenizer = tokenizer.params();
        assert_eq!(None, tokenizer.next());
        assert_eq!(None, tokenizer.trailing().trailing())
    }

    #[test]
    fn test_tag() {
        let mut tokenizer = Tokenizer::new("@key1=value1 CMD").tags();
        assert_eq!(Some(("key1", "value1")), tokenizer.next());
        assert_eq!(None, tokenizer.next());
        let mut tokenizer = tokenizer.prefix();
        assert_eq!((None, None, None), tokenizer.parts());
        let mut tokenizer = tokenizer.command();
        assert_eq!(Some("CMD"), tokenizer.command());
        let mut tokenizer = tokenizer.params();
        assert_eq!(None, tokenizer.next());
        assert_eq!(None, tokenizer.trailing().trailing())
    }

    #[test]
    fn test_tags() {
        let mut tokenizer = Tokenizer::new("@key1=value1;key2=value2 CMD").tags();
        assert_eq!(Some(("key1", "value1")), tokenizer.next());
        assert_eq!(Some(("key2", "value2")), tokenizer.next());
        assert_eq!(None, tokenizer.next());
        let mut tokenizer = tokenizer.prefix();
        assert_eq!((None, None, None), tokenizer.parts());
        let mut tokenizer = tokenizer.command();
        assert_eq!(Some("CMD"), tokenizer.command());
        let mut tokenizer = tokenizer.params();
        assert_eq!(None, tokenizer.next());
        assert_eq!(None, tokenizer.trailing().trailing())
    }

    #[test]
    fn test_prefix() {
        let mut tokenizer = Tokenizer::new(":name!user@host CMD").tags();
        assert_eq!(None, tokenizer.next());
        let mut tokenizer = tokenizer.prefix();
        assert_eq!(
            (Some("name"), Some("user"), Some("host")),
            tokenizer.parts()
        );
        let mut tokenizer = tokenizer.command();
        assert_eq!(Some("CMD"), tokenizer.command());
        let mut tokenizer = tokenizer.params();
        assert_eq!(None, tokenizer.next());
        assert_eq!(None, tokenizer.trailing().trailing())
    }

    #[test]
    fn test_prefix_name() {
        let mut tokenizer = Tokenizer::new(":name CMD").tags();
        assert_eq!(None, tokenizer.next());
        let mut tokenizer = tokenizer.prefix();
        assert_eq!((Some("name"), None, None), tokenizer.parts());
        let mut tokenizer = tokenizer.command();
        assert_eq!(Some("CMD"), tokenizer.command());
        let mut tokenizer = tokenizer.params();
        assert_eq!(None, tokenizer.next());
        assert_eq!(None, tokenizer.trailing().trailing())
    }

    #[test]
    fn test_prefix_user() {
        let mut tokenizer = Tokenizer::new(":name!user CMD").tags();
        assert_eq!(None, tokenizer.next());
        let mut tokenizer = tokenizer.prefix();
        assert_eq!((Some("name"), Some("user"), None), tokenizer.parts());
        let mut tokenizer = tokenizer.command();
        assert_eq!(Some("CMD"), tokenizer.command());
        let mut tokenizer = tokenizer.params();
        assert_eq!(None, tokenizer.next());
        assert_eq!(None, tokenizer.trailing().trailing())
    }

    #[test]
    fn test_prefix_host() {
        let mut tokenizer = Tokenizer::new(":name@host CMD").tags();
        assert_eq!(None, tokenizer.next());
        let mut tokenizer = tokenizer.prefix();
        assert_eq!((Some("name"), None, Some("host")), tokenizer.parts());
        let mut tokenizer = tokenizer.command();
        assert_eq!(Some("CMD"), tokenizer.command());
        let mut tokenizer = tokenizer.params();
        assert_eq!(None, tokenizer.next());
        assert_eq!(None, tokenizer.trailing().trailing())
    }

    #[test]
    fn test_params() {
        let mut tokenizer = Tokenizer::new("CMD param0 param1").tags();
        assert_eq!(None, tokenizer.next());
        let mut tokenizer = tokenizer.prefix();
        assert_eq!((None, None, None), tokenizer.parts());
        let mut tokenizer = tokenizer.command();
        assert_eq!(Some("CMD"), tokenizer.command());
        let mut tokenizer = tokenizer.params();
        assert_eq!(Some("param0"), tokenizer.next());
        assert_eq!(Some("param1"), tokenizer.next());
        assert_eq!(None, tokenizer.next());
        assert_eq!(None, tokenizer.trailing().trailing())
    }

    #[test]
    fn test_params_trailing() {
        let mut tokenizer = Tokenizer::new("CMD param0 param1 :Trailing parameter!").tags();
        assert_eq!(None, tokenizer.next());
        let mut tokenizer = tokenizer.prefix();
        assert_eq!((None, None, None), tokenizer.parts());
        let mut tokenizer = tokenizer.command();
        assert_eq!(Some("CMD"), tokenizer.command());
        let mut tokenizer = tokenizer.params();
        assert_eq!(Some("param0"), tokenizer.next());
        assert_eq!(Some("param1"), tokenizer.next());
        assert_eq!(None, tokenizer.next());
        assert_eq!(Some("Trailing parameter!"), tokenizer.trailing().trailing())
    }

    #[test]
    fn test_trailing() {
        let mut tokenizer = Tokenizer::new("CMD :Trailing parameter!").tags();
        assert_eq!(None, tokenizer.next());
        let mut tokenizer = tokenizer.prefix();
        assert_eq!((None, None, None), tokenizer.parts());
        let mut tokenizer = tokenizer.command();
        assert_eq!(Some("CMD"), tokenizer.command());
        let mut tokenizer = tokenizer.params();
        assert_eq!(None, tokenizer.next());
        assert_eq!(Some("Trailing parameter!"), tokenizer.trailing().trailing())
    }

    #[test]
    fn test_all() {
        let mut tokenizer = Tokenizer::new(
            "@key1=value1;key2=value2 :name!user@host CMD param0 param1 :Trailing parameter!@:=;",
        )
        .tags();
        assert_eq!(Some(("key1", "value1")), tokenizer.next());
        assert_eq!(Some(("key2", "value2")), tokenizer.next());
        let mut tokenizer = tokenizer.prefix();
        assert_eq!(
            (Some("name"), Some("user"), Some("host")),
            tokenizer.parts()
        );
        let mut tokenizer = tokenizer.command();
        assert_eq!(Some("CMD"), tokenizer.command());
        let mut tokenizer = tokenizer.params();
        assert_eq!(Some("param0"), tokenizer.next());
        assert_eq!(Some("param1"), tokenizer.next());
        assert_eq!(
            Some("Trailing parameter!@:=;"),
            tokenizer.trailing().trailing()
        )
    }
}
