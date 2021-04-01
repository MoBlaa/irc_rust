extern crate rand;
extern crate test;

use rand::Rng;
use test::Bencher;

use irc_rust::errors::ParserError;
use irc_rust::params::Parameterized;
use irc_rust::tags::Taggable;
use irc_rust::tokenizer::PartialCfg;
use irc_rust::Message;
use std::collections::{HashMap, HashSet};
use std::iter::FromIterator;

#[bench]
fn bench_full(b: &mut Bencher) {
    // Excluding the allocation of the string
    let str = String::from("@key1=value1;key2=value2 :name!user@host CMD param1 param2 :trailing");
    let raw = str.as_str();
    b.iter(move || {
        let message = Message::from(raw);
        let parsed = message.parsed().unwrap();

        let val = parsed.tag("key1");
        assert_eq!(val, Some("value1"));
        let val = parsed.tag("key2");
        assert_eq!(val, Some("value2"));
        // 189 ns/iter

        let mut tags = parsed.tags();
        let next_is_key1 = |next: Option<(&&str, &&str)>| {
            let (key, val) = next.unwrap();
            *key == "key1" && *val == "value1"
        };
        let next_is_key2 = |next: Option<(&&str, &&str)>| {
            let (key, val) = next.unwrap();
            *key == "key2" && *val == "value2"
        };

        let first = tags.next();
        let second = tags.next();
        assert!(
            (next_is_key1(first) && next_is_key2(second))
                || (next_is_key2(first) && next_is_key1(second))
        );
        // 319 ns/iter

        let prefix = parsed.prefix();
        assert!(prefix.is_some());
        let prefix = prefix.unwrap();

        assert_eq!(prefix.name(), Some("name"));
        assert_eq!(prefix.user(), Some("user"));
        assert_eq!(prefix.host(), Some("host"));
        // 519 ns/iter

        assert_eq!(parsed.command(), Some("CMD"));
        // 585 ns/iter

        let mut iter = parsed.params();
        assert_eq!(iter.next(), Some(&Some("param1")));
        assert_eq!(iter.next(), Some(&Some("param2")));
        assert!(iter.next().is_none());
        assert_eq!(parsed.trailing(), Some("trailing"))
        // 793 ns/iter
    });
}

#[bench]
fn bench_tag_get(b: &mut Bencher) -> Result<(), ParserError> {
    let mut str = String::from("@");
    for i in 0..100 {
        str = format!("{}key{}=value{}", str, i, i);
        if i < 100 - 1 {
            str.push(';');
        }
    }
    str.push_str(" CMD");

    b.iter(|| {
        let mut rng = rand::thread_rng();
        let ikey = rng.gen_range(0, 100);
        let skey = format!("key{}", ikey);

        let message = Message::from(str.as_str());
        let message = message
            .partial(PartialCfg {
                tags: HashSet::from_iter(vec![skey.as_str()]),
                ..Default::default()
            })
            .unwrap();

        let val = message.tag(&skey);
        assert!(val.is_some(), "{:?}", val);
        assert_eq!(val.unwrap(), format!("value{}", ikey));
    });

    Ok(())
}

#[bench]
fn bench_tags_iter_100(b: &mut Bencher) -> Result<(), ParserError> {
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
    b.iter(|| {
        let message = Message::from(str.as_str());
        let tags = message
            .tags()
            .unwrap()
            .into_iter()
            .collect::<Result<Vec<(&str, &str)>, ParserError>>()
            .unwrap();
        for (key, value) in tags.iter() {
            let stored = key_values.get(&key.to_string());
            assert!(
                stored.is_some(),
                "No value for key '{}' found in '{:?}'",
                key,
                tags
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

    b.iter(|| {
        for param in message.params().unwrap() {
            assert_eq!(param, "param")
        }
    });
}
