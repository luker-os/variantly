use variantly::Variantly;

#[derive(Variantly)]
enum TestEnum {
    A,
    B(String),
    C(String),
    D(u128),
    #[allow(dead_code)]
    // Included to validate that struct-like enums variants do not cause compilation issues.
    E {
        value: u128,
    },
    #[allow(dead_code)]
    // Included to validate that multi value tuple enum variants do not cause compilation issues.
    F(String, u128, String), // TODO, shouldn't this be more supported?
}

#[test]
fn it_derives_working_and() {
    let and = TestEnum::B("Value 1".into()).and_b(TestEnum::B("Value 2".into()));
    assert_eq!(and.unwrap_b(), "Value 2");

    let and = TestEnum::B("Test".into()).and_b(TestEnum::A);
    assert_eq!(and.unwrap_b(), "Test");

    let and = TestEnum::A.and_b(TestEnum::B("Test".into()));
    assert!(and.is_a());

    let and = TestEnum::A.and_b(TestEnum::C("Test".into()));
    assert!(and.is_a());
}

#[test]
fn it_derives_working_and_then() {
    let enum_ = TestEnum::B("Hello".into());
    let enum_ = enum_.and_then_b(|inner| format!("{} World!", inner));

    assert_eq!(enum_.unwrap_b(), "Hello World!")
}

#[test]
fn it_derives_working_expect() {
    let enum_ = TestEnum::B("Test".into());
    let value = enum_.expect_b("This shouldn't fail");

    assert_eq!(value, "Test")
}

#[test]
#[should_panic(expected = "Incorrect Enum Variant")]
fn it_derives_working_expect_that_panics() {
    let enum_ = TestEnum::B("Test".into());
    enum_.expect_c("Incorrect Enum Variant");
}

#[test]
fn it_derives_working_is_fn_for_unit_variant() {
    let enum_ = TestEnum::A;
    assert!(enum_.is_a());
    assert!(!enum_.is_b());
    assert!(!enum_.is_c());
    assert!(!enum_.is_d());
    assert!(!enum_.is_e());
}

#[test]
fn it_derives_working_is_fn_for_nameless_single_value_variant() {
    let enum_ = TestEnum::B("Test".into());
    assert!(!enum_.is_a());
    assert!(enum_.is_b());
    assert!(!enum_.is_c());
    assert!(!enum_.is_d());
    assert!(!enum_.is_e());
}

#[test]
fn it_derives_working_is_fn_for_named_value_variant() {
    let enum_ = TestEnum::E { value: 0 };
    assert!(!enum_.is_a());
    assert!(!enum_.is_b());
    assert!(!enum_.is_c());
    assert!(!enum_.is_d());
    assert!(enum_.is_e());
}

#[test]
fn it_derives_working_is_not_fn_for_unit_variant() {
    let enum_ = TestEnum::A;
    assert!(!enum_.is_not_a());
    assert!(enum_.is_not_b());
    assert!(enum_.is_not_c());
    assert!(enum_.is_not_d());
    assert!(enum_.is_not_e());
}

#[test]
fn it_derives_working_is_not_fn_for_nameless_single_value_variant() {
    let enum_ = TestEnum::B("Test".into());
    assert!(enum_.is_not_a());
    assert!(!enum_.is_not_b());
    assert!(enum_.is_not_c());
    assert!(enum_.is_not_d());
    assert!(enum_.is_not_e());
}

#[test]
fn it_derives_working_is_not_fn_for_named_value_variant() {
    let enum_ = TestEnum::E { value: 0 };
    assert!(enum_.is_not_a());
    assert!(enum_.is_not_b());
    assert!(enum_.is_not_c());
    assert!(enum_.is_not_d());
    assert!(!enum_.is_not_e());
}

#[test]
fn it_derives_working_ok() {
    let enum_1 = TestEnum::B("Test".into());
    let enum_2 = TestEnum::A;

    assert_eq!(enum_1.ok_b(), Some("Test".into()));
    assert_eq!(enum_2.ok_b(), None);
}

#[test]
fn it_derives_working_ok_or() {
    let enum_1 = TestEnum::D(123);
    let enum_2 = TestEnum::B("Test".into());
    let err = "Wrong Variant Provided";

    let result_1 = enum_1.ok_or_d(err);
    let result_2 = enum_2.ok_or_d(err);

    assert!(result_1.is_ok());
    assert_eq!(result_1.unwrap(), 123);

    assert!(result_2.is_err());
    assert_eq!(result_2.unwrap_err(), "Wrong Variant Provided");
}

#[test]
fn it_derives_working_ok_or_else() {
    let enum_1 = TestEnum::D(123);
    let enum_2 = TestEnum::B("Test".into());
    let err = || "Wrong Variant Provided";

    let result_1 = enum_1.ok_or_else_d(err);
    let result_2 = enum_2.ok_or_else_d(err);

    assert!(result_1.is_ok());
    assert_eq!(result_1.unwrap(), 123);

    assert!(result_2.is_err());
    assert_eq!(result_2.unwrap_err(), "Wrong Variant Provided");
}

#[test]
fn it_derives_working_or() {
    let or = TestEnum::D(123).or_d(TestEnum::D(456));
    assert_eq!(or.unwrap_d(), 123);

    let or = TestEnum::D(123).or_d(TestEnum::B("Test".into()));
    assert_eq!(or.unwrap_d(), 123);

    let or = TestEnum::B("Test".into()).or_d(TestEnum::D(456));
    assert_eq!(or.unwrap_d(), 456);

    let or = TestEnum::B("Test".into()).or_d(TestEnum::C("Other".into()));
    assert_eq!(or.unwrap_c(), "Other");
}

#[test]
fn it_derives_working_or_else() {
    let or_else = || 456;

    let enum_ = TestEnum::D(123).or_else_d(or_else);
    assert_eq!(enum_.unwrap_d(), 123);

    let enum_ = TestEnum::C("Test".into()).or_else_d(or_else);
    assert_eq!(enum_.unwrap_d(), 456);
}

#[test]
fn it_derives_working_or_replace() {
    let mut enum_ = TestEnum::D(123);
    let original = enum_.replace_d(456);
    assert_eq!(enum_.unwrap_d(), 456);
    assert_eq!(original.unwrap_d(), 123);

    let mut enum_ = TestEnum::C("Test".into());
    let failed_replace = enum_.replace_d(456);
    assert_eq!(enum_.unwrap_c(), "Test");
    assert_eq!(failed_replace.unwrap_d(), 456);
}

#[test]
fn it_derives_working_or_unwrap() {
    assert_eq!(TestEnum::C("Test".into()).unwrap_c(), "Test")
}

#[test]
#[should_panic]
fn it_derives_working_or_unwrap_that_panics() {
    TestEnum::A.unwrap_b();
}

#[test]
fn it_derives_working_unwrap_or() {
    assert_eq!(
        TestEnum::C("Test".into()).unwrap_or_c("Default".into()),
        "Test"
    );

    assert_eq!(TestEnum::D(123).unwrap_or_c("Default".into()), "Default");
}

#[test]
fn it_derives_working_unwrap_or_else() {
    let or_else = || String::from("Expensive Default");
    assert_eq!(TestEnum::C("Test".into()).unwrap_or_else_c(or_else), "Test");

    assert_eq!(
        TestEnum::D(123).unwrap_or_else_c(or_else),
        "Expensive Default"
    );
}
