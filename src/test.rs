use crate::message::Message;

#[test]
fn test_parse() {
    let message = Message::from("@key1=value1;key2=value2 :name!user@host CMD param1 param2 :trailing");

    assert_eq!(message.to_string(), "@key1=value1;key2=value2 :name!user@host CMD param1 param2 :trailing");

    let tags = message.tags().unwrap();
    let val = &tags["key1"];
    assert_eq!(val, "value1");
    let val = &tags["key2"];
    assert_eq!(val, "value2");

    let mut tags = message.tags().unwrap().iter();
    let (key, val) = tags.next().unwrap();
    assert_eq!(key, "key1");
    assert_eq!(val, "value1");
    let (key, val) = tags.next().unwrap();
    assert_eq!(key, "key2");
    assert_eq!(val, "value2");

    let prefix = message.prefix().unwrap();
    assert_eq!(prefix.name(), "name");
    assert_eq!(prefix.user().unwrap(), "user");
    assert_eq!(prefix.host().unwrap(), "host");

    assert_eq!(message.command(), "CMD");

    let params = message.params().unwrap();
    let mut iter = params.iter();
    assert_eq!(iter.next().unwrap(), "param1");
    assert_eq!(iter.next().unwrap(), "param2");
    assert!(iter.next().is_none());
    assert_eq!(params.trailing.unwrap(), "trailing");
}

#[test]
fn test_tags() {
    let message = Message::from("@tag1=value1;tag2=value2 CMD");

    let tags = message.tags().unwrap();
    let val = &tags["tag1"];
    assert_eq!(val, "value1");
    let val = &tags["tag2"];
    assert_eq!(val, "value2");

    let mut tags = message.tags().unwrap().iter();
    let (key, val) = tags.next().unwrap();
    assert_eq!(key, "tag1");
    assert_eq!(val, "value1");
    let (key, val) = tags.next().unwrap();
    assert_eq!(key, "tag2");
    assert_eq!(val, "value2");
    assert!(tags.next().is_none());

    let message = Message::from("@tag1=value1 CMD");

    let mut tags = message.tags().unwrap().iter();
    let (key, val) = tags.next().unwrap();
    assert_eq!(key, "tag1");
    assert_eq!(val, "value1");
    assert!(tags.next().is_none());

    let message = Message::from("@tag1=value1;tag2=value2 :name CMD :trailing");

    let mut tags = message.tags().unwrap().iter();
    let (key, val) = tags.next().unwrap();
    assert_eq!(key, "tag1");
    assert_eq!(val, "value1");
    let (key, val) = tags.next().unwrap();
    assert_eq!(key, "tag2");
    assert_eq!(val, "value2");
    assert!(tags.next().is_none());

    assert!(message.prefix().is_some());

    let message = Message::from("@tag1=value1;tag2=value2 CMD :trailing");

    let mut tags = message.tags().unwrap().iter();
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
fn test_without_prefix() {
    let message = Message::from("CMD param1 param2 :trailing");

    let prefix = message.prefix();
    assert!(prefix.is_none());

    assert_eq!(message.command(), "CMD");

    let params = message.params().unwrap();
    let mut iter = params.iter();
    assert_eq!(iter.next().unwrap(), "param1");
    assert_eq!(iter.next().unwrap(), "param2");
    assert!(iter.next().is_none());

    assert_eq!(params.trailing.unwrap(), "trailing")
}

#[test]
fn test_command_only() {
    let message = Message::from("CMD");

    assert!(message.prefix().is_none());

    assert_eq!(message.command(), "CMD");

    assert!(message.params().is_none());
}

#[test]
fn test_cmd_and_trailing() {
    let message = Message::from("CMD :trailing");

    assert!(message.prefix().is_none());

    assert_eq!(message.command(), "CMD");

    let params = message.params().unwrap();
    let mut iter = params.iter();
    assert!(iter.next().is_none());

    assert_eq!(params.trailing.unwrap(), "trailing")
}

#[test]
fn test_cmd_and_param() {
    let message = Message::from("CMD param1");

    assert!(message.prefix().is_none());

    assert_eq!(message.command(), "CMD");

    let params = message.params().unwrap();
    let mut iter = params.iter();
    assert_eq!(iter.next().unwrap(), "param1");
    assert!(iter.next().is_none());

    assert!(params.trailing.is_none());
}

#[test]
fn test_prefix() {
    let message = Message::from(":name CMD");

    let prefix = message.prefix().unwrap();
    assert_eq!(prefix.name(), "name");
    assert!(prefix.user().is_none());
    assert!(prefix.host().is_none());

    assert_eq!(message.command(), "CMD");

    assert!(message.params().is_none());

    let message = Message::from(":name@host CMD");

    let prefix = message.prefix().unwrap();
    assert_eq!(prefix.name(), "name");
    assert!(prefix.user().is_none());
    assert_eq!(prefix.host().unwrap(), "host");

    assert_eq!(message.command(), "CMD");

    assert!(message.params().is_none());
}

#[test]
fn test_message_builder() {
    let message = Message::builder()
        .tag("key1", "value1")
        .tag("key2", "value2")
        .prefix_name("name")
        .prefix_user("user")
        .prefix_host("host")
        .command("CMD")
        .param("param1").param("param2")
        .trailing("trailing")
        .build();
    let str = message.to_string();
    assert!(str.as_str() == "@key1=value1;key2=value2 :name!user@host CMD param1 param2 :trailing" || str.as_str() == "@key2=value2;key1=value1 :name!user@host CMD param1 param2 :trailing");

    let message = message.to_builder()
        .tag("key1", "value3")
        .prefix_name("name1")
        .param("param2")
        .set_param(1, "param3")
        .trailing("other trailing!")
        .build();
    let str = message.to_string();
    assert!(str.as_str() == "@key1=value3;key2=value2 :name1!user@host CMD param1 param3 param2 :other trailing!" || str.as_str() == "@key2=value2;key1=value3 :name1!user@host CMD param1 param3 param2 :other trailing!");
}