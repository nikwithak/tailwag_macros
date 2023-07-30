use quote::{format_ident, quote};

use crate::database_definition::{DatabaseTableDefinition, TableColumn};

pub fn build_get_query(
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

    let query = quote!(SELECT #(#qualified_column_names),* FROM #table_name);

    query.to_string().replace(", ", ",\n")
}
