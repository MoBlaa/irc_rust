pub mod irc_rust {
    use std::str::SplitWhitespace;

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

    pub struct Params<'a> {
        pub raw: &'a str,
        split: SplitWhitespace<'a>,
        trailing: Option<&'a str>,
        done: bool
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
                done: false
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

    pub struct Message<'a> {
        pub raw: &'a str
    }

    impl<'a> Message<'a> {
        pub fn prefix(&self) -> Option<Prefix<'a>> {
            if self.raw.starts_with(':') {
                match self.raw.find(' ') {
                    Some(index) => Some(Prefix {
                        raw: &self.raw[1..index],
                    }),
                    None => Some(Prefix {
                        raw: &self.raw[1..]
                    })
                }
            } else {
                None
            }
        }

        pub fn command(&self) -> &'a str {
            let without_prefix = match self.raw.find(' ') {
                Some(start) => {
                    if self.raw.starts_with(":") {
                        &self.raw[start + 1..]
                    } else {
                        &self.raw
                    }
                }
                None => self.raw
            };
            match without_prefix.find(' ') {
                Some(end) => &without_prefix[..end],
                None => without_prefix
            }
        }

        pub fn params(&self) -> Option<Params<'a>> {
            let command = self.command();
            let cmd_start = self.raw.find(command).unwrap();
            self.raw[cmd_start..].find(' ')
                .and_then(|param_start| Some(Params::new(&self.raw[cmd_start + param_start..])))
        }
    }

    impl<'a> ToString for Message<'a> {
        fn to_string(&self) -> String {
            self.raw.to_string()
        }
    }
}

#[cfg(test)]
mod test {
    use crate::irc_rust::Message;

    #[test]
    fn test_parse() {
        let message = Message {
            raw: ":name!user@host CMD param1 param2 :trailing"
        };

        let prefix = message.prefix().unwrap();
        assert_eq!(prefix.name(), "name");
        assert_eq!(prefix.user().unwrap(), "user");
        assert_eq!(prefix.host().unwrap(), "host");

        assert_eq!(message.command(), "CMD");

        let mut params = message.params().unwrap();
        assert_eq!(params.next().unwrap(), "param1");
        assert_eq!(params.next().unwrap(), "param2");
        assert_eq!(params.next().unwrap(), "trailing");

        assert_eq!(params.trailing().unwrap(), "trailing")
    }

    #[test]
    fn test_without_prefix() {
        let message = Message {
            raw: "CMD param1 param2 :trailing"
        };

        let prefix = message.prefix();
        assert!(prefix.is_none());

        assert_eq!(message.command(), "CMD");

        let mut params = message.params().unwrap();
        assert_eq!(params.next().unwrap(), "param1");
        assert_eq!(params.next().unwrap(), "param2");
        assert_eq!(params.next().unwrap(), "trailing");

        assert_eq!(params.trailing().unwrap(), "trailing")
    }

    #[test]
    fn test_command_only() {
        let message = Message {
            raw: "CMD"
        };

        assert!(message.prefix().is_none());

        assert_eq!(message.command(), "CMD");

        assert!(message.params().is_none());
    }

    #[test]
    fn test_cmd_and_trailing() {
        let message = Message {
            raw: "CMD :trailing"
        };

        assert!(message.prefix().is_none());

        assert_eq!(message.command(), "CMD");

        let mut params = message.params().unwrap();
        assert_eq!(params.next().unwrap(), "trailing");

        assert_eq!(params.trailing().unwrap(), "trailing")
    }

    #[test]
    fn test_cmd_and_param() {
        let message = Message {
            raw: "CMD param1"
        };

        assert!(message.prefix().is_none());

        assert_eq!(message.command(), "CMD");

        let mut params = message.params().unwrap();
        assert_eq!(params.next().unwrap(), "param1");
        assert!(params.next().is_none());

        assert!(params.trailing().is_none());
    }

    #[test]
    fn test_prefix() {
        let message = Message {
            raw: ":name CMD"
        };

        let prefix = message.prefix().unwrap();
        assert_eq!(prefix.name(), "name");
        assert!(prefix.user().is_none());
        assert!(prefix.host().is_none());

        assert_eq!(message.command(), "CMD");

        assert!(message.params().is_none());

        let message = Message {
            raw: ":name@host CMD"
        };

        let prefix = message.prefix().unwrap();
        assert_eq!(prefix.name(), "name");
        assert!(prefix.user().is_none());
        assert_eq!(prefix.host().unwrap(), "host");

        assert_eq!(message.command(), "CMD");

        assert!(message.params().is_none());
    }
}
