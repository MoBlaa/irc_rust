use std::str::SplitWhitespace;

pub struct Params<'a> {
    pub raw: &'a str,
    split: SplitWhitespace<'a>,
    trailing: Option<&'a str>,
    done: bool,
}

impl<'a> Params<'a> {
    pub fn new(raw: &'a str) -> Params<'a> {
        let (split, trailing) = match raw.find(" :") {
            // Split into parameter list and trailing
            Some(index) => (raw[..index].split_whitespace(), Some(&raw[index + 2..])),
            // Only split parameters
            None => (raw.split_whitespace(), None)
        };

        Params {
            raw,
            split,
            trailing,
            done: false,
        }
    }

    pub fn trailing(&self) -> Option<&'a str> {
        self.trailing
    }
}

impl<'a> ToString for Params<'a> {
    fn to_string(&self) -> String {
        self.raw.to_string()
    }
}

impl<'a> Iterator for Params<'a> {
    type Item = &'a str;

    fn next(&mut self) -> Option<&'a str> {
        match self.split.next() {
            Some(next) => Some(next),
            None => {
                if self.done {
                    None
                } else {
                    self.done = true;
                    self.trailing
                }
            }
        }
    }
}