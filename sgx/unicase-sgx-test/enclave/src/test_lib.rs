use std::prelude::v1::*;
use unicase::UniCase;
use std::hash::{Hash, Hasher};
use std::collections::hash_map::DefaultHasher;

fn hash<T: Hash>(t: &T) -> u64 {
    let mut s = DefaultHasher::new();
    t.hash(&mut s);
    s.finish()
}

//#[test]
pub fn test_copy_for_refs() {
    fn foo<T>(_: UniCase<T>) {}

    let a = UniCase::new("foobar");
    foo(a);
    foo(a);
}

//#[test]
pub fn test_eq_ascii() {
    let a = UniCase::new("foobar");
    let b = UniCase::new("FOOBAR");
    let c = UniCase::ascii("FoObAr");

    assert_eq!(a, b);
    assert_eq!(a, c);
    assert_eq!(hash(&a), hash(&b));
    assert_eq!(hash(&a), hash(&c));
    assert!(a.is_ascii());
    assert!(b.is_ascii());
    assert!(c.is_ascii());
}

//#[test]
pub fn test_eq_unicode() {
    let a = UniCase::new("στιγμας");
    let b = UniCase::new("στιγμασ");
    assert_eq!(a, b);
    assert_eq!(hash(&a), hash(&b));
}

//#[cfg(__unicase__iter_cmp)]
//#[test]
pub fn test_case_cmp() {
    assert!(UniCase::new("a") < UniCase::new("B"));

    assert!(UniCase::new("A") < UniCase::new("b"));
    assert!(UniCase::new("aa") > UniCase::new("a"));

    assert!(UniCase::new("a") < UniCase::new("aa"));
    assert!(UniCase::new("a") < UniCase::new("AA"));
}

//#[test]
pub fn test_from_impls() {
    let view: &'static str = "foobar";
    let _: UniCase<&'static str> = view.into();
    let _: UniCase<&str> = view.into();
    let _: UniCase<String> = view.into();

    let owned: String = view.to_owned();
    let _: UniCase<&str> = (&owned).into();
    let _: UniCase<String> = owned.into();
}

//#[test]
pub fn test_into_impls() {
    let view: UniCase<&'static str> = UniCase::new("foobar");
    let _: &'static str = view.into();
    let _: &str = view.into();

    let owned: UniCase<String> = "foobar".into();
    let _: String = owned.clone().into();
    let _: &str = owned.as_ref();
}

//#[cfg(__unicase__const_fns)]
//#[test]
pub fn test_unicase_unicode_const() {
    const _UNICASE: UniCase<&'static str> = UniCase::unicode("");
}
