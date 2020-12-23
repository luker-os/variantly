use proc_macro::TokenStream;

use crate::idents::generate_idents;
use proc_macro2::TokenStream as TokenStream2;
use quote::{format_ident, quote};
use syn::{parse_macro_input, Fields, FieldsUnnamed, Ident, ItemEnum};

pub fn derive_variantly_fns(input: TokenStream) -> TokenStream {
    // parse necessary information from input
    let item_enum = parse_macro_input!(input as ItemEnum);
    let enum_name = format_ident!("{}", item_enum.ident.to_string());

    // For collecting impl functions
    let mut functions = vec![];

    item_enum.variants.iter().for_each(|variant| {
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
                match self {
                    #enum_name::#ident#ignore => true,
                    _ => false
                }
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

    println!("outputs:\n{}", output.to_string());

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
    let types = &fields.unnamed;
    let vars = generate_idents(types.len());
    let vars = quote! { (#( #vars ),*)};
    let types = quote! { #types };

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

    let variant = quote! { #enum_name::#ident };

    // used for both pattern matching and constructing variants:
    // var_pattern = SomeEnum::SomeVariant(some_variable_1, some_variable_2)
    let var_pattern = quote! { #enum_name::#ident#vars };

    // Create and push actual impl functions
    functions.push(quote! {
        fn #and(self, and: #enum_name) -> #enum_name {
            match (&self, and) {
                (&#variant(..), #var_pattern) => #var_pattern,
                _ => self
            }
        }

        fn #and_then<F: FnOnce((#types)) -> (#types)>(self, and_then: F) -> #enum_name {
            match self {
                #var_pattern => {
                    let #vars = and_then(#vars);
                    #var_pattern
                },
                _ => self
            }
        }

        fn #expect(self, msg: &str) -> (#types) {
            self.#unwrap_or_else(|| panic!("{}", msg))
        }

        fn #ok(self) -> Option<(#types)> { // TODO: Verify this is ok for single var enums
            match self {
                #var_pattern => Some((#vars)),
                _ => None,
            }
        }

        fn #ok_or<E>(self, or: E) -> Result<(#types), E> {
            self.#ok_or_else(|| or)
        }

        fn #ok_or_else<E, F: FnOnce() -> E>(self, or_else: F) -> Result<(#types), E> {
            match self {
                #var_pattern => Ok((#vars)),
                _ => Err(or_else())
            }
        }

        fn #or(self, or: #enum_name) -> #enum_name {
            match self {
                #var_pattern => #var_pattern,
                _ => or,
            }
        }

        fn #or_else<F: FnOnce() -> (#types)>(self, or_else: F) -> #enum_name {
            match self {
                #var_pattern => #var_pattern,
                _ => {
                    let #vars = or_else();
                    #var_pattern
                }
            }
        }

        fn #unwrap(self) -> (#types) {
            self.#unwrap_or_else(|| panic!())
        }

        fn #unwrap_or(self, or: (#types)) -> (#types) {
            self.#unwrap_or_else(|| or)
        }

        fn #unwrap_or_else<F: FnOnce() -> (#types)>(self, or_else: F) -> (#types) {
            match self {
                #var_pattern => (#vars),
                _ => or_else()
            }
        }
    });
}
