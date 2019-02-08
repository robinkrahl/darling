use proc_macro2::TokenStream;

use ast::Fields;
use ast::Style;
use codegen::Field;

pub struct FieldsGen<'a> {
    fields: &'a Fields<Field<'a>>,
    allow_unknown_fields: bool,
}

impl<'a> FieldsGen<'a> {
    pub fn new(fields: &'a Fields<Field<'a>>, allow_unknown_fields: bool) -> Self {
        Self {
            fields,
            allow_unknown_fields,
        }
    }

    /// Create declarations for all the fields in the struct.
    pub(in codegen) fn declarations(&self) -> TokenStream {
        match *self.fields {
            Fields {
                style: Style::Struct,
                ref fields,
            } => {
                let vdr = fields.iter().map(Field::as_declaration);
                quote!(#(#vdr)*)
            }
            _ => panic!("FieldsGen doesn't support tuples yet"),
        }
    }

    /// Generate the loop which walks meta items looking for property matches.
    pub(in codegen) fn core_loop(&self) -> TokenStream {
        let arms = self.fields.as_ref().map(Field::as_match);

        // If we're allowing unknown fields, then handling one is a no-op.
        // Otherwise, we're going to push a new spanned error pointing at the field.
        let handle_unknown = if self.allow_unknown_fields {
            quote!()
        } else {
            let names = self.fields.as_ref().map(Field::as_name);
            quote! {
                __errors.push(::darling::Error::unknown_field_with_alts(__other, &[#(#names),*]).with_span(__inner));
            }
        };

        quote!(
            for __item in __items {
                if let ::syn::NestedMeta::Meta(ref __inner) = *__item {
                    let __name = __inner.name().to_string();
                    match __name.as_str() {
                        #(#arms)*
                        __other => { #handle_unknown }
                    }
                }
            }
        )
    }

    pub fn require_fields(&self) -> TokenStream {
        match *self.fields {
            Fields {
                style: Style::Struct,
                ref fields,
            } => {
                let checks = fields.iter().map(Field::as_presence_check);
                quote!(#(#checks)*)
            }
            _ => panic!("FieldsGen doesn't support tuples for requirement checks"),
        }
    }

    pub(in codegen) fn initializers(&self) -> TokenStream {
        let inits = self.fields.as_ref().map(Field::as_initializer);

        quote!(#(#inits),*)
    }
}
