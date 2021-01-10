use crate::{
    error::Result,
    idents::generate_idents,
    input::{compare_used_names, try_parse_variants, validate_compare, VariantParsed},
};

use darling::ast::Style::{Struct, Tuple, Unit};
use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::{format_ident, quote};
use syn::{Ident, ItemEnum, Type};

pub fn derive_variantly_fns(item_enum: ItemEnum) -> Result<TokenStream> {
    let enum_name = &item_enum.ident;

    // For collecting impl functions
    let mut functions = vec![];

    let variants = try_parse_variants(&item_enum)?;

    validate_compare(&variants, vec![compare_used_names])?;

    variants.iter().for_each(|variant| {
        // This will be initialized with a tokenstream representing how to match & ignore any variables held by a variant.
        let ignore;
        let ident = &variant.ident;
        match &variant.fields.style {
            Tuple => {
                handle_tuple(&variant, &mut functions, &enum_name);
                ignore = quote!((..));
            }
            Struct => ignore = quote!({ .. }),
            Unit => ignore = quote!(),
        }

        // include any impl functions that are common to all variant types.
        identify!(variant.used_name, [is, is_not]);
        functions.push(quote! {
            pub fn #is(&self) -> bool {
                match self {
                    #enum_name::#ident#ignore => true,
                    _ => false
                }
            }

            pub fn #is_not(&self) -> bool {
                !self.#is()
            }
        });
    });

    let generics = &item_enum.generics;
    let where_clause = &generics.where_clause;

    // Declare the actual impl block & iterate over all fns.
    let output: TokenStream = quote! {
        impl#generics #enum_name#generics #where_clause {
            #(#functions)*
        }
    }
    .into();

    Ok(output)
}

/// Construct all impl functions related to variants with tuple style internal variables and add them to the functions vec.
fn handle_tuple(variant: &VariantParsed, functions: &mut Vec<TokenStream2>, enum_name: &Ident) {
    // parse necessary information from variant & fields.
    let ident = &variant.ident;
    let formatted_variant = &variant.used_name;
    let types: Vec<&Type> = variant
        .fields
        .fields
        .iter()
        .map(|field| &field.ty)
        .collect();

    // Generate a unique ident per type used in the variant
    let vars = generate_idents(types.len());
    let vars = quote! { (#( #vars ),*)};
    let types = quote! { (#( #types ),*)};

    // declare ident variables with helper macro.
    identify!(
        variant.used_name,
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
        pub fn #formatted_variant(self) -> Option<(#types)> {
            self.#ok()
        }

        pub fn #and(self, and: Self) -> Self {
            match (&self, and) {
                (&#variant(..), #var_pattern) => #var_pattern,
                _ => self
            }
        }

        pub fn #and_then<F: FnOnce((#types)) -> (#types)>(self, and_then: F) -> Self {
            match self {
                #var_pattern => {
                    let #vars = and_then(#vars);
                    #var_pattern
                },
                _ => self
            }
        }

        pub fn #expect(self, msg: &str) -> (#types) {
            self.#unwrap_or_else(|| panic!("{}", msg))
        }

        pub fn #ok(self) -> Option<(#types)> {
            match self {
                #var_pattern => Some((#vars)),
                _ => None,
            }
        }

        pub fn #ok_or<E>(self, or: E) -> Result<(#types), E> {
            self.#ok_or_else(|| or)
        }

        pub fn #ok_or_else<E, F: FnOnce() -> E>(self, or_else: F) -> Result<(#types), E> {
            match self {
                #var_pattern => Ok((#vars)),
                _ => Err(or_else())
            }
        }

        pub fn #or(self, or: Self) -> Self {
            match self {
                #var_pattern => #var_pattern,
                _ => or,
            }
        }

        pub fn #or_else<F: FnOnce() -> (#types)>(self, or_else: F) -> Self {
            match self {
                #var_pattern => #var_pattern,
                _ => {
                    let #vars = or_else();
                    #var_pattern
                }
            }
        }

        pub fn #unwrap(self) -> (#types) {
            self.#unwrap_or_else(|| panic!())
        }

        pub fn #unwrap_or(self, or: (#types)) -> (#types) {
            self.#unwrap_or_else(|| or)
        }

        pub fn #unwrap_or_else<F: FnOnce() -> (#types)>(self, or_else: F) -> (#types) {
            match self {
                #var_pattern => (#vars),
                _ => or_else()
            }
        }
    });
}
