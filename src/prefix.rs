/// Message prefix containing a name (servername or nickname) and optional
/// user and host. If the user and host are set the name is semantically
/// seen as the nickname.
pub struct Prefix {
    raw: String
}

impl ToString for Prefix {
    fn to_string(&self) -> String {
        self.raw.to_string()
    }
}

impl Prefix {
    /// Create a new Prefix from the given string. Expects the string to be a valid prefix string.
    pub fn new(raw: &str) -> Prefix {
        Prefix {
            raw: raw.to_string()
        }
    }

    // Creates a builder for simpler creation as alternative to building a string by itself.
    pub fn builder(name: &str) -> PrefixBuilder {
        PrefixBuilder {
            name,
            user: None,
            host: None,
        }
    }

    // Returns the (server- or nick-) name.
    pub fn name(&self) -> &str {
        let end = self.raw.find('!')
            .or(self.raw.find('@'))
            .or(self.raw.find(' '))
            .unwrap_or(self.raw.len());
        &self.raw[..end]
    }

    // Returns the host if present.
    pub fn host(&self) -> Option<&str> {
        self.raw.find('@')
            .and_then(|index| Some(&self.raw[index + 1..]))
    }

    // Returns the host if present.
    pub fn user(&self) -> Option<&str> {
        self.raw.find('!')
            .and_then(|start| {
                let end = self.raw.find('@')
                    .unwrap_or(self.raw.len());
                Some(&self.raw[start + 1..end])
            })
    }
}

/// A Prefix builder.
pub struct PrefixBuilder<'a> {
    name: &'a str,
    user: Option<&'a str>,
    host: Option<&'a str>,
}

impl<'a> PrefixBuilder<'a> {
    /// Set the user.
    pub fn user(mut self, user: &'a str) -> PrefixBuilder<'a> {
        self.user = Some(user);
        self
    }

    /// Set the host.
    pub fn host(mut self, host: &'a str) -> PrefixBuilder<'a> {
        self.host = Some(host);
        self
    }

    /// Returns a valid prefix or an error if user is set without host.
    pub fn build(self) -> Result<Prefix, &'a str> {
        if self.user.is_some() && self.host.is_none() {
            return Err("user can only be present if host is also present");
        }

        let mut str = String::from(self.name);
        if let Some(user) = self.user {
            str.push('!');
            str.push_str(&user);
        }
        if let Some(host) = self.host {
            str.push('@');
            str.push_str(&host);
        }
        Ok(Prefix {
            raw: str
        })
    }
}