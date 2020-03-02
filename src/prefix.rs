pub struct Prefix<'a> {
    pub raw: &'a str,
}

impl<'a> ToString for Prefix<'a> {
    fn to_string(&self) -> String {
        self.raw.to_string()
    }
}

impl<'a> Prefix<'a> {
    pub fn name(&self) -> &'a str {
        let end = self.raw.find('!')
            .or(self.raw.find('@'))
            .or(self.raw.find(' '))
            .unwrap_or(self.raw.len());
        &self.raw[..end]
    }

    pub fn host(&self) -> Option<&'a str> {
        self.raw.find('@')
            .and_then(|index| Some(&self.raw[index + 1..]))
    }

    pub fn user(&self) -> Option<&'a str> {
        self.raw.find('!')
            .and_then(|start| {
                let end = self.raw.find('@')
                    .unwrap_or(self.raw.len());
                Some(&self.raw[start + 1..end])
            })
    }
}