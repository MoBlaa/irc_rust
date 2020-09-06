use std::ops::Index;
use core::fmt;

/// Tag Map as described through IRCv3.
#[derive(Debug, Clone, Copy, Eq, Ord, PartialOrd, PartialEq, Hash, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Tags<'a> {
    raw: &'a str,
}

impl<'a> Tags<'a> {
    /// Create a new Tag map from the given string. Expects it to be in valid IRCv3 format.
    pub fn new() -> Tags<'a> {
        Tags {
            raw: "",
        }
    }

    /// Character length of the tags if formatted as IRC string.
    pub fn len(&self) -> usize {
        self.raw.len()
    }

    pub fn is_empty(&self) -> bool {
        self.raw.is_empty()
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
        let key_equals = format!("{}=", key);
        self.raw.find(&key_equals)
            .map(|start| {
                start + key.len() + 1
            })
            .and_then(|start| {
                self.raw[start..].find(';')
                    .or_else(|| self.raw[start..].find(' '))
                    .or_else(|| Some(self.raw.len() - start))
                    .map(|end| (start, start + end))
            })
    }

    pub fn get(&self, key: &'a str) -> Option<&'a str> {
        self.find(key)
            .map(|(start, end)| &self.raw[start..end])
    }
}

impl<'a> From<&'a str> for Tags<'a> {
    fn from(raw: &'a str) -> Self {
        Tags {
            raw
        }
    }
}

impl<'a> Index<&'a str> for Tags<'a> {
    type Output = str;

    fn index(&self, key: &'a str) -> &Self::Output {
        // Find the key
        let (start, end) = self.find(key)
            .expect("no element with key found");
        &self.raw[start..end]
    }
}

impl<'a> fmt::Display for Tags<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.raw.fmt(f)
    }
}

#[cfg(test)]
mod tests {
    use crate::tags::Tags;

    #[test]
    fn test_get_and_index() {
        let tags = Tags::from("hello=world;whats=goes");
        let get = tags.get("hello");
        let index = &tags["hello"];
        assert_eq!(get, Some("world"));
        assert_eq!(index, "world");
        let get = tags.get("whats");
        let index = &tags["whats"];
        assert_eq!(get, Some("goes"));
        assert_eq!(index, "goes");
        let get = tags.get("world");
        // Would panic
        // let index = &tags["world"];
        assert_eq!(get, None);
    }
}
