mod helper;
use helper::{
    TestEnum,
    TestEnum::{Int, Unit},
};
#[test]
fn single_value_tuple() {
    // Match
    assert_eq!(Int(123).int_mut_or("ERR").unwrap(), &mut 123);

    // Non-Match
    assert_eq!(Unit.int_mut_or("ERR").unwrap_err(), "ERR");
}

#[test]
fn multi_value_tuple() {
    // Match
    assert_eq!(
        TestEnum::new_tuple(123).tuple_mut_or("ERR").unwrap(),
        (&mut "123".into(), &mut 123)
    );

    // Non-Match
    assert_eq!(Unit.tuple_mut_or("ERR").unwrap_err(), "ERR");
}
