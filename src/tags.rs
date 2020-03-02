use std::ops::Index;

/// Tag Map as described through IRCv3.
pub struct Tags<'a> {
    raw: &'a str,
}

impl<'a> Tags<'a> {
    /// Create a new Tag map from the given string. Expects it to be in valid IRCv3 format.
    pub fn new(raw: &'a str) -> Tags<'a> {
        Tags {
            raw,
        }
    }

    /// Character length of the tags if formatted as IRC string.
    pub fn len(&self) -> usize {
        self.raw.len()
    }

    /// Iterator over the tag entries.
    pub fn iter(&self) -> impl Iterator<Item=(&'a str, &'a str)> {
        self.raw.split(';')
            .map(|kv| {
                let mut split = kv.split('=');
                (split.next().unwrap(), split.next().unwrap())
            })
    }

    // Search for the key and return start and end of the value
    fn find(&self, key: &'a str) -> Option<(usize, usize)> {
        self.raw.find(key)
            .map(|start| {
                start + key.len() + 1
            })
            .and_then(|start| {
                self.raw[start..].find(';')
                    .or(self.raw[start..].find(' '))
                    .or(Some(self.raw.len() - start))
                    .map(|end| (start, start + end))
            })
    }
}

impl<'a> Index<&'a str> for Tags<'a> {
    type Output = str;

    fn index(&self, key: &'a str) -> &Self::Output {
        // Find the key
        let (start, end) = self.find(key).unwrap();
        &self.raw[start..end]
    }
}