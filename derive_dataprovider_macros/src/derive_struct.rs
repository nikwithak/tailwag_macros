use derive_dataprovider_logic::database_definition::table_definition::DatabaseTableDefinition;
use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use syn::{Data, DeriveInput};

use crate::function::{build_create_table_query, build_get_query, build_insert_query};

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
            let table = DatabaseTableDefinition::from(input);

            let query_create = build_create_table_query(&table).to_string();
            let query_list = build_get_query(&table).to_string();
            let query_insert = build_insert_query(&table).to_string();

            // Build the actual implementation
            let parse_args_impl_tokens = quote!(
                const CREATE_TABLE_QUERY: &str = #query_create;
                const LIST_ALL_QUERY: &str = #query_list;
                const INSERT_NEW_QUERY: &str = #query_insert;
                ////////////////////////////////////////
                // The actual output is defined here. //
                ////////////////////////////////////////
                impl #trait_name for #ident {
                    fn build_create_table_query() -> String {
                        // TODO: Store it as a `const` and return &str
                        format!("{}", #query_create)
                    }
                    fn build_list_table_query() -> String {
                        format!("{}", #query_list)
                    }
                    fn build_insert_table_query() -> String {
                        format!("{}", #query_insert)
                    }
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
