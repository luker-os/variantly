mod helper;
use helper::{
    TestEnum,
    TestEnum::{Int, OtherUnit, Tuple, Unit},
};

#[test]
fn single_value_tuple() {
    // True True
    assert_eq!(Int(123).and_int(Int(456)).unwrap_int(), 456);

    // True False
    assert_eq!(Int(123).and_int(Unit).unwrap_int(), 123);

    // False True
    assert!(Unit.and_int(Int(123)).is_unit());

    // False False
    assert!(Unit.and_int(OtherUnit).is_unit());
}

fn tuple(num: u128) -> TestEnum {
    Tuple(num.to_string(), num)
}
#[test]
fn multi_value_tuple() {
    // True True
    assert_eq!(
        tuple(123).and_tuple(tuple(456)).unwrap_tuple(),
        ("456".into(), 456)
    );

    // True False
    assert_eq!(
        tuple(123).and_tuple(Unit).unwrap_tuple(),
        ("123".into(), 123)
    );

    // False True
    assert!(Unit.and_tuple(tuple(123)).is_unit());

    // False False
    assert!(Unit.and_tuple(Unit).is_unit());
}
