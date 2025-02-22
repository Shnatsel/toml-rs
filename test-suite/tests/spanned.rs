extern crate serde;
extern crate toml;
#[macro_use]
extern crate serde_derive;

use std::collections::HashMap;
use std::fmt::Debug;
use toml::value::Datetime;
use toml::Spanned;

/// A set of good datetimes.
pub fn good_datetimes() -> Vec<&'static str> {
    let mut v = Vec::new();
    v.push("1997-09-09T09:09:09Z");
    v.push("1997-09-09T09:09:09+09:09");
    v.push("1997-09-09T09:09:09-09:09");
    v.push("1997-09-09T09:09:09");
    v.push("1997-09-09");
    v.push("09:09:09");
    v.push("1997-09-09T09:09:09.09Z");
    v.push("1997-09-09T09:09:09.09+09:09");
    v.push("1997-09-09T09:09:09.09-09:09");
    v.push("1997-09-09T09:09:09.09");
    v.push("09:09:09.09");
    v
}

#[test]
fn test_spanned_field() {
    #[derive(Deserialize)]
    struct Foo<T> {
        foo: Spanned<T>,
    }

    #[derive(Deserialize)]
    struct BareFoo<T> {
        foo: T,
    }

    fn good<'de, T>(s: &'de str, expected: &str, end: Option<usize>)
    where
        T: serde::Deserialize<'de> + Debug + PartialEq,
    {
        let foo: Foo<T> = toml::from_str(s).unwrap();

        assert_eq!(6, foo.foo.start());
        if let Some(end) = end {
            assert_eq!(end, foo.foo.end());
        } else {
            assert_eq!(s.len(), foo.foo.end());
        }
        assert_eq!(expected, &s[foo.foo.start()..foo.foo.end()]);

        // Test for Spanned<> at the top level
        let foo_outer: Spanned<BareFoo<T>> = toml::from_str(s).unwrap();

        assert_eq!(0, foo_outer.start());
        assert_eq!(s.len(), foo_outer.end());
        assert_eq!(foo.foo.into_inner(), foo_outer.into_inner().foo);
    }

    good::<String>("foo = \"foo\"", "\"foo\"", None);
    good::<u32>("foo = 42", "42", None);
    // leading plus
    good::<u32>("foo = +42", "+42", None);
    // table
    good::<HashMap<String, u32>>(
        "foo = {\"foo\" = 42, \"bar\" = 42}",
        "{\"foo\" = 42, \"bar\" = 42}",
        None,
    );
    // array
    good::<Vec<u32>>("foo = [0, 1, 2, 3, 4]", "[0, 1, 2, 3, 4]", None);
    // datetime
    good::<String>(
        "foo = \"1997-09-09T09:09:09Z\"",
        "\"1997-09-09T09:09:09Z\"",
        None,
    );

    for expected in good_datetimes() {
        let s = format!("foo = {}", expected);
        good::<Datetime>(&s, expected, None);
    }
    // ending at something other than the absolute end
    good::<u32>("foo = 42\nnoise = true", "42", Some(8));
}
