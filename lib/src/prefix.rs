use std::fmt;
use std::ops::{Range, RangeFrom, RangeTo};

pub trait Prefixed<'a>: Sized {
    fn name(&self) -> &'a str;
    fn user(&self) -> Option<&'a str>;
    fn host(&self) -> Option<&'a str>;
    fn as_parts(&self) -> (&'a str, Option<&'a str>, Option<&'a str>) {
        (self.name(), self.user(), self.host())
    }
}

/// Message prefix containing a name (servername or nickname) and optional
/// user and host. If the user and host are set the name is semantically
/// seen as the nickname.
#[derive(Debug, Clone, Copy, Eq, Ord, PartialOrd, PartialEq, Hash, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Prefix<'a> {
    raw: &'a str,
}

impl<'a> Prefix<'a> {
    /// Create a new Prefix from the given string. Expects the string to be a valid prefix string.
    pub fn new() -> Self {
        Prefix { raw: "" }
    }

    fn name_bounds(&self) -> RangeTo<usize> {
        let end = self
            .raw
            .find('!')
            .or_else(|| self.raw.find('@'))
            .or_else(|| self.raw.find(' '))
            .unwrap_or_else(|| self.raw.len());
        ..end
    }

    fn host_bounds(&self) -> Option<RangeFrom<usize>> {
        self.raw.find('@').map(|index| index + 1..)
    }

    fn user_bounds(&self) -> Option<Range<usize>> {
        self.raw.find('!').map(|start| {
            let end = self.raw.find('@').unwrap_or_else(|| self.raw.len());
            start + 1..end
        })
    }

    pub fn into_parts(self) -> (&'a str, Option<&'a str>, Option<&'a str>) {
        let name_bounds = self.name_bounds();
        let user_bounds = self.user_bounds();
        let host_bounds = self.host_bounds();

        (
            &self.raw[name_bounds],
            user_bounds.map(|range| &self.raw[range]),
            host_bounds.map(|range| &self.raw[range]),
        )
    }
}

impl<'a> Prefixed<'a> for Prefix<'a> {
    // Returns the (server- or nick-) name.
    fn name(&self) -> &'a str {
        &self.raw[self.name_bounds()]
    }

    // Returns the host if present.
    fn user(&self) -> Option<&'a str> {
        self.user_bounds().map(|range| &self.raw[range])
    }

    // Returns the host if present.
    fn host(&self) -> Option<&'a str> {
        self.host_bounds().map(|range| &self.raw[range])
    }
}

impl<'a> From<&'a str> for Prefix<'a> {
    fn from(raw: &'a str) -> Self {
        Prefix { raw }
    }
}

impl<'a> fmt::Display for Prefix<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.raw.fmt(f)
    }
}

impl<'a> AsRef<str> for Prefix<'a> {
    fn as_ref(&self) -> &str {
        self.raw
    }
}
