use derive_dataprovider_logic::database_definition::table_definition::{
    DatabaseTableDefinition, TableColumn,
};
use quote::{format_ident, quote};

pub fn build_create_table_query(
    DatabaseTableDefinition {
        table_name,
        columns,
        ..
    }: &DatabaseTableDefinition
) -> String {
    let column_toks = columns.iter().map(
        |TableColumn {
             column_name,
             column_type,
             is_primary_key,
             is_nullable,
             ..
         }| {
            let name = format_ident!("{}", column_name);
            let column_type = format_ident!("{}", column_type.as_str());
            // From #[column(primary_key, pk, pk=true)]
            let primary_key = match is_primary_key {
                true => quote!(PRIMARY KEY),
                false => quote!(),
            };
            // If Option
            let nullable = match is_nullable {
                true => quote!(),
                false => quote!(NOT NULL),
            };
            // TODO
            // let default = (if trait impls default)

            quote!(#name #column_type #primary_key #nullable)
        },
    );
    let table_name = format_ident!("{}", table_name);

    let create_query = quote!(
        CREATE TABLE IF NOT EXISTS #table_name (
            #(#column_toks),*
        );
    )
    .to_string();
    create_query.replace(", ", ",\n    ")
}
