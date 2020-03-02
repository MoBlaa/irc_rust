extern crate test;

use test::Bencher;

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