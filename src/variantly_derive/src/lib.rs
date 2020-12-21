extern crate proc_macro;

mod parse_input;

use parse_input::{
    collect_idents_by_valueness, collect_parsed_idents, collect_variant_types, collect_variants,
    split_variants_by_valueness,
};
use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{parse_macro_input, punctuated::Punctuated, token::Comma, Field, ItemEnum, Variant};

#[proc_macro_derive(Variantly)]
pub fn variantly(input: TokenStream) -> TokenStream {
    // parse necessary information from input
    let r#enum = parse_macro_input!(input as ItemEnum);
    let enum_name = format_ident!("{}", r#enum.ident.to_string());
    let variants: Vec<Variant> = collect_variants(&r#enum);
    let r#type: Vec<Punctuated<Field, Comma>> = collect_variant_types(&variants);

    let (valued, not_valued) = split_variants_by_valueness(&variants);
    let (valued_name, not_valued_name) = collect_idents_by_valueness(&variants);

    /// Helper for declaring Vec<Ident> based on snake_cased enum variant names and a given suffix.
    macro_rules! declare_idents {
        (variants = $variants:expr, [$($id:ident),*,]) => {
            $( let $id = collect_parsed_idents(&$variants, stringify!($id)); )*
        };
    }

    // Declare all necessary Vec<Ident> variables
    declare_idents! {
        variants = not_valued,
        [
            is_not,
            is,
        ]
    };

    let not_valued_impls = quote! {
        // Repeat the following for each enum variant
        #(
            impl #enum_name {

                fn #is(&self) -> bool {
                    variantly::is!(#enum_name::#not_valued_name, self)
                }

                fn #is_not(&self) -> bool {
                    !self.#is()
                }
            }
        )*
    };

    // Declare all necessary Vec<Ident> variables
    declare_idents! {
        variants = valued,
        [
            and_then,
            and,
            expect,
            is_not,
            is,
            ok_or_else,
            ok_or,
            ok,
            or_else,
            or,
            replace,
            unwrap_or_else,
            unwrap_or,
            unwrap,
        ]
    };

    // The following are only relevant if the enum variant contains a value.
    let valued_impls = quote! {
        // Repeat the following for each enum variant
        #(
            impl #enum_name {

                fn #is(&self) -> bool {
                    variantly::is!(#enum_name::#valued_name(_), self)
                }

                fn #is_not(&self) -> bool {
                    !self.#is()
                }

                fn #and(self, and: #enum_name) -> #enum_name {
                    variantly::and!(#enum_name::#valued_name, self, and)
                }

                fn #and_then<F: FnOnce(#r#type) -> #r#type>(self, and_then: F) -> #enum_name {
                    variantly::and_then!(#enum_name::#valued_name, self, and_then)
                }

                fn #expect(self, msg: &str) -> #r#type {
                    variantly::expect!(#enum_name::#valued_name, self, msg)
                }

                fn #ok(self) -> Option<#r#type> {
                    variantly::ok!(#enum_name::#valued_name, self)
                }

                fn #ok_or<E>(self, or: E) -> Result<#r#type, E> {
                    variantly::ok_or!(#enum_name::#valued_name, self, or)
                }

                fn #ok_or_else<E, F: FnOnce() -> E>(self, or_else: F) -> Result<#r#type, E> {
                    variantly::ok_or_else!(#enum_name::#valued_name, self, (or_else))
                }

                fn #or(self, or: #enum_name) -> #enum_name {
                    variantly::or!(#enum_name::#valued_name, self, or)
                }

                fn #or_else<F: FnOnce() -> #r#type>(self, or_else: F) -> #enum_name {
                    variantly::or_else!(#enum_name::#valued_name, self, or_else)
                }

                fn #replace(&mut self, value: #r#type) -> #enum_name {
                    variantly::replace!(#enum_name::#valued_name, self, value)
                }

                fn #unwrap(self) -> #r#type {
                    variantly::unwrap!(#enum_name::#valued_name, self)
                }

                fn #unwrap_or(self, or: #r#type) -> #r#type {
                    variantly::unwrap_or!(#enum_name::#valued_name, self, or)
                }

                fn #unwrap_or_else<F: FnOnce() -> #r#type>(self, or_else: F) -> #r#type {
                    variantly::unwrap_or_else!(#enum_name::#valued_name, self, (or_else))
                }
            }
        )*
    };

    let impls = quote! {
        #not_valued_impls
        #valued_impls
    }
    .into();

    impls
}
