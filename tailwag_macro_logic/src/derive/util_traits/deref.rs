use proc_macro2::TokenStream;
use quote::quote;
use syn::{Data, DeriveInput};

use crate::util::attribute_parsing::GetAttribute;

/// Logic for deriving Deref - only tested with structs using named fields.
///
/// Derefs to the field tagged with the `#[deref]` attribute  - if not found, it defaults to the first declared field in the struct.
pub fn derive_deref(input: &DeriveInput) -> TokenStream {
    let &DeriveInput {
        ident,
        data,
        ..
    } = &input;
    let Data::Struct(data) = data else {
    panic!("Only Structs are supported")
  };

    let target = data
        .fields
        .iter()
        .find(|f| f.get_attribute("deref").is_some())
        .map_or_else(
            || {
                if data.fields.len() <= 1 {
                    data.fields.iter().next()
                } else {
                    panic!("More than one field found in struct. Use #[deref] to tag the field you want to deref.")
                }
            },
            |f| Some(f),
        )
        .map(|field| {
            let target_name = &field.ident;
            let target_type = &field.ty;
            (quote!(&self.#target_name), quote!(#target_type))
        });

    let ident_string = ident.to_string();
    // If it's a fieldless struct, we simply deref the name of the struct.
    let (target_ident, target_type) = target.unwrap_or((quote!(#ident_string), quote!(str)));

    // TODO: Time for generics!
    let tokens = quote!(
        impl std::ops::Deref for #ident {
            type Target = #target_type;
            fn deref(&self) -> &Self::Target {
                #target_ident
            }
        }
    );
    tokens.into()
}
