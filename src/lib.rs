//! Derive helper methods for enum variants that are familiar from `std::option::Option` & `std::result::Result` such as `unwrap_or` or `and_then`.
//! # Example
//! ```ignore, no_run
//! #[derive(Variantly)]
//! enum Color {
//!     RGB(u8, u8, u8),
//!     HSV(u8, u8, u8),
//!     Grey(u8),
//!     FromOutOfSpace,
//!     #[variantly(rename = "darkness")]
//!     Black,
//! }
//!
//! fn example() {
//!     let color = Color::HSV(123, 45, 67);
//!
//!     // boolean helper function for determining variant:
//!     assert!(color.is_hsv());
//!     assert!(!color.is_rgb());
//!
//!     // Get inner values:
//!     let (h, s, v) = color.unwrap_hsv();
//!     assert_eq!((h, s, v), (123, 45, 67));
//!
//!     // Single values don't require tuple destructuring:
//!     let color = Color::Grey(128);
//!     let value = color.unwrap_grey();
//!     assert_eq!(value, 128);
//!
//!     // Alter inner value, only if hsv:
//!     let color = Color::HSV(111, 22, 33);
//!     let color = color.and_then_hsv(|(h, s, _)| (h, s, 100));
//!     assert_eq!(color.unwrap_hsv(), (111, 22, 100));
//!
//!     // Safely unwrap with a fallback:
//!     let color = Color::RGB(255, 255, 0);
//!     let (r, g, b) = color.unwrap_or_rgb((0, 0, 0));
//!     assert_eq!((r, g, b), (255, 255, 0));
//!     // Since color is of the HSV variant, the default is not used.
//!
//!     // Safely unwrap using the fallback
//!     let color = Color::FromOutOfSpace;
//!     let (r, g, b) = color.unwrap_or_rgb((0, 0, 0));
//!     assert_eq!((r, g, b), (0, 0, 0));
//!
//!     // Convert into an Option
//!     let color = Color::RGB(0, 255, 255);
//!     let optional_rgb = color.ok_rgb();
//!     assert_eq!(Some((0, 255, 255)), optional_rgb);
//!
//!     // Convert into a Result
//!     let color = Color::RGB(255, 0, 255);
//!     let result_rgb = color.ok_or_rgb("Error: This is not an RGB variant!");
//!     assert_eq!(Ok((255, 0, 255)), result_rgb);
//!
//!     // Operations like this can also use their familiar `_else` versions:
//!     let color = Color::FromOutOfSpace;
//!     let result_rgb = color.ok_or_else_rgb(|| Some("This is a computationally expensive error!"));
//!     assert!(result_rgb.is_err());
//!
//!     // The `#[variantly(rename = "darkness")]` attribute renames methods.
//!     let color = Color::Black;
//!     assert!(color.is_darkness())
//! }
//! ```
//! # Derived functions
//! The following are supported function types that are derived for all enum variants:
//! 1. `is`
//! 1. `is_not`
//!
//! The following are supported function types that are derived for tuple-like structs. This includes structs that hold one or many tuple style values.
//! 1. `and`
//! 1. `and_then`
//! 1. `expect`
//! 1. `ok`
//! 1. `ok_or`
//! 1. `ok_or_else`
//! 1. `or`
//! 1. `unwrap`
//! 1. `unwrap_or`
//! 1. `unwrap_or_else`
//!
//! # Derived function naming
//! Derived functions are named by parsing and combining the enum variant they correspond with and the name of the operation they perform.
//! Simplified, this looks like:
//! ```
//! use inflector::cases::snakecase::to_snake_case;
//!
//! fn name_fn(operation: &str, variant_name: &str) -> String {
//!     let snake_case_variant = to_snake_case(&variant_name);
//!     format!("{}_{}", operation, snake_case_variant)
//! }
//!
//! assert_eq!(
//!     name_fn("unwrap","VariantA"),
//!     String::from("unwrap_variant_a")
//! )
//! ```
//!
//! # Renaming Methods
//! The `varianty` attribute may be placed on a variant in order to customize the resulting method names.
//! ```ignore, no_run
//! #[derive(Variantly)]
//! enum SomeEnum {
//!     #[variantly(rename = "variant_a")]
//!     SomeVariantWithALongName(String),
//!     VariantB,
//! }
//! ```
//! Functions associated with `SomeVariantWithALongName` will now be accessible only with the `variant_a`
//! suffix, such as `.unwrap_or_else_variant_a()`. This can help control overly verbose fn names.
//! Note that the input to `rename` is used as is and is not coerced into snake_case.
//!
//!
//! The above is also relevant when two variant names would expand to create conflicting function names:
//! ```ignore, no_run
//! #[derive(Variantly)]
//! enum SomeEnum {
//!     #[variantly(rename = "capital")]
//!     ABC,
//!     #[variantly(rename = "lower")]
//!     abc,
//! }
//! ```
//! Without the `rename` attribute in the above, both variants would create conflicting functions such as `.is_abc()` due to the coercion to snake_case.
//! This is avoided by using the rename input to create meaningful and unique fn names.
//!

#[macro_use]
extern crate darling;
extern crate proc_macro;

#[macro_use]
mod idents;

mod derive;
mod error;
mod input;

use derive::derive_variantly_fns;
use proc_macro::TokenStream;
use syn::{parse_macro_input, ItemEnum};

/// The `Variantly` derive macro. See [the module level documentation](self) for more.
#[proc_macro_derive(Variantly, attributes(variantly))]
pub fn variantly(input: TokenStream) -> TokenStream {
    let item_enum = parse_macro_input!(input as ItemEnum);
    derive_variantly_fns(item_enum).unwrap_or_else(|err| err.to_compile_error())
}
