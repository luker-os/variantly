//! This crate holds the Variantly derive macro. The use of this derive is dependent on the variantly crate.
//! As such, this crate should not directly be used, but instead be accessed via the re-export at the root of the variantly crate.
//!
//! Please see the variently crate for more detailed documentation about using this derive macro.

extern crate proc_macro;

#[macro_use]
mod idents;

mod derive;

use derive::derive_variantly_fns;
use proc_macro::TokenStream;

/// This export should not be used directly and should instead be imported from the variantly crate.
/// Derive convenience functions that are familiar from Result and Option for other enums.
#[proc_macro_derive(Variantly)]
pub fn variantly(input: TokenStream) -> TokenStream {
    derive_variantly_fns(input)
}
