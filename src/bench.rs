extern crate rand;
extern crate test;

use test::Bencher;

use rand::Rng;

use crate::message::Message;
use crate::{Params, Tags};
use std::convert::TryFrom;

#[bench]
fn bench_parse(b: &mut Bencher) {
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

        let val = &tags["key1"];
        assert_eq!(val, "value1");
        let val = &tags["key2"];
        assert_eq!(val, "value2");
        // 189 ns/iter

        let mut tags = tags.iter();
        let (key, val) = tags.next().unwrap();
        assert_eq!(key, "key1");
        assert_eq!(val, "value1");
        let (key, val) = tags.next().unwrap();
        assert_eq!(key, "key2");
        assert_eq!(val, "value2");
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
    for i in 0..1000 {
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
fn bench_tag_index(b: &mut Bencher) {
    let mut str = String::from("@");
    for i in 0..1000 {
        str = format!("{}key{}=value{}", str, i, i);
        if i != 1000 {
            str.push(';');
        }
    }
    str.push_str(" CMD");
    let message = Message::from(str);
    let tags = message.tags().unwrap().unwrap();

    b.iter(|| {
        let mut rng = rand::thread_rng();
        let ikey = rng.gen_range(0, 1000);
        let skey = format!("key{}", ikey);
        let val = &tags[&skey];
        assert_eq!(val, format!("value{}", ikey));
    })
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
