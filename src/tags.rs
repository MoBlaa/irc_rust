pub struct Tags<'a> {
    pub raw: &'a str,
    cursor: usize,
    done: bool,
}

impl<'a> Tags<'a> {
    pub fn new(raw: &'a str) -> Tags<'a> {
        Tags {
            raw,
            cursor: 0,
            done: false,
        }
    }
}

impl<'a> Iterator for Tags<'a> {
    type Item = (&'a str, &'a str);

    fn next(&mut self) -> Option<(&'a str, &'a str)> {
        if self.done {
            return None;
        }
        let next: &'a str = match self.raw[self.cursor..].find(';') {
            Some(end) => {
                let result = &self.raw[self.cursor..self.cursor + end];
                self.cursor = self.cursor + end + 1;
                result
            }
            None => {
                self.done = true;
                &self.raw[self.cursor..]
            }
        };
        next.find('=').and_then(|index| Some((&next[..index], &next[index + 1..])))
    }
}