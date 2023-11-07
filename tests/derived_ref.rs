mod helper;
use helper::{
    TestEnum,
    TestEnum::{Int, Unit},
};

#[test]
fn single_value_tuple() {
    // Match
    assert_eq!(Int(123).int_ref(), Some(&123));

    // Non-Match
    assert_eq!(Unit.int_ref(), None);
}

#[test]
fn multi_value_tuple() {
    // Match
    assert_eq!(
        TestEnum::new_tuple(123).tuple_ref(),
        Some((&"123".into(), &123))
    );

    // Non-Match
    assert_eq!(Unit.tuple_ref(), None);
}
