pub use crate::irc_rust::Prefix;

#[cfg(test)]
mod test {
    fn test_prefix() {
        let servername = String::from("hello");
        let prefix = Prefix {
            raw: &servername,
        };
        assert_eq!("hello", prefix.name())
    }
}