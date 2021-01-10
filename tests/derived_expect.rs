mod helper;
use helper::{
    TestEnum,
    TestEnum::{Int, Unit},
};

#[test]
fn single_value_tuple() {
    assert_eq!(Int(123).expect_int("This should have been an int"), 123);
}

#[test]
#[should_panic(expected = "This should have been an int")]
fn single_value_tuple_panic() {
    Unit.expect_int("This should have been an int");
}

#[test]
fn multi_value_tuple() {
    assert_eq!(
        TestEnum::new_tuple(123).expect_tuple("This should have been a tuple"),
        ("123".into(), 123)
    );
}

#[test]
#[should_panic(expected = "This should have been a tuple")]
fn multi_value_tuple_panic() {
    Unit.expect_tuple("This should have been a tuple");
}
