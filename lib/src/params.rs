use core::fmt;

/// Parameter list with an optional trailing message.
#[derive(Debug, Clone, Copy, Eq, Ord, PartialOrd, PartialEq, Hash, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Params<'a> {
    raw: &'a str,
}

impl<'a> Params<'a> {
    /// Create a new Parameter list from the given string. Expects the string to be a valid parameter list.
    pub fn new() -> Params<'a> {
        Params { raw: "" }
    }

    fn trailing_start(&self) -> Option<usize> {
        self.raw.find(" :")
    }

    /// Returns the trailing parameter which is seperated from the
    /// other parameters with ' :'.
    pub fn trailing(&self) -> Option<&str> {
        self.trailing_start().map(|index| &self.raw[index + 2..])
    }

    /// Create an iterator over the parameter list excluding the trailing parameter.
    pub fn iter(&self) -> impl Iterator<Item = &str> {
        let params = match self.trailing_start() {
            // Split into parameter list and trailing
            Some(index) => &self.raw[..index],
            // Only split parameters
            None => self.raw,
        };
        params.split_whitespace()
    }

    pub fn into_parts(self) -> (impl Iterator<Item = &'a str>, Option<&'a str>) {
        let (params, trailing) = match self.trailing_start() {
            Some(index) => (&self.raw[..index], Some(&self.raw[index + 2..])),
            None => (self.raw, None),
        };
        (params.split_whitespace(), trailing)
    }
}

impl<'a> fmt::Display for Params<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.raw.fmt(f)
    }
}

impl<'a> From<&'a str> for Params<'a> {
    fn from(raw: &'a str) -> Self {
        Params { raw }
    }
}

impl<'a> AsRef<str> for Params<'a> {
    fn as_ref(&self) -> &str {
        self.raw
    }
}
