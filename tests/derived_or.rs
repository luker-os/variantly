mod helper;
use helper::{
    TestEnum,
    TestEnum::{Int, OtherUnit, Tuple, Unit},
};

#[test]
fn single_value_tuple() {
    // True True
    assert_eq!(Int(123).or_int(Int(456)).unwrap_int(), 123);

    // True False
    assert_eq!(Int(123).or_int(Unit).unwrap_int(), 123);

    // False True
    assert_eq!(Unit.or_int(Int(123)).unwrap_int(), 123);

    // False False
    assert!(Unit.or_int(OtherUnit).is_other_unit());
}

fn tuple(num: u128) -> TestEnum {
    Tuple(num.to_string(), num)
}
#[test]
fn multi_value_tuple() {
    // True True
    assert_eq!(
        tuple(123).or_tuple(tuple(456)).unwrap_tuple(),
        ("123".into(), 123)
    );

    // True False
    assert_eq!(
        tuple(123).or_tuple(Unit).unwrap_tuple(),
        ("123".into(), 123)
    );

    // False True
    assert_eq!(
        Unit.or_tuple(tuple(123)).unwrap_tuple(),
        ("123".into(), 123)
    );

    // False False
    assert!(Unit.or_tuple(Unit).is_unit());
}
