/// Parameter list with an optional trailing message.
pub struct Params<'a> {
    raw: &'a str,
    pub trailing: Option<&'a str>
}

impl<'a> Params<'a> {
    /// Create a new Parameter list from the given string. Expects the string to be a valid parameter list.
    pub fn new(raw: &'a str) -> Params<'a> {
        let trailing = raw.find(" :").map(|index| &raw[index + 2..]);

        Params {
            raw,
            trailing
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

impl<'a> ToString for Params<'a> {
    fn to_string(&self) -> String {
        self.raw.to_string()
    }
}