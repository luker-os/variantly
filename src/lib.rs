//! Derive associated functions for enums that are familiar from `std::option::Option` & `std::result::Result`.
//! # Example
//! ```ignore, no_run
//! #[derive(Variantly)]
//! enum Color {
//!     RGB(u8, u8, u8),
//!     HSV(u8, u8, u8),
//!     Grey(u8),
//!     FromOutOfSpace,
//! }
//!
//! fn example() {
//!     let color = Color::HSV(255, 255, 0);
//!
//!     // boolean helper function for determining variant:
//!     assert!(color.is_hsv());
//!     assert!(!color.is_rgb());
//!
//!     // Get inner values:
//!     let (h, s, v) = color.unwrap_hsv();
//!     assert_eq!((h, s, v), (255, 255, 000));
//!
//!     // Single values don't require tuple destructuring:
//!     let color = Color::Grey(128);
//!     let value = color.unwrap_grey();
//!     assert_eq!(value, 128);
//!
//!     // Alter inner value, only if hsv:
//!     let color = Color::HSV(255, 255, 0);
//!     let color = color.and_then_hsv(|(h, s, _)| (h, s, 255));
//!     assert_eq!(color.unwrap_hsv(), (255, 255, 255));
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
//! }
//! ```
//! # Derived functions
//! The following are supported function types that are derived for all enum variants:
//! 1. `is`
//! 1. `is_not`
//!
//! The following are supported function types that are derived for tuple-like structs. This includes structs that hold one or many unnamed values.
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
//! fn name_fn(operation: String, variant_name: String) -> String {
//!     let snake_case_variant = to_snake_case(&variant_name);
//!     format!("{}_{}", snake_case_variant, operation)
//! }
//!
//! #[test]
//!fn it_makes_a_name() {
//!     assert_eq!(
//!         name_fn("unwrap","VariantA"),
//!         "unwrap_variant_a".into()
//!     )
//! }
//! ```
//!
extern crate proc_macro;

#[macro_use]
mod idents;

mod derive;

use derive::derive_variantly_fns;
use proc_macro::TokenStream;
use syn::{parse_macro_input, ItemEnum};

/// The `Variantly` derive macro. See [the module level documentation](self) for more.
#[proc_macro_derive(Variantly)]
pub fn variantly(input: TokenStream) -> TokenStream {
    let item_enum = parse_macro_input!(input as ItemEnum);
    match derive_variantly_fns(item_enum) {
        Ok(ok) => ok,
        Err(err) => err.to_compile_error().into(),
    }
}
