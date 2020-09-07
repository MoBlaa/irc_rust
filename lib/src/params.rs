use core::fmt;

/// Parameter list with an optional trailing message.
#[derive(Debug, Clone, Copy, Eq, Ord, PartialOrd, PartialEq, Hash, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Params<'a> {
    raw: &'a str,
    trailing_start: Option<usize>
}

impl<'a> Params<'a> {
    /// Create a new Parameter list from the given string. Expects the string to be a valid parameter list.
    pub fn new() -> Params<'a> {
        Params {
            raw: "",
            trailing_start: None
        }
    }

    /// Returns the trailing parameter which is seperated from the
    /// other parameters with ' :'.
    pub fn trailing(&self) -> Option<&'a str> {
        self.trailing_start
            .map(|index| &self.raw[index + 2..])
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
        self.raw.fmt(f)
    }
}

impl<'a> From<&'a str> for Params<'a> {
    fn from(raw: &'a str) -> Self {
        let trailing_start = raw.find(" :");

        Params {
            raw,
            trailing_start
        }
    }
}

impl<'a> AsRef<str> for Params<'a> {
    fn as_ref(&self) -> &str {
        self.raw
    }
}
