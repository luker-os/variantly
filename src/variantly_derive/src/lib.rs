extern crate proc_macro;

use inflector::cases::snakecase::to_snake_case;
use proc_macro::TokenStream;
use quote::{format_ident, quote};
use std::iter::repeat;
use syn::{
    parse_macro_input, punctuated::Punctuated, token::Comma, Field, Ident, ItemEnum, Variant,
};

#[proc_macro_derive(Variantly)]
pub fn variantly(input: TokenStream) -> TokenStream {
    let enm = parse_macro_input!(input as ItemEnum);
    let enm_name = format_ident!("{}", enm.ident.to_string());
    let enm_name = repeat(enm_name);

    let variants: Vec<Variant> = enm.variants.iter().map(|variant| variant.clone()).collect();

    let name: Vec<Ident> = variants
        .iter()
        .map(|variant| variant.ident.clone())
        .collect();

    let types: Vec<Punctuated<Field, Comma>> = variants
        .iter()
        .map(|variant| {
            if let syn::Fields::Unnamed(value) = variant.fields.clone() {
                value.unnamed
            } else {
                panic!();
            }
        })
        .collect();

    /// Helper for declaring Vec<Ident> based on snake_cased enum variant names and a given suffix.
    macro_rules! declare_idents {
        ($($id:ident),*) => {
            $(
                let name = stringify!($id);
                let $id = variants
                    .iter()
                    .map(|variant| {
                        format_ident!("{}_{}", name, to_snake_case(&variant.ident.to_string()))
                    })
                    .collect::<Vec<Ident>>();
            )*
        };
    }

    declare_idents! {is, ok, ok_or, ok_or_else, expect, contains, unwrap, unwrap_or, unwrap_or_else, is_not, or, or_else, replace, and, and_then};

    let result = quote! {
        #(
            impl #enm_name {

                fn #or(self, other: #enm_name) -> #enm_name {
                    variantly::or!(self, other, #enm_name::#name)
                }

                fn #and(self, other: #enm_name) -> #enm_name {
                    variantly::and!(self, other, #enm_name::#name)
                }

                fn #and_then<F: FnOnce(#types) -> #types>(self, and_then: F) -> #enm_name {
                    variantly::and_then!(self, and_then, #enm_name::#name)
                }

                fn #or_else<F: FnOnce() -> #types>(self, or_else: F) -> #enm_name {
                    variantly::or_else!(self, or_else, #enm_name::#name)
                }

                fn #contains(&self, x: &#types) -> bool {
                    variantly::contains!(self, #enm_name::#name, x)
                }

                fn #expect(self, msg: &str) -> #types {
                    variantly::expect!(self, #enm_name::#name, msg)
                }

                fn #is(&self) -> bool {
                    variantly::is!(self, #enm_name::#name)
                }

                fn #is_not(&self) -> bool {
                    !self.#is()
                }

                fn #ok(self) -> Option<#types> {
                    variantly::ok!(self, #enm_name::#name)
                }

                fn #replace(&mut self, value: #types) -> #enm_name {
                    variantly::replace!(self, #enm_name::#name, value)
                }

                fn #ok_or<E>(self, or: E) -> Result<#types, E> {
                    variantly::ok_or!(self, #enm_name::#name, or    )
                }

                fn #ok_or_else<E, F: FnOnce() -> E>(self, or_else: F) -> Result<#types, E> {
                    variantly::ok_or_else!(self, #enm_name::#name, (or_else))
                }

                fn #unwrap(self) -> #types {
                    variantly::unwrap!(self, #enm_name::#name)
                }

                fn #unwrap_or(self, or: #types) -> #types {
                    variantly::unwrap_or!(self, #enm_name::#name, or)
                }

                fn #unwrap_or_else(self, or_else: fn() -> #types) -> #types { // TODO should this be FnOnce
                    variantly::unwrap_or_else!(self, #enm_name::#name, (or_else))
                }
            }
        )*
    }
    .into();
    result
}
