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
        ($($id:ident),*,) => {
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

    declare_idents! {
        and_then,
        and,
        contains,
        expect,
        is_not, or,
        is,
        ok_or_else,
        ok_or,
        ok,
        or_else,
        replace,
        unwrap_or_else,
        unwrap_or,
        unwrap,
    };

    let result = quote! {
        #(
            impl #enm_name {

                fn #and(self, and: #enm_name) -> #enm_name {
                    variantly::and!(#enm_name::#name, self, and)
                }

                fn #and_then<F: FnOnce(#types) -> #types>(self, and_then: F) -> #enm_name {
                    variantly::and_then!(#enm_name::#name, self, and_then)
                }

                fn #contains(&self, target: &#types) -> bool {
                    variantly::contains!(#enm_name::#name, self, target)
                }

                fn #expect(self, msg: &str) -> #types {
                    variantly::expect!(#enm_name::#name, self, msg)
                }

                fn #is(&self) -> bool {
                    variantly::is!(#enm_name::#name, self)
                }

                fn #is_not(&self) -> bool {
                    !self.#is()
                }

                fn #ok(self) -> Option<#types> {
                    variantly::ok!(#enm_name::#name, self)
                }


                fn #ok_or<E>(self, or: E) -> Result<#types, E> {
                    variantly::ok_or!(#enm_name::#name, self, or)
                }

                fn #ok_or_else<E, F: FnOnce() -> E>(self, or_else: F) -> Result<#types, E> {
                    variantly::ok_or_else!(#enm_name::#name, self, (or_else))
                }

                fn #or(self, or: #enm_name) -> #enm_name {
                    variantly::or!(#enm_name::#name, self, or)
                }

                fn #or_else<F: FnOnce() -> #types>(self, or_else: F) -> #enm_name {
                    variantly::or_else!(#enm_name::#name, self, or_else)
                }

                fn #replace(&mut self, value: #types) -> #enm_name {
                    variantly::replace!(#enm_name::#name, self, value)
                }

                fn #unwrap(self) -> #types {
                    variantly::unwrap!(#enm_name::#name, self)
                }

                fn #unwrap_or(self, or: #types) -> #types {
                    variantly::unwrap_or!(#enm_name::#name, self, or)
                }

                fn #unwrap_or_else(self, or_else: fn() -> #types) -> #types { // TODO should this be FnOnce
                    variantly::unwrap_or_else!(#enm_name::#name, self, (or_else))
                }
            }
        )*
    }
    .into();
    result
}
