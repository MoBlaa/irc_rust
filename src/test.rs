use crate::message::Message;

#[test]
fn test_tags() {
    let message = Message::new("@tag1=value1;tag2=value2 CMD");

    let mut tags = message.tags().unwrap();
    let (key, val) = tags.next().unwrap();
    assert_eq!(key, "tag1");
    assert_eq!(val, "value1");
    let (key, val) = tags.next().unwrap();
    assert_eq!(key, "tag2");
    assert_eq!(val, "value2");
    assert!(tags.next().is_none());

    let message = Message::new("@tag1=value1 CMD");

    let mut tags = message.tags().unwrap();
    let (key, val) = tags.next().unwrap();
    assert_eq!(key, "tag1");
    assert_eq!(val, "value1");
    assert!(tags.next().is_none());

    let message = Message::new("@tag1=value1;tag2=value2 :name CMD :trailing");

    let mut tags = message.tags().unwrap();
    let (key, val) = tags.next().unwrap();
    assert_eq!(key, "tag1");
    assert_eq!(val, "value1");
    let (key, val) = tags.next().unwrap();
    assert_eq!(key, "tag2");
    assert_eq!(val, "value2");
    assert!(tags.next().is_none());

    assert!(message.prefix().is_some());

    let message = Message::new("@tag1=value1;tag2=value2 CMD :trailing");

    let mut tags = message.tags().unwrap();
    let (key, val) = tags.next().unwrap();
    assert_eq!(key, "tag1");
    assert_eq!(val, "value1");
    let (key, val) = tags.next().unwrap();
    assert_eq!(key, "tag2");
    assert_eq!(val, "value2");
    assert!(tags.next().is_none());

    assert!(message.prefix().is_none());
}

#[test]
fn test_parse() {
    let message = Message::new(":name!user@host CMD param1 param2 :trailing");

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
    let message = Message::new("CMD param1 param2 :trailing");

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
    let message = Message::new("CMD");

    assert!(message.prefix().is_none());

    assert_eq!(message.command(), "CMD");

    assert!(message.params().is_none());
}

#[test]
fn test_cmd_and_trailing() {
    let message = Message::new("CMD :trailing");

    assert!(message.prefix().is_none());

    assert_eq!(message.command(), "CMD");

    let mut params = message.params().unwrap();
    assert_eq!(params.next().unwrap(), "trailing");

    assert_eq!(params.trailing().unwrap(), "trailing")
}

#[test]
fn test_cmd_and_param() {
    let message = Message::new("CMD param1");

    assert!(message.prefix().is_none());

    assert_eq!(message.command(), "CMD");

    let mut params = message.params().unwrap();
    assert_eq!(params.next().unwrap(), "param1");
    assert!(params.next().is_none());

    assert!(params.trailing().is_none());
}

#[test]
fn test_prefix() {
    let message = Message::new(":name CMD");

    let prefix = message.prefix().unwrap();
    assert_eq!(prefix.name(), "name");
    assert!(prefix.user().is_none());
    assert!(prefix.host().is_none());

    assert_eq!(message.command(), "CMD");

    assert!(message.params().is_none());

    let message = Message::new(":name@host CMD");

    let prefix = message.prefix().unwrap();
    assert_eq!(prefix.name(), "name");
    assert!(prefix.user().is_none());
    assert_eq!(prefix.host().unwrap(), "host");

    assert_eq!(message.command(), "CMD");

    assert!(message.params().is_none());
}
