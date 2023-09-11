use proc_macro2::TokenStream;
use quote::quote;
use syn::{AttrStyle, Attribute, Data, DeriveInput, Field};

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
    fn get_attribute_named<'a>(
        field: &'a Field,
        attr_name: &str,
    ) -> Option<&'a Attribute> {
        field
            .attrs
            .iter()
            .filter(|a| a.style == AttrStyle::Outer)
            .find(|a| a.path().is_ident(attr_name))
    }

    let target = data
        .fields
        .iter()
        .find(|f| get_attribute_named(&f, "deref").is_some())
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
