use crate::errors::ParserError;
use crate::message::Message;
use crate::params::Parameterized;
use crate::tags::Taggable;
use std::error::Error;

#[test]
fn test_parse() -> Result<(), Box<dyn Error>> {
    let message =
        Message::from("@key1=value1;key2=value2 :name!user@host CMD param1 param2 :trailing");

    assert_eq!(
        message.to_string(),
        "@key1=value1;key2=value2 :name!user@host CMD param1 param2 :trailing"
    );

    let parsed = message.parsed()?;

    let val = parsed.tag("key1");
    assert_eq!(val, Some("value1"));
    let val = parsed.tag("key2");
    assert_eq!(val, Some("value2"));

    let mut tags = parsed.tags();
    let (key, val) = tags.next().unwrap();
    let (key2, val2) = tags.next().unwrap();
    assert!(
        (*key == "key1" && *val == "value1" && *key2 == "key2" && *val2 == "value2")
            || (*key2 == "key1" && *val2 == "value1" && *key == "key2" && *val == "value2")
    );

    let (name, user, host) = message.prefix()?.unwrap();
    assert_eq!(name, "name");
    assert_eq!(user, Some("user"));
    assert_eq!(host, Some("host"));

    assert_eq!(message.command()?, "CMD");

    let mut iter = message.params()?.into_iter();
    assert_eq!(iter.next(), Some("param1"));
    assert_eq!(iter.next(), Some("param2"));
    assert!(iter.next().is_none());
    assert_eq!(parsed.trailing(), Some("trailing"));

    Ok(())
}

#[test]
fn test_tags() -> Result<(), Box<dyn Error>> {
    let message = Message::from("@tag1=value1;tag2=value2 CMD");

    let mut tags = message.tags()?.into_iter();
    let (key, val) = tags.next().unwrap()?;
    assert_eq!(key, "tag1");
    assert_eq!(val, "value1");
    let (key, val) = tags.next().unwrap()?;
    assert_eq!(key, "tag2");
    assert_eq!(val, "value2");
    assert!(tags.next().is_none());

    let message = Message::from("@tag1=value1 CMD");

    let mut tags = message.tags()?.into_iter();
    let (key, val) = tags.next().unwrap()?;
    assert_eq!(key, "tag1");
    assert_eq!(val, "value1");
    assert!(tags.next().is_none());

    let message = Message::from("@tag1=value1;tag2=value2 :name CMD :trailing");

    let mut tags = message.tags()?.into_iter();
    let (key, val) = tags.next().unwrap()?;
    assert_eq!(key, "tag1");
    assert_eq!(val, "value1");
    let (key, val) = tags.next().unwrap()?;
    assert_eq!(key, "tag2");
    assert_eq!(val, "value2");
    assert!(tags.next().is_none());

    assert!(message.prefix().unwrap().is_some());

    let message = Message::from("@tag1=value1;tag2=value2 CMD :trailing");

    let mut tags = message.tags()?.into_iter();
    let (key, val) = tags.next().unwrap()?;
    assert_eq!(key, "tag1");
    assert_eq!(val, "value1");
    let (key, val) = tags.next().unwrap()?;
    assert_eq!(key, "tag2");
    assert_eq!(val, "value2");
    assert!(tags.next().is_none());

    assert!(message.prefix().unwrap().is_none());

    Ok(())
}

#[test]
fn test_without_prefix() -> Result<(), Box<dyn Error>> {
    let message = Message::from("CMD param1 param2 :trailing");

    let prefix = message.prefix()?;
    assert!(prefix.is_none());

    assert_eq!(message.command()?, "CMD");

    let mut iter = message.params()?.into_iter();
    assert_eq!(iter.next(), Some("param1"));
    assert_eq!(iter.next(), Some("param2"));
    assert!(iter.next().is_none());

    assert_eq!(message.trailing()?, Some("trailing"));

    Ok(())
}

#[test]
fn test_command_only() -> Result<(), Box<dyn Error>> {
    let message = Message::from("CMD");

    assert!(message.prefix()?.is_none());

    assert_eq!(message.command()?, "CMD");

    assert!(message.params()?.next().is_none());

    Ok(())
}

#[test]
fn test_cmd_and_trailing() -> Result<(), Box<dyn Error>> {
    let message = Message::from("CMD :trailing");

    assert!(message.prefix()?.is_none());

    assert_eq!(message.command()?, "CMD");

    let mut iter = message.params()?;
    assert!(iter.next().is_none());

    assert_eq!(message.trailing()?, Some("trailing"));

    Ok(())
}

#[test]
fn test_cmd_and_param() -> Result<(), Box<dyn Error>> {
    let message = Message::from("CMD param1");

    assert!(message.prefix()?.is_none());

    assert_eq!(message.command()?, "CMD");

    let mut iter = message.params()?;
    assert_eq!(iter.next(), Some("param1"));
    assert!(iter.next().is_none());

    assert!(message.trailing()?.is_none());

    Ok(())
}

#[test]
fn test_prefix() -> Result<(), Box<dyn Error>> {
    let message = Message::from(":name CMD");

    let prefix = message.prefix()?;
    assert!(prefix.is_some());
    let (name, user, host) = prefix.unwrap();
    assert_eq!(name, "name");
    assert!(user.is_none());
    assert!(host.is_none());

    assert_eq!(message.command()?, "CMD");

    let next_param = message.params()?.next();
    assert!(next_param.is_none(), "{:?}", next_param);

    let message = Message::from(":name@host CMD");

    let prefix = message.prefix()?;
    assert!(prefix.is_some());
    let (name, user, host) = prefix.unwrap();
    assert_eq!(name, "name");
    assert!(user.is_none());
    assert_eq!(host, Some("host"));

    assert_eq!(message.command()?, "CMD");

    assert!(message.params()?.next().is_none());

    Ok(())
}

#[test]
fn test_message_builder() -> Result<(), ParserError> {
    let message = Message::builder("CMD")
        .tag("key1", "value1")
        .tag("key2", "value2")
        .prefix("name", Some("user"), Some("host"))
        .param("param1")
        .param("param2")
        .trailing("trailing")
        .build();
    let str = message.to_string();
    assert!(
        str.as_str() == "@key1=value1;key2=value2 :name!user@host CMD param1 param2 :trailing"
            || str.as_str()
                == "@key2=value2;key1=value1 :name!user@host CMD param1 param2 :trailing"
    );

    let message = message
        .to_builder()?
        .tag("key1", "value3")
        .prefix("name1", Some("user"), Some("host"))
        .param("param2")
        .set_param(1, "param3")
        .trailing("other trailing!")
        .build();
    let str = message.to_string();
    assert!(str.as_str() == "@key1=value3;key2=value2 :name1!user@host CMD param1 param3 param2 :other trailing!" || str.as_str() == "@key2=value2;key1=value3 :name1!user@host CMD param1 param3 param2 :other trailing!", "Actual: {}", str.as_str());

    Ok(())
}
