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

/// Iterate over a vec of variants and collect the ident of each as-is, categorized by the type of enum variant (named, unnamed or unit).
pub fn collect_idents_by_valueness(variants: &Vec<Variant>) -> (Vec<Ident>, Vec<Ident>, Vec<Ident>) {
    let mut named: Vec<Ident> = vec![];
    let mut unnamed: Vec<Ident> = vec![];
    let mut unit: Vec<Ident> = vec![];
    variants.iter().for_each(|variant| {

        match variant.fields {
            syn::Fields::Unnamed(_) => named.push(variant.ident.clone()),
            syn::Fields::Named(_) => unnamed.push(variant.ident.clone()),
            _ => unit.push(variant.ident.clone())
        }
    });

    (named, unnamed, unit)
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
        .filter_map(|variant| {
            if let syn::Fields::Unnamed(value) = variant.fields.clone() {
                Some(value.unnamed)
            } else {
                None
            }
        })
        .collect()
}

/// Iterate over a vec of variants and clone each element into a vec categorized by variant type (named, unnamed or unit).
pub fn split_variants_by_valueness(variants: &Vec<Variant>) -> (Vec<Variant>, Vec<Variant>, Vec<Variant>) {
    let mut named: Vec<Variant> = vec![];
    let mut unnamed: Vec<Variant> = vec![];
    let mut unit: Vec<Variant> = vec![];
    variants.iter().for_each(|variant| {

        match variant.fields {
            syn::Fields::Unnamed(_) => named.push(variant.clone()),
            syn::Fields::Named(_) => unnamed.push(variant.clone()),
            _ => unit.push(variant.clone())
        }
    });

    (named, unnamed, unit)
}
