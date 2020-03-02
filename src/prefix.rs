pub struct Prefix {
    raw: String,
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
            name: name.to_string(),
            user: None,
            host: None
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

pub struct PrefixBuilder {
    name: String,
    user: Option<String>,
    host: Option<String>,
}

impl<'a> PrefixBuilder {
    pub fn user(mut self, user: &str, host: &str) -> PrefixBuilder {
        self.user = Some(user.to_string());
        self.host = Some(host.to_string());
        self
    }

    pub fn host(mut self, host: &str) -> PrefixBuilder {
        self.host = Some(host.to_string());
        self
    }

    pub fn build(self) -> Prefix {
        let mut str = String::from(self.name);
        if let Some(user) = self.user {
            str.push('!');
            str.push_str(&user);
        }
        if let Some(host) = self.host {
            str.push('@');
            str.push_str(&host);
        }
        Prefix {
            raw: str
        }
    }
}