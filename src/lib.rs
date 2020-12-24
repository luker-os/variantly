//! Derive associated functions for enums that are familiar from `std::option::Option` & `std::result::Result`.
//! # Example
//! ```ignore, no_run
//! #[derive(Variantly, PartialEq, Debug)]
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
//!     // Determine variant type with boolean:
//!     assert!(color.is_hsv());
//!     assert!(!color.is_rgb());
//!
//!     // Alter inner value, only if rgba:
//!     let color = color.and_then_hsv(|(h, s, _)| (h, s, 255));
//!     assert_eq!(color, Color::HSV(255, 255, 255));
//!
//!     // Get inner values:
//!     let (h, s, v) = color.unwrap_hsv();
//!     assert_eq!((h, s, v), (255, 255, 255));
//!
//!     // Single values don't require tuple destructuring:
//!     let color = Color::Grey(128);
//!     let value = color.unwrap_grey();
//!     assert_eq!(value, 128);
//!
//!     // Safely unwrap with a fallback:
//!     let color = Color::RGB(255, 255, 0);
//!     let (r, g, b) = color.unwrap_or_rgb((0, 0, 0));
//!     assert_eq!((r, g, b), (255, 255, 0));
//!     // Since color is of the RGBA variant, the default is not used.
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
//! # Derived function naming
//! 
//! # Derived functions
//! ```
//! ```
//! 
extern crate proc_macro;

#[macro_use]
mod idents;

mod derive;

use derive::derive_variantly_fns;
use proc_macro::TokenStream;

#[proc_macro_derive(Variantly)]
pub fn variantly(input: TokenStream) -> TokenStream {
    derive_variantly_fns(input)
}
