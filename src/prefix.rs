pub struct Prefix {
    raw: String
}

impl ToString for Prefix {
    fn to_string(&self) -> String {
        self.raw.to_string()
    }
}

impl Prefix {
    pub fn new(raw: &str) -> Prefix {
        Prefix {
            raw: raw.to_string()
        }
    }

    pub fn builder(name: &str) -> PrefixBuilder {
        PrefixBuilder {
            name,
            user: None,
            host: None,
        }
    }

    pub fn name(&self) -> &str {
        let end = self.raw.find('!')
            .or(self.raw.find('@'))
            .or(self.raw.find(' '))
            .unwrap_or(self.raw.len());
        &self.raw[..end]
    }

    pub fn host(&self) -> Option<&str> {
        self.raw.find('@')
            .and_then(|index| Some(&self.raw[index + 1..]))
    }

    pub fn user(&self) -> Option<&str> {
        self.raw.find('!')
            .and_then(|start| {
                let end = self.raw.find('@')
                    .unwrap_or(self.raw.len());
                Some(&self.raw[start + 1..end])
            })
    }
}

pub struct PrefixBuilder<'a> {
    name: &'a str,
    user: Option<&'a str>,
    host: Option<&'a str>,
}

impl<'a> PrefixBuilder<'a> {
    pub fn user(mut self, user: &'a str) -> PrefixBuilder<'a> {
        self.user = Some(user);
        self
    }

    pub fn host(mut self, host: &'a str) -> PrefixBuilder<'a> {
        self.host = Some(host);
        self
    }

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