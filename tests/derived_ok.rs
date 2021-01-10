mod helper;
use helper::{
    TestEnum,
    TestEnum::{Int, Unit},
};

#[test]
fn single_value_tuple() {
    // Match
    assert_eq!(Int(123).ok_int(), Some(123));

    // Non-Match
    assert_eq!(Unit.ok_int(), None);
}

#[test]
fn multi_value_tuple() {
    // Match
    assert_eq!(
        TestEnum::new_tuple(123).ok_tuple(),
        Some(("123".into(), 123))
    );

    // Non-Match
    assert_eq!(Unit.ok_tuple(), None);
}
