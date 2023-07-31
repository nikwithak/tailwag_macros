use crate::{database_definition::table_definition::DatabaseTableDefinition, AsSql};

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct CreateTable {
    table_definition: DatabaseTableDefinition,
}

impl CreateTable {
    pub fn new(table: DatabaseTableDefinition) -> Self {
        Self {
            table_definition: table,
        }
    }

    pub fn table_definition(&self) -> &DatabaseTableDefinition {
        &self.table_definition
    }
}

impl AsSql for CreateTable {
    fn as_sql(&self) -> String {
        let mut sql = format!(
            "CREATE TABLE IF NOT EXISTS {} {{\n",
            self.table_definition.table_name.as_str()
        );
        let columns_sql = self
            .table_definition
            .columns
            .iter()
            .map(|col| col.as_sql())
            .collect::<Vec<String>>()
            .join(",\n");

        sql.push_str(&columns_sql);
        sql.push_str("\n};");

        sql
    }
}

#[cfg(test)]
mod test {
    use create_table::CreateTable;

    use crate::{
        database_definition::{
            migration::create_table,
            table_definition::{
                DatabaseColumnType, DatabaseTableDefinition, Identifier, TableColumn,
            },
        },
        AsSql,
    };

    #[test]
    fn as_sql_works() {
        let table_definition = DatabaseTableDefinition {
            table_name: Identifier::new("new_table".to_string()).unwrap(),
            columns: vec![
                TableColumn {
                    column_name: Identifier::new("uuid_pk_nonnull".to_string()).unwrap(),
                    column_type: DatabaseColumnType::Uuid,
                    is_primary_key: true,
                    is_nullable: false,
                },
                TableColumn {
                    column_name: Identifier::new("string".to_string()).unwrap(),
                    column_type: DatabaseColumnType::String,
                    is_primary_key: false,
                    is_nullable: true,
                },
                TableColumn {
                    column_name: Identifier::new("bool_nonnull".to_string()).unwrap(),
                    column_type: DatabaseColumnType::Boolean,
                    is_primary_key: false,
                    is_nullable: false,
                },
                TableColumn {
                    column_name: Identifier::new("float_nonnull".to_string()).unwrap(),
                    column_type: DatabaseColumnType::Float,
                    is_primary_key: false,
                    is_nullable: false,
                },
                TableColumn {
                    column_name: Identifier::new("int".to_string()).unwrap(),
                    column_type: DatabaseColumnType::Int,
                    is_primary_key: false,
                    is_nullable: true,
                },
                TableColumn {
                    column_name: Identifier::new("create_timestamp".to_string()).unwrap(),
                    column_type: DatabaseColumnType::Timestamp,
                    is_primary_key: false,
                    is_nullable: true,
                },
            ],
        };

        let create_table = CreateTable {
            table_definition,
        };

        // Act
        let queries = create_table.as_sql();
        // let mut queries = result_sql.split("\n").collect::<Vec<&str>>();

        #[rustfmt::skip]
        let expected_query = vec![
            "CREATE TABLE IF NOT EXISTS new_table {",
                " uuid_pk_nonnull UUID PRIMARY KEY NOT NULL ,",
                " string VARCHAR   ,",
                " bool_nonnull BOOL  NOT NULL ,",
                " float_nonnull FLOAT  NOT NULL ,",
                " int INT   ,",
                " create_timestamp TIMESTAMP   ",
            "};"
        ].join("\n");

        assert_eq!(queries, expected_query);
    }
}
