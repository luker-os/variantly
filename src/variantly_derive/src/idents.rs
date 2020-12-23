///! Parse or generate idents.
use quote::format_ident;
use syn::Ident;
use uuid::Uuid;

/// Declare a series of vars named by `operation` that contain an ident created
/// by concatenating the stringified `operation`, and the passed in `ident`.
macro_rules! identify {
    ($ident:expr, [$($operation:ident$(,)*)*]) => {
        $(
            let $operation = format_ident!(
                "{}_{}",
                stringify!($operation),
                to_snake_case(&$ident.to_string())
            );
        )*
    };
}

/// Generate the given number of unique and random idents and collect them into a vec.
pub fn generate_idents(count: usize) -> Vec<Ident> {
    let mut idents: Vec<Ident> = vec![];
    for _ in 0..count {
        idents.push(unique_ident())
    }
    idents
}

/// Generate a valid, unique and random ident.
pub fn unique_ident() -> Ident {
    format_ident!("ident_{}", Uuid::new_v4().to_simple().to_string())
}
