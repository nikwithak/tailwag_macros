use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use syn::{Data, DeriveInput};
use tailwag_orm::database_definition::table_definition::DatabaseTableDefinition;

use crate::database_table_definition::{self, build_table_definition};

const TRAIT_NAME: &'static str = "PostgresDataProvider";

pub(crate) fn derive_struct(input: &DeriveInput) -> TokenStream {
    let &DeriveInput {
        ident,
        data,
        ..
    } = &input;

    // Panic with error message if we get a non-struct
    let Data::Struct(data) = data else { panic!("Only Structs are supported") };

    match &data.fields {
        syn::Fields::Named(fields) => {
            let _field_names = fields.named.iter().map(|f| &f.ident);
            /////////////////////////////////////////
            // GENERIC stuff. Part of the template //
            /////////////////////////////////////////
            let trait_name = format_ident!("{}", TRAIT_NAME);

            //////////////////////////////////////////////////////////////////////////////////////////
            //   SPECIFIC stuff - this is where you derive useful objects for your implementation   //
            //////////////////////////////////////////////////////////////////////////////////////////
            let table = database_table_definition::build_table_definition(&input);

            /////////////////////////////////////////
            //         Functions Exported          //
            /////////////////////////////////////////
            let functions: Vec<TokenStream> = database_table_definition::functions(&input);

            ////////////////////////////////////////
            // The actual output is defined here. //
            ////////////////////////////////////////

            // TODO: Figure out Generics, when they end up being needed.
            let parse_args_impl_tokens = quote!(
                impl #trait_name for #ident {
                    #(#functions)*
                }
            );

            parse_args_impl_tokens.into()
        },
        syn::Fields::Unnamed(_) => unimplemented!("Unnamed fields not supported yet"),
        syn::Fields::Unit => unimplemented!("Unit fields not supported yet"),
    }
}
