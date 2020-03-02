pub struct Params<'a> {
    raw: &'a str,
    pub trailing: Option<&'a str>
}

impl<'a> Params<'a> {
    pub fn new(raw: &'a str) -> Params<'a> {
        let trailing = raw.find(" :").map(|index| &raw[index + 2..]);

        Params {
            raw,
            trailing
        }
    }

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