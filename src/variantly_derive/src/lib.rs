//! This crate holds the Variantly derive macro. The use of this derive is dependent on the variantly crate.
//! As such, this crate should not directly be used, but instead be accessed via the re-export at the root of the variantly crate.
//!
//! Please see the variently crate for more detailed documentation about using this derive macro.

extern crate proc_macro;

mod parse_input;

use inflector::cases::snakecase::to_snake_case;
use parse_input::generate_idents;
use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::{format_ident, quote};
use syn::{parse_macro_input, Fields, FieldsUnnamed, Ident, ItemEnum};

macro_rules! identify {
    ($funcs: expr, $ident:expr, ($($operation:ident$(,)*)*), $($tt:tt)*) => {
        $(
            let $operation = format_ident!(
                "{}_{}",
                stringify!($operation),
                to_snake_case(&$ident.to_string())
            );
        )*
        $funcs.push(quote! { $($tt)* });
    };
}

fn handle_unnamed(
    fields: &FieldsUnnamed,
    ident: &Ident,
    funcs: &mut Vec<TokenStream2>,
    enum_name: &Ident,
) {
    let r#type = &fields.unnamed;
    let vars = generate_idents(r#type.len());
    let vars = quote! { (#( #vars ),*)};
    let r#type = quote! { #r#type };

    identify!(funcs, ident, (and_then, and, expect, ok_or_else, ok_or, ok, or_else, or, unwrap_or_else, unwrap_or, unwrap),
        fn #and(self, and: #enum_name) -> #enum_name {
            variantly::and!(#enum_name::#ident, self, and, #vars)
        }

        fn #and_then<F: FnOnce((#r#type)) -> (#r#type)>(self, and_then: F) -> #enum_name {
            variantly::and_then!(#enum_name::#ident, self, and_then, #vars)
        }

        fn #expect(self, msg: &str) -> (#r#type) {
            variantly::expect!(#enum_name::#ident, self, msg, #vars)
        }

        fn #ok(self) -> Option<(#r#type)> {
            variantly::ok!(#enum_name::#ident, self, #vars)
        }

        fn #ok_or<E>(self, or: E) -> Result<(#r#type), E> {
            variantly::ok_or!(#enum_name::#ident, self, or, #vars)
        }

        fn #ok_or_else<E, F: FnOnce() -> E>(self, or_else: F) -> Result<(#r#type), E> {
            variantly::ok_or_else!(#enum_name::#ident, self, (or_else), #vars)
        }

        fn #or(self, or: #enum_name) -> #enum_name {
            variantly::or!(#enum_name::#ident, self, or, #vars)
        }

        fn #or_else<F: FnOnce() -> (#r#type)>(self, or_else: F) -> #enum_name {
            variantly::or_else!(#enum_name::#ident, self, or_else, #vars)
        }

        fn #unwrap(self) -> (#r#type) {
            variantly::unwrap!(#enum_name::#ident, self, #vars)
        }

        fn #unwrap_or(self, or: (#r#type)) -> (#r#type) {
            variantly::unwrap_or!(#enum_name::#ident, self, or, #vars)
        }

        fn #unwrap_or_else<F: FnOnce() -> (#r#type)>(self, or_else: F) -> (#r#type) {
            variantly::unwrap_or_else!(#enum_name::#ident, self, (or_else), #vars)
        }
    );
}

#[proc_macro_derive(Variantly)]
pub fn variantly(input: TokenStream) -> TokenStream {
    // parse necessary information from input
    let r#enum = parse_macro_input!(input as ItemEnum);
    let enum_name = format_ident!("{}", r#enum.ident.to_string());

    let mut funcs = vec![];

    r#enum.variants.iter().for_each(|variant| {
        let ident = &variant.ident;
        let ignore;
        match &variant.fields {
            Fields::Unnamed(fields) => {
                handle_unnamed(fields, &ident, &mut funcs, &enum_name);
                ignore = quote!((..));
            }
            Fields::Named(_) => ignore = quote!({ .. }),
            Fields::Unit => ignore = quote!(),
        }

        identify!(funcs, ident, (is, is_not),
            fn #is(&self) -> bool {
                variantly::is!(#enum_name::#ident#ignore, self)
            }

            fn #is_not(&self) -> bool {
                !self.#is()
            }
        );
    });
    let output: TokenStream = quote! {
        impl #enum_name {
            #(#funcs)*
        }
    }
    .into();

    // println!("outputs: {}", output.to_string());

    output
}
