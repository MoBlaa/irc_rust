/// Message prefix containing a name (servername or nickname) and optional
/// user and host. If the user and host are set the name is semantically
/// seen as the nickname.
pub struct Prefix<'a> {
    raw: &'a str
}

impl<'a> ToString for Prefix<'a> {
    fn to_string(&self) -> String {
        self.raw.to_string()
    }
}

impl<'a> Prefix<'a> {
    /// Create a new Prefix from the given string. Expects the string to be a valid prefix string.
    pub fn new(raw: &'a str) -> Prefix<'a> {
        Prefix {
            raw
        }
    }

    // Returns the (server- or nick-) name.
    pub fn name(&self) -> &'a str {
        let end = self.raw.find('!')
            .or(self.raw.find('@'))
            .or(self.raw.find(' '))
            .unwrap_or(self.raw.len());
        &self.raw[..end]
    }

    // Returns the host if present.
    pub fn host(&self) -> Option<&'a str> {
        self.raw.find('@')
            .and_then(|index| Some(&self.raw[index + 1..]))
    }

    // Returns the host if present.
    pub fn user(&self) -> Option<&'a str> {
        self.raw.find('!')
            .and_then(|start| {
                let end = self.raw.find('@')
                    .unwrap_or(self.raw.len());
                Some(&self.raw[start + 1..end])
            })
    }
}