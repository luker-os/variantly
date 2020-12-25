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
    pub fn tuple(num: u128) -> Self {
        Self::Tuple(num.to_string(), num)
    }
}

#[derive(Variantly)]
enum Color {
    RGB(u8, u8, u8),
    HSV(u8, u8, u8),
    Grey(u8),
    FromOutOfSpace,
}

#[test]
fn example() {
     let color = Color::HSV(255, 255, 0);
 
     // boolean helper function for determining variant:
     assert!(color.is_hsv());
     assert!(!color.is_rgb());
 
     // Get inner values:
     let (h, s, v) = color.unwrap_hsv();
     assert_eq!((h, s, v), (255, 255, 000));
 
     // Single values don't require tuple destructuring:
     let color = Color::Grey(128);
     let value = color.unwrap_grey();
     assert_eq!(value, 128);
 
     // Alter inner value, only if hsv:
     let color = Color::HSV(255, 255, 0);
     let color = color.and_then_hsv(|(h, s, _)| (h, s, 255));
     assert_eq!(color.unwrap_hsv(), (255, 255, 255));
 
     // Safely unwrap with a fallback:
     let color = Color::RGB(255, 255, 0);
     let (r, g, b) = color.unwrap_or_rgb((0, 0, 0));
     assert_eq!((r, g, b), (255, 255, 0));
     // Since color is of the HSV variant, the default is not used.
 
     // Safely unwrap using the fallback
     let color = Color::FromOutOfSpace;
     let (r, g, b) = color.unwrap_or_rgb((0, 0, 0));
     assert_eq!((r, g, b), (0, 0, 0));
 
     // Convert into an Option
     let color = Color::RGB(0, 255, 255);
     let optional_rgb = color.ok_rgb();
     assert_eq!(Some((0, 255, 255)), optional_rgb);
 
     // Convert into a Result
     let color = Color::RGB(255, 0, 255);
     let result_rgb = color.ok_or_rgb("Error: This is not an RGB variant!");
     assert_eq!(Ok((255, 0, 255)), result_rgb);
 
     // Operations like this can also use their familiar `_else` versions:
     let color = Color::FromOutOfSpace;
     let result_rgb = color.ok_or_else_rgb(|| Some("This is a computationally expensive error!"));
     assert!(result_rgb.is_err());
}
