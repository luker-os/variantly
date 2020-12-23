mod helper;
use helper::{
    TestEnum,
    TestEnum::{Int, Unit},
};

#[test]
fn single_value_tuple() {
    assert_eq!(Int(123).unwrap_int(), 123);
}

#[test]
#[should_panic]
fn single_value_tuple_panic() {
    Unit.unwrap_int();
}

#[test]
fn multi_value_tuple() {
    assert_eq!(TestEnum::tuple(123).unwrap_tuple(), ("123".into(), 123));
}

#[test]
#[should_panic]
fn multi_value_tuple_panic() {
    Unit.unwrap_tuple();
}
