use core::fmt;
use serde::{Deserialize, Serialize};

/// Parameter list with an optional trailing message.
#[derive(Serialize, Deserialize, Debug, Clone, Copy, Eq, Ord, PartialOrd, PartialEq, Hash, Default)]
pub struct Params<'a> {
    raw: &'a str,
    pub trailing: Option<&'a str>
}

impl<'a> Params<'a> {
    /// Create a new Parameter list from the given string. Expects the string to be a valid parameter list.
    pub fn new() -> Params<'a> {
        Params {
            raw: "",
            trailing: None
        }
    }

    /// Create an iterator over the parameter list excluding the trailing parameter.
    pub fn iter(&self) -> impl Iterator<Item = &'a str> {
        match self.raw.find(" :") {
            // Split into parameter list and trailing
            Some(index) => self.raw[..index].split_whitespace(),
            // Only split parameters
            None => self.raw.split_whitespace()
        }
    }
}

impl<'a> fmt::Display for Params<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.raw)
    }
}

impl<'a> From<&'a str> for Params<'a> {
    fn from(raw: &'a str) -> Self {
        let trailing = raw.find(" :").map(|index| &raw[index + 2..]);

        Params {
            raw,
            trailing
        }
    }
}
