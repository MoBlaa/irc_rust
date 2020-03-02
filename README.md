# irc-rust [![https://docs.rs/mio/badge.svg](https://docs.rs/irc-rust/badge.svg)](https://docs.rs/irc-rust)
IRC Helper easing the access and creation of IRC Messages.

## Basic Usage

```
use irc_rust::message::Message;

let message = Message::new("@key1=value1;key2=value2 :name!user@host CMD param1 param2 :trailing");

assert_eq!(message.to_string(), "@key1=value1;key2=value2 :name!user@host CMD param1 param2 :trailing");
```