mod helper;
use helper::{
    TestEnum,
    TestEnum::{Int, Unit},
};

#[test]
fn single_value_tuple() {
    // Match
    assert_eq!(Int(123).or_else_int(|| 456).unwrap_int(), 123);

    // Non-Match
    assert_eq!(Unit.or_else_int(|| 456).unwrap_int(), 456);
}

#[test]
fn multi_value_tuple() {
    // Match
    assert_eq!(
        TestEnum::new_tuple(123).unwrap_or_else_tuple(|| ("456".into(), 456)),
        ("123".into(), 123)
    );

    // Non-Match
    assert_eq!(
        Unit.unwrap_or_else_tuple(|| ("456".into(), 456)),
        ("456".into(), 456)
    );
}
