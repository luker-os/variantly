use variantly::Variantly;
#[derive(Variantly)]
pub enum TestEnum {
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
    F(String, u128, String),
}