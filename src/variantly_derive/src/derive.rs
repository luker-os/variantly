use proc_macro::TokenStream;

use crate::idents::generate_idents;
use proc_macro2::TokenStream as TokenStream2;
use quote::{format_ident, quote};
use syn::{parse_macro_input, Fields, FieldsUnnamed, Ident, ItemEnum};

pub fn derive_variantly_fns(input: TokenStream) -> TokenStream {
    // parse necessary information from input
    let r#enum = parse_macro_input!(input as ItemEnum);
    let enum_name = format_ident!("{}", r#enum.ident.to_string());

    // For collecting impl functions
    let mut functions = vec![];

    r#enum.variants.iter().for_each(|variant| {
        let ident = &variant.ident;
        // This will be initialized with a tokenstream representing how to match & ignore any variables held by a variant.
        let ignore;

        // variant-type specific logic
        match &variant.fields {
            Fields::Unnamed(fields) => {
                handle_unnamed(fields, &ident, &mut functions, &enum_name);
                ignore = quote!((..));
            }
            Fields::Named(_) => ignore = quote!({ .. }),
            Fields::Unit => ignore = quote!(),
        }

        // include any impl functions that are common to all variant types.
        identify!(ident, [is, is_not]);
        functions.push(quote! {
            fn #is(&self) -> bool {
                variantly::is!(#enum_name::#ident#ignore, self)
            }

            fn #is_not(&self) -> bool {
                !self.#is()
            }
        });
    });

    // Declare the actual impl block & iterate over all include fns.
    let output: TokenStream = quote! {
        impl #enum_name {
            #(#functions)*
        }
    }
    .into();

    // println!("outputs: {}", output.to_string());

    output
}

/// Construct all impl functions related to variants with unnamed internal variables and add them to the functions vec.
fn handle_unnamed(
    fields: &FieldsUnnamed,
    ident: &Ident,
    functions: &mut Vec<TokenStream2>,
    enum_name: &Ident,
) {
    // parse necessary information from fields.
    let r#type = &fields.unnamed;
    let vars = generate_idents(r#type.len());
    let vars = quote! { (#( #vars ),*)};
    let r#type = quote! { #r#type };

    // declare ident variables with helper macro.
    identify!(
        ident,
        [
            and_then,
            and,
            expect,
            ok_or_else,
            ok_or,
            ok,
            or_else,
            or,
            unwrap_or_else,
            unwrap_or,
            unwrap
        ]
    );

    // Create and push actual impl functions
    functions.push(quote! {
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
    });
}
