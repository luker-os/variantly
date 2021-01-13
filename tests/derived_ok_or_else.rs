mod helper;
use helper::{
    TestEnum,
    TestEnum::{Int, Unit},
};

#[test]
#[allow(deprecated)] // Test deprecated functions for back-compat. Remove in 1.0.0 or next pre-stable minor bump.
fn single_value_tuple_deprecated() {
    // Match
    assert_eq!(Int(123).ok_or_else_int(|| "ERR").unwrap(), 123);

    // Non-Match
    assert_eq!(Unit.ok_or_else_int(|| "ERR").unwrap_err(), "ERR");
}

#[test]
#[allow(deprecated)] // Test deprecated functions for back-compat. Remove in 1.0.0 or next pre-stable minor bump.
fn multi_value_tuple_deprecated() {
    // Match
    assert_eq!(
        TestEnum::new_tuple(123).ok_or_else_tuple(|| "ERR").unwrap(),
        ("123".into(), 123)
    );

    // Non-Match
    assert_eq!(Unit.ok_or_else_tuple(|| "ERR").unwrap_err(), "ERR");
}

#[test]
fn single_value_tuple() {
    // Match
    assert_eq!(Int(123).int_or_else(|| "ERR").unwrap(), 123);

    // Non-Match
    assert_eq!(Unit.int_or_else(|| "ERR").unwrap_err(), "ERR");
}

#[test]
fn multi_value_tuple() {
    // Match
    assert_eq!(
        TestEnum::new_tuple(123).tuple_or_else(|| "ERR").unwrap(),
        ("123".into(), 123)
    );

    // Non-Match
    assert_eq!(Unit.tuple_or_else(|| "ERR").unwrap_err(), "ERR");
}
