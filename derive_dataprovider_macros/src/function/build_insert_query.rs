use quote::{format_ident, quote};

use derive_dataprovider_logic::database_definition::{
    self, table_definition::DatabaseTableDefinition,
};

pub fn build_insert_query(
    DatabaseTableDefinition {
        table_name,
        columns,
        ..
    }: &DatabaseTableDefinition
) -> String {
    let qualified_column_names = columns
        .iter()
        .map(|c| &c.column_name)
        .map(|column_name| quote!(#table_name.#column_name));

    let column_lookups = columns.iter().map(|c| &c.column_name).map(|col| quote!(self.#col));
    let query = quote!(INSERT INTO #table_name (#(#qualified_column_names),*) VALUES (#(#column_lookups),*) #table_name);
    query.to_string().replace(", ", ",\n")
}
