mod helper;
use helper::{
    TestEnum,
    TestEnum::{Int, Unit},
};

#[test]
fn single_value_tuple() {
    let and_then = |val| val + 100;

    // Match
    assert_eq!(Int(123).and_then_int(and_then).unwrap_int(), 223);

    // Non-Match
    assert!(Unit.and_then_int(and_then).is_unit());
}

#[test]
fn multi_value_tuple() {
    let and_then = |(word, num)| (format!("{}{}", word, word), num + num);

    // Match
    assert_eq!(
        TestEnum::tuple(123).and_then_tuple(and_then).unwrap_tuple(),
        ("123123".into(), 246)
    );

    // Non-Match
    assert!(Unit.and_then_tuple(and_then).is_unit());
}
