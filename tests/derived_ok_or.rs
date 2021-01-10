mod helper;
use helper::{
    TestEnum,
    TestEnum::{Int, Unit},
};

#[test]
fn single_value_tuple() {
    // Match
    assert_eq!(Int(123).ok_or_int("ERR").unwrap(), 123);

    // Non-Match
    assert_eq!(Unit.ok_or_int("ERR").unwrap_err(), "ERR");
}

#[test]
fn multi_value_tuple() {
    // Match
    assert_eq!(
        TestEnum::new_tuple(123).ok_or_tuple("ERR").unwrap(),
        ("123".into(), 123)
    );

    // Non-Match
    assert_eq!(Unit.ok_or_tuple("ERR").unwrap_err(), "ERR");
}
