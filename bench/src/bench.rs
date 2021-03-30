extern crate rand;
extern crate test;

use rand::Rng;
use test::Bencher;

use irc_rust::{InvalidIrcFormatError, Message, Parameterized, Params, Prefixed, Tags};
use std::collections::HashMap;
use std::convert::TryFrom;

#[bench]
fn bench_full(b: &mut Bencher) {
    // Excluding the allocation of the string
    let str = String::from("@key1=value1;key2=value2 :name!user@host CMD param1 param2 :trailing");
    let raw = str.as_str();
    b.iter(move || {
        let message = Message::from(raw);

        let tags = message.tags();
        assert!(tags.is_ok(), "{:?}", tags);
        let tags = tags.unwrap();
        assert!(tags.is_some());
        let tags = tags.unwrap();

        let val = tags.get("key1");
        assert_eq!(val, Some("value1"));
        let val = tags.get("key2");
        assert_eq!(val, Some("value2"));
        // 189 ns/iter

        let mut tags = tags.iter();
        let next_is_key1 = |next: Option<(&str, &str)>| {
            let (key, val) = next.unwrap();
            key == "key1" && val == "value1"
        };
        let next_is_key2 = |next: Option<(&str, &str)>| {
            let (key, val) = next.unwrap();
            key == "key2" && val == "value2"
        };

        let first = tags.next();
        let second = tags.next();
        assert!(
            (next_is_key1(first) && next_is_key2(second))
                || (next_is_key2(first) && next_is_key1(second))
        );
        // 319 ns/iter

        let prefix = message.prefix();
        assert!(prefix.is_ok());
        let prefix = prefix.unwrap();
        assert!(prefix.is_some());
        let prefix = prefix.unwrap();

        assert_eq!(prefix.name(), "name");
        assert_eq!(prefix.user(), Some("user"));
        assert_eq!(prefix.host(), Some("host"));
        // 519 ns/iter

        assert_eq!(message.command(), "CMD");
        // 585 ns/iter

        let params = message.params().unwrap();
        let mut iter = params.iter();
        assert_eq!(iter.next().unwrap(), "param1");
        assert_eq!(iter.next().unwrap(), "param2");
        assert!(iter.next().is_none());
        assert_eq!(params.trailing().unwrap(), "trailing")
        // 793 ns/iter
    });
}

#[bench]
fn bench_tag_create(b: &mut Bencher) {
    let mut str = String::new();
    for i in 0..20 {
        str = format!("{}key{}=value{}", str, i, i);
        if i != 1000 {
            str.push(';');
        }
    }

    b.iter(|| {
        let tags = Tags::try_from(str.as_str());
        assert!(tags.is_ok(), "{:?}", tags);
        let tags = tags.unwrap();
        assert_eq!(tags.as_ref(), str.as_str());
    })
}

#[bench]
fn bench_tag_get(b: &mut Bencher) -> Result<(), InvalidIrcFormatError> {
    let mut str = String::from("");
    for i in 0..100 {
        str = format!("{}key{}=value{}", str, i, i);
        if i < 100 - 1 {
            str.push(';');
        }
    }
    let tags = Tags::try_from(str.as_str())?;

    b.iter(|| {
        let mut rng = rand::thread_rng();
        let ikey = rng.gen_range(0, 100);
        let skey = format!("key{}", ikey);
        let val = tags.get(&skey);
        assert!(val.is_some(), "{:?}", val);
        assert_eq!(val.unwrap(), format!("value{}", ikey));
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
    let tags = Tags::try_from(str.as_str())?;

    b.iter(|| {
        for (key, value) in tags.iter() {
            let stored = key_values.get(key);
            assert!(
                stored.is_some(),
                "No value for key '{}' found in '{}'",
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
        for param in message.params().unwrap().iter() {
            assert_eq!(param, "param")
        }
    });
}

#[bench]
fn bench_params_create(b: &mut Bencher) {
    let mut str = String::new();
    for _ in 0..100 {
        str.push_str(" param");
    }

    b.iter(|| {
        let params = Params::from(str.as_str());
        assert_eq!(params.as_ref(), str.as_str());
    });
}
