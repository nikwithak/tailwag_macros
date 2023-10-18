use proc_macro2::TokenStream;
use quote::quote;
use syn::{Data, DeriveInput};

/// Logic for deriving Deref - only tested with structs using named fields.
///
/// Derefs to the field tagged with the `#[deref]` attribute  - if not found, it defaults to the first declared field in the struct.
pub fn derive_display(input: &DeriveInput) -> TokenStream {
    let &DeriveInput {
        ident,
        data,
        ..
    } = &input;
    let Data::Enum(data) = data else {
        panic!("this macro currently only supports no-field Enums")
    };

    let variants = data.variants.iter().map(|v| {
        if v.fields.len() > 0 {
            panic!("This macro currently only supports no-field Enums")
        } else {
            &v.ident
        }
    });
    let variants_str = variants.clone().map(|v| v.to_string());

    // If it's a fieldless struct, we simply deref the name of the struct.
    // let (target_ident, target_type) = target.unwrap_or((quote!(#ident_string), quote!(str)));

    // TODO: Time for generics!
    let tokens = quote!(
        impl ::std::fmt::Display for #ident {
            fn fmt(
                &self,
                f: &mut std::fmt::Formatter<'_>,
            ) -> std::fmt::Result {
                match self {
                    #(Self::#variants => f.write_str(#variants_str),)*
                }
            }
        }
    );
    tokens.into()
}
