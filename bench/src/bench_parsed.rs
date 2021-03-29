extern crate rand;
extern crate test;

use self::rand::Rng;
use irc_rust::{InvalidIrcFormatError, Message, Parameterized, Parsed, Taggable};
use std::collections::HashMap;
use std::convert::TryFrom;
use test::Bencher;

#[bench]
fn bench_full_parsed(b: &mut Bencher) {
    // Excluding the allocation of the string
    // TODO: Generate String more randomly but with realistic values
    let str = String::from("@key1=value1;key2=value2 :name!user@host CMD param1 param2 :trailing");
    let raw = str.as_str();
    b.iter(move || {
        let message = Message::from(raw);
        let parsed = message.parsed().unwrap();

        let val = parsed.tag("key1");
        assert!(val.is_some());
        assert_eq!(val.unwrap(), "value1");
        let val = parsed.tag("key2");
        assert!(val.is_some());
        assert_eq!(val.unwrap(), "value2");

        let mut tags = parsed.tags();
        let next_is_key1 = |next: Option<(&&str, &&str)>| {
            let (key, val) = next.unwrap();
            key == &"key1" && val == &"value1"
        };
        let next_is_key2 = |next: Option<(&&str, &&str)>| {
            let (key, val) = next.unwrap();
            key == &"key2" && val == &"value2"
        };

        let first = tags.next();
        let second = tags.next();
        assert!(
            (next_is_key1(first) && next_is_key2(second))
                || (next_is_key2(first) && next_is_key1(second))
        );

        let prefix = parsed.prefix();
        assert!(prefix.is_some());
        let prefix = prefix.unwrap();
        assert_eq!(prefix.0, Some("name"));
        assert_eq!(prefix.1, Some("user"));
        assert_eq!(prefix.2, Some("host"));

        assert_eq!(parsed.command(), Some("CMD"));

        let mut iter = parsed.params();
        assert_eq!(iter.next().unwrap(), &Some("param1"));
        assert_eq!(iter.next().unwrap(), &Some("param2"));
        assert!(iter.next().is_none());
        let trailing = parsed.trailing();
        assert!(trailing.is_some());
        assert_eq!(trailing.unwrap(), "trailing")
    });
}

#[bench]
fn bench_tag_get(b: &mut Bencher) -> Result<(), InvalidIrcFormatError> {
    let mut str = String::from("@");
    for i in 0..1000 {
        str = format!("{}key{}=value{}", str, i, i);
        if i != 1000 {
            str.push(';');
        }
    }
    str.push_str(" CMD");
    let message = Message::from(str);
    let parsed = Parsed::try_from(&message)?;

    b.iter(|| {
        let mut rng = rand::thread_rng();
        let ikey = rng.gen_range(0, 1000);
        let skey = format!("key{}", ikey);
        let val = parsed.tag(&skey.as_str());
        assert!(val.is_some(), "No value for key '{}'", skey);
        assert_eq!(val.unwrap(), &format!("value{}", ikey));
    });

    Ok(())
}

#[bench]
fn bench_tags_iter_100(b: &mut Bencher) -> Result<(), InvalidIrcFormatError> {
    let mut str = String::from("");
    let mut key_values = HashMap::new();
    for i in 0..100 {
        let key = format!("key{}", i);
        let value = format!("value{}", i);
        key_values.insert(key.clone(), value.clone());

        str = format!("{}{}={}", str, key, value);
        if i < 100 - 1 {
            str.push(';');
        }
    }
    str.push_str(" CMD");
    let message = Message::from(str);
    let parsed = Parsed::try_from(&message)?;

    b.iter(|| {
        for (key, value) in parsed.tags() {
            let stored = key_values.get(*key);
            assert!(
                stored.is_some(),
                "No value for key '{}' found in '{}'",
                key,
                message
            );
            assert_eq!(stored.unwrap(), value);
        }
    });

    Ok(())
}

#[bench]
fn bench_params_iter(b: &mut Bencher) {
    let mut str = String::from("CMD");
    for _ in 0..100 {
        str.push_str(" param");
    }
    let message = Message::from(str);
    let parsed = message.parsed().unwrap();

    b.iter(|| {
        for param in parsed.params() {
            assert_eq!(param, &Some("param"))
        }
    });
}
