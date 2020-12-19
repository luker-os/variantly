use inflector::cases::snakecase::to_snake_case;
use quote::format_ident;
use syn::{punctuated::Punctuated, token::Comma, Field, Ident, ItemEnum, Variant};


/// Iterate over a vec of variants and map each to an ident, prefixed by `prefix`
pub fn collect_parsed_idents(variants: &Vec<Variant>, prefix: &str) -> Vec<Ident> {
    variants
        .iter()
        .map(|variant| format_ident!("{}_{}", prefix, to_snake_case(&variant.ident.to_string())))
        .collect()
}

/// Iterate over a vec of variants and collect the ident of each as-is.
pub fn collect_idents(variants: &Vec<Variant>) -> Vec<Ident> {
    variants
        .iter()
        .map(|variant| variant.ident.clone())
        .collect()
}

/// Iterate over and collect the variants contained by an ItemEnum
pub fn collect_variants(r#enum: &ItemEnum) -> Vec<Variant> {
    r#enum
        .variants
        .iter()
        .map(|variant| variant.clone())
        .collect()
}

/// Iterate over a vec of variants and map each to the associated type contained by the given variant
pub fn collect_variant_types(variants: &Vec<Variant>) -> Vec<Punctuated<Field, Comma>> {
    variants
        .iter()
        .map(|variant| {
            if let syn::Fields::Unnamed(value) = variant.fields.clone() {
                value.unnamed
            } else {
                panic!(); // TODO: Deal with variants that do not have an inner type.
            }
        })
        .collect()
}
