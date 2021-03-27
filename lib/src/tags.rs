use crate::errors::InvalidIrcFormatError;
use core::fmt;
use std::convert::TryFrom;

/// Tag Map as described through IRCv3.
///
#[derive(Debug, Clone, Eq, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Tags<'a> {
    raw: &'a str,
}

impl<'a> Tags<'a> {
    /// Create a new Tag map from the given string. Expects it to be in valid IRCv3 format.
    pub fn new() -> Tags<'a> {
        Tags { raw: "" }
    }

    /// Character length of the tags if formatted as IRC string.
    pub fn len_raw(&self) -> usize {
        self.raw.len()
    }

    /// Please use [iter] for these operations.
    #[deprecated]
    pub fn len(&self) -> usize {
        self.iter().count()
    }

    /// Please use [iter] for these operations.
    #[deprecated]
    pub fn is_empty(&self) -> bool {
        self.iter().next().is_none()
    }

    /// Iterator over the tag entries.
    pub fn iter(&self) -> impl Iterator<Item = (&'a str, &'a str)> {
        self.raw.split(';').map(|kv| {
            let mut split = kv.split('=');
            (split.next().unwrap(), split.next().unwrap())
        })
    }

    // Search for the key and return start and end of the value
    fn find(&self, key: &str) -> Option<(usize, usize)> {
        let key_equals = format!("{}=", key);
        self.raw
            .find(&key_equals)
            .map(|start| start + key.len() + 1)
            .and_then(|start| {
                self.raw[start..]
                    .find(';')
                    .or_else(|| self.raw[start..].find(' '))
                    .or_else(|| Some(self.raw.len() - start))
                    .map(|end| (start, start + end))
            })
    }

    pub fn get(&self, key: &str) -> Option<&str> {
        self.find(key).map(|(start, end)| &self.raw[start..end])
    }
}

impl<'a> TryFrom<&'a str> for Tags<'a> {
    type Error = InvalidIrcFormatError;

    fn try_from(raw: &'a str) -> Result<Self, Self::Error> {
        let raw = raw.trim_matches(&[' ', '@'][..]);

        if raw.contains(' ') {
            return Err(InvalidIrcFormatError::Tag(raw.to_string()));
        }

        Ok(Tags { raw })
    }
}

impl<'a> fmt::Display for Tags<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.raw.fmt(f)
    }
}

impl<'a> AsRef<str> for Tags<'a> {
    fn as_ref(&self) -> &str {
        self.raw
    }
}

#[cfg(test)]
mod tests {
    use crate::tags::Tags;
    use crate::InvalidIrcFormatError;
    use std::convert::TryFrom;

    #[test]
    fn test_get_and_index() -> Result<(), InvalidIrcFormatError> {
        let tags = Tags::try_from("hello=world;whats=goes;hello2=world2")?;
        let get = tags.get("hello");
        assert_eq!(get, Some("world"));
        let get = tags.get("whats");
        assert_eq!(get, Some("goes"));
        let get = tags.get("world");
        assert_eq!(get, None);
        Ok(())
    }
}
