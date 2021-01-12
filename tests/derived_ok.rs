mod helper;
use helper::{
    TestEnum,
    TestEnum::{Int, Unit},
};

#[test]
#[allow(deprecated)] // Test deprecated for back-compat. Remove in 1.0.0 or next pre-stable minor bump.
fn single_value_tuple_deprecated() {
    // Match
    assert_eq!(Int(123).ok_int(), Some(123));

    // Non-Match
    assert_eq!(Unit.ok_int(), None);

    // Shorthand methods

    // Match
    assert_eq!(Int(123).int(), Some(123));

    // Non-Match
    assert_eq!(Unit.int(), None);
}

#[test]
#[allow(deprecated)] // Test deprecated for back-compat. Remove in 1.0.0 or next pre-stable minor bump.
fn multi_value_tuple_deprecated() {
    // Match
    assert_eq!(
        TestEnum::new_tuple(123).ok_tuple(),
        Some(("123".into(), 123))
    );

    // Non-Match
    assert_eq!(Unit.ok_tuple(), None);
}

#[test]
fn single_value_tuple() {
    // Match
    assert_eq!(Int(123).int(), Some(123));

    // Non-Match
    assert_eq!(Unit.int(), None);

    // Shorthand methods

    // Match
    assert_eq!(Int(123).int(), Some(123));

    // Non-Match
    assert_eq!(Unit.int(), None);
}

#[test]
fn multi_value_tuple() {
    // Match
    assert_eq!(TestEnum::new_tuple(123).tuple(), Some(("123".into(), 123)));

    // Non-Match
    assert_eq!(Unit.tuple(), None);
}
