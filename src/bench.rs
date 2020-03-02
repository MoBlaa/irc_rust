extern crate rand;
extern crate test;

use test::Bencher;

use rand::Rng;

use crate::message::Message;

#[bench]
fn bench_parse(b: &mut Bencher) {
    b.iter(|| {
        let message = Message::from("@key1=value1;key2=value2 :name!user@host CMD param1 param2 :trailing");

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
        assert_eq!(params.trailing.unwrap(), "trailing")
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
    let message = Message::new(str);
    let tags = message.tags().unwrap();

    b.iter(|| {
        let mut rng = rand::thread_rng();
        let ikey = rng.gen_range(0, 1000);
        let skey = format!("key{}", ikey);
        let val = &tags[&skey];
        assert_eq!(val, format!("value{}", ikey));
    })
}