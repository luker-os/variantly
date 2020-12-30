use crate::error::Result;
use darling::{ast::Fields, FromVariant};
use syn::{Attribute, Ident, ItemEnum, Type, Visibility};

/// Struct for parsing relevant input to each variant of a variantly derived enum.
#[derive(FromVariant, Debug)]
#[darling(attributes(variantly))]
pub struct VariantInput {
    pub ident: Ident,
    #[darling(default)]
    pub rename: Option<Ident>,
    pub fields: Fields<FieldParsed>,
}

/// Struct for parsing relevant information from a variant field
#[derive(FromField, Debug)]
#[darling(forward_attrs)]
pub struct FieldParsed {
    pub ident: Option<Ident>,
    pub ty: Type,
    pub attrs: Vec<Attribute>,
    pub vis: Visibility,
}

/// Parsed input to each variant of a variantly derived enum.
#[derive(Debug)]
pub struct VariantParsed {
    pub ident: Ident,
    pub used_name: Ident,
    pub fields: Fields<FieldParsed>,
}

impl From<VariantInput> for VariantParsed {
    fn from(variant: VariantInput) -> Self {
        let ident = &variant.ident;
        VariantParsed {
            used_name: variant.rename.unwrap_or_else(|| ident.clone()),
            ident: variant.ident,
            fields: variant.fields,
        }
    }
}

/// Attempt to parse an ItemEnum into a vec of parsed variants.
pub fn try_parse_variants(item_enum: &ItemEnum) -> Result<Vec<VariantParsed>> {
    item_enum
        .variants
        .iter()
        .map(|variant| {
            VariantInput::from_variant(variant)
                .map(VariantInput::into)
                .map_err(darling::Error::into)
        })
        .collect()
}

/// Helper function for validation that requires comparing each variant with each other variant.
/// Visits each pair only once and early returns on the first failure.
pub fn validate_compare<F: Fn(&VariantParsed, &VariantParsed) -> Result<()>>(
    variants: &Vec<VariantParsed>,
    validations: Vec<F>,
) -> Result<()> {
    // Enumerate over the entire set.
    variants
        .as_slice()
        .iter()
        .enumerate()
        .try_for_each(|(index, variant_a)| -> Result<()> {
            // Iterate over variants not visited already by the primary iterator.
            variants[(index + 1)..variants.len()]
                .iter()
                .try_for_each(|variant_b| {
                    // Run the current pair against all validation fns
                    validations
                        .iter()
                        .try_for_each(|validation| validation(variant_a, variant_b))
                })
        })
}

/// Validate that the used names for each variant will not cause naming conflicts.
pub fn compare_used_names(a: &VariantParsed, b: &VariantParsed) -> Result<()> {
    if a.used_name == b.used_name {
        let message = format!("`{}` cannot be coerced into a unique & idiomatic snake_case function name as it would collide with the `{}` variant of the same Enum. \
            use the following attribute on this or the conflicting variant to resolve: `#[variantly(rename = \"some_other_name\")]`",
            &a.ident, &b.ident);
        Err(syn::Error::new(a.ident.span(), message).into())
    } else {
        Ok(())
    }
}
