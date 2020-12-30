mod helper;
use helper::{
    TestEnum,
    TestEnum::{StructLike, Tuple, Unit},
};

#[test]
fn unit() {
    let enm = Unit;
    assert!(enm.is_unit());
    assert!(!enm.is_not_unit());

    assert!(enm.is_not_other_unit());
    assert!(!enm.is_other_unit());

    assert!(enm.is_not_string());
    assert!(!enm.is_string());

    assert!(enm.is_not_tuple());
    assert!(!enm.is_tuple());

    assert!(enm.is_not_struct_like());
    assert!(!enm.is_struct_like());
}

#[test]
fn tuple() {
    let enm = Tuple("Test".into(), 123);
    assert!(enm.is_tuple());
    assert!(!enm.is_not_tuple());

    assert!(enm.is_not_unit());
    assert!(!enm.is_unit());

    assert!(enm.is_not_other_unit());
    assert!(!enm.is_other_unit());

    assert!(enm.is_not_string());
    assert!(!enm.is_string());

    assert!(enm.is_not_struct_like());
    assert!(!enm.is_struct_like());
}

#[test]
fn struct_like() {
    let enm: TestEnum = StructLike { value: 123 };
    assert!(enm.is_struct_like());
    assert!(!enm.is_not_struct_like());

    assert!(enm.is_not_unit());
    assert!(!enm.is_unit());

    assert!(enm.is_not_other_unit());
    assert!(!enm.is_other_unit());

    assert!(enm.is_not_string());
    assert!(!enm.is_string());

    assert!(enm.is_not_tuple());
    assert!(!enm.is_tuple());
}
