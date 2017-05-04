#![recursion_limit = "256"]

#[macro_use]
extern crate lazy_static;

#[macro_use]
extern crate quote;

extern crate syn;

extern crate serde_case;

pub mod codegen;
mod errors;
mod from_derive_input;
mod from_meta_item;
mod ident_list;
pub mod options;

pub use errors::{Result, Error};
pub use from_derive_input::FromDeriveInput;
pub use from_meta_item::{ApplyMetaItem, FromMetaItem};
pub use ident_list::IdentList;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn do_things() {
        let foo = options::Field {
            target_name: syn::parse_ident("lorem").unwrap(),
            attr_name: Some("ipsum".to_string()),
            ty: syn::parse_type("bool").unwrap(),
            default: None,
            with: None,
        };

        let bar = options::Field {
            target_name: syn::parse_ident("dolor").unwrap(),
            attr_name: None,
            ty: syn::parse_type("String").unwrap(),
            default: None,
            with: None,
        };

        let container = options::Container {
            ident: syn::parse_ident("Foo").unwrap(),
            generics: Default::default(),
            default: None,
            rename_rule: serde_case::RenameRule::None,
        };

        let derive_tgt = codegen::TraitImpl {
            fields: vec![foo.as_codegen_field(), bar.as_codegen_field()],
            ..(&container).into()
        };

        println!("{}", quote!(#derive_tgt));
    }
}