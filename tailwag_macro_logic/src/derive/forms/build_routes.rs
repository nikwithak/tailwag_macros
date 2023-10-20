use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use syn::{Data, DeriveInput, Field, FieldsNamed, Ident};

use crate::util::{self, attribute_parsing::GetAttribute};

pub fn derive_struct(input: &DeriveInput) -> TokenStream {
    let &DeriveInput {
        ident,
        data,
        ..
    } = &input;
    // Panic with error message if we get a non-struct
    let Data::Struct(data) = data else { panic!("Only Structs are supported") };

    match &data.fields {
        syn::Fields::Named(fields) => {
            let fields_filtered: Vec<&Field> = fields
                .named
                .iter()
                .filter(|field| {
                    // TODO: Abstract this for easier filtering
                    field.get_attribute("request_ignore").is_some() // TODO: `request(ignore)`
                        || field.ident.as_ref().expect("Unexpected unnamed field found").to_string()
                            != "id" // Always include "id"
                })
                .collect();
            let field_names_filtered: Vec<&Ident> = fields_filtered
                .iter()
                .map(|f| f.ident.as_ref().expect("Found missing ident for field: {}"))
                .collect();

            let parse_args_impl_tokens = quote!(
                #[axum::async_trait]
                impl tailwag::web::traits::rest_api::BuildRoutes<#ident> for #ident
                {

                }
            );

            parse_args_impl_tokens.into()
        },
        syn::Fields::Unnamed(_) => unimplemented!("Unnamed fields not supported yet"),
        syn::Fields::Unit => unimplemented!("Unit fields not supported yet"),
    }
}
