#![allow(dead_code)]

use variantly::Variantly;

/// Validate that complex enum variants can validly expand.
#[derive(Variantly)]
pub enum ComplexEnum<'a, A, B>
where
    B: Fn() -> String,
{
    One((((), ()), ()), ((), ())),
    Two(A, B),
    Three(&'a ComplexEnum<'a, A, B>),
    Four {
        first: &'a ComplexEnum<'a, String, B>,
        second: &'static str,
    },
}

#[derive(Variantly, Clone)]
pub enum TestEnum {
    Unit,
    OtherUnit,
    String(String),
    Int(u128),
    Tuple(String, u128),
    StructLike { value: u128 },
}

impl TestEnum {
    pub fn new_tuple(num: u128) -> Self {
        Self::Tuple(num.to_string(), num)
    }
}
