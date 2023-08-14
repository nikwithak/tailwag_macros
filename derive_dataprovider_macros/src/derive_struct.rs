use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use syn::{Data, DeriveInput};
use tailwag_orm::database_definition::table_definition::DatabaseTableDefinition;

use crate::database_table_definition;

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

            let trait_name = format_ident!("{}", "PostgresDataProvider");
            let table = database_table_definition::build_table_definition(&input);

            // Build the actual implementation
            let parse_args_impl_tokens = quote!(
                ////////////////////////////////////////
                // The actual output is defined here. //
                ////////////////////////////////////////
                impl #trait_name for #ident {
                    fn make_migrations() -> Result<(), String> {
                        Ok(())
                    }
                }
            );

            parse_args_impl_tokens.into()
        },
        syn::Fields::Unnamed(_) => unimplemented!("Unnamed fields not supported yet"),
        syn::Fields::Unit => unimplemented!("Unit fields not supported yet"),
    }
}
