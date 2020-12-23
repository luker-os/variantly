use quote::format_ident;
use syn::Ident;
use uuid::Uuid;

pub fn generate_idents(count: usize) -> Vec<Ident> {
    let mut idents: Vec<Ident> = vec![];
    for _ in 0..count {
        idents.push(unique_ident())
    }
    idents
}

pub fn unique_ident() -> Ident {
    format_ident!("ident_{}", Uuid::new_v4().to_simple().to_string())
}
