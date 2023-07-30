mod migration;
pub use migration::*;

#[cfg(test)]
mod tests {
    use migration::{AlterColumn, AlterColumnAction, AlterTable, AlterTableAction};

    use crate::{
        database_definition::table_definition::{
            DatabaseColumnType, DatabaseTableDefinition, Identifier, TableColumn,
        },
        AsSql,
    };

    use super::{migration, Migration};

    fn get_before() -> DatabaseTableDefinition {
        DatabaseTableDefinition {
            table_name: Identifier::new("my_table".to_string()).unwrap(),
            columns: vec![
                TableColumn {
                    column_name: Identifier::new("string_nullable".to_string()).unwrap(),
                    column_type: DatabaseColumnType::String,
                    is_primary_key: false,
                    is_nullable: true,
                },
                TableColumn {
                    column_name: Identifier::new("bool".to_string()).unwrap(),
                    column_type: DatabaseColumnType::Boolean,
                    is_primary_key: false,
                    is_nullable: false,
                },
                TableColumn {
                    column_name: Identifier::new("int".to_string()).unwrap(),
                    column_type: DatabaseColumnType::Int,
                    is_primary_key: false,
                    is_nullable: false,
                },
                TableColumn {
                    column_name: Identifier::new("float".to_string()).unwrap(),
                    column_type: DatabaseColumnType::Float,
                    is_primary_key: false,
                    is_nullable: false,
                },
                TableColumn {
                    column_name: Identifier::new("timestamp".to_string()).unwrap(),
                    column_type: DatabaseColumnType::Timestamp,
                    is_primary_key: false,
                    is_nullable: false,
                },
                TableColumn {
                    column_name: Identifier::new("uuid".to_string()).unwrap(),
                    column_type: DatabaseColumnType::Uuid,
                    is_primary_key: false,
                    is_nullable: false,
                },
            ],
        }
    }

    #[test]
    fn as_sql_generates_sql_script() {
        let migration = Migration {
            table_actions: vec![AlterTable {
                table_name: Identifier::new("my_table".to_string()).unwrap(),
                actions: vec![
                    AlterTableAction::AlterColumn(AlterColumn {
                        column_name: Identifier::new("bool".to_string()).unwrap(),
                        actions: vec![
                            AlterColumnAction::SetType(DatabaseColumnType::String),
                            AlterColumnAction::SetNullability(true),
                        ],
                    }),
                    AlterTableAction::AlterColumn(AlterColumn {
                        column_name: Identifier::new("int".to_string()).unwrap(),
                        actions: vec![AlterColumnAction::SetType(DatabaseColumnType::Float)],
                    }),
                    AlterTableAction::AddColumn(TableColumn {
                        column_name: Identifier::new("new_column".to_string()).unwrap(),
                        column_type: DatabaseColumnType::String,
                        is_primary_key: false,
                        is_nullable: false,
                    }),
                    AlterTableAction::AlterColumn(AlterColumn {
                        column_name: Identifier::new("string_nullable".to_string()).unwrap(),
                        actions: vec![AlterColumnAction::SetNullability(false)],
                    }),
                    AlterTableAction::DropColumn(Identifier::new("timestamp".to_string()).unwrap()),
                ],
            }],
        };

        const EXPECTED_SQL: &str =
            "ALTER TABLE IF EXISTS my_table ALTER COLUMN bool TYPE STRING NONNULL";
    }

    #[test]
    fn new_from_table_definitions_modifies() {
        // Arrange
        let before = get_before();
        let mut after = before.clone();
        after
            .columns
            .iter_mut()
            .find(|c| c.column_name.value().eq("int"))
            // Tests Type changes
            .map(|c| c.column_type = DatabaseColumnType::Float);
        after
            .columns
            .iter_mut()
            .find(|c| c.column_name.value().eq("string_nullable"))
            .map(|c| {
                // Tests Nullability changes
                c.is_nullable = false;
            });
        after.columns.iter_mut().find(|c| c.column_name.value().eq("bool")).map(|c| {
            // Tests a mix of the two changes
            c.column_type = DatabaseColumnType::String;
            c.is_nullable = true;
        });
        after.columns.push(TableColumn {
            column_name: Identifier::new("new_column".to_string()).unwrap(),
            column_type: DatabaseColumnType::String,
            is_primary_key: false,
            is_nullable: false,
        });
        // Delete "timestamp" column
        after.columns = after
            .columns
            .into_iter()
            .filter(|c| !c.column_name.value().eq("timestamp"))
            .collect();

        // Act
        let migration = Migration::new_from_table_definitions(&before, &after).unwrap().unwrap();

        // Assert
        assert_eq!(
            migration,
            Migration {
                table_actions: vec![AlterTable {
                    table_name: Identifier::new("my_table".to_string()).unwrap(),
                    actions: vec![
                        AlterTableAction::AlterColumn(AlterColumn {
                            column_name: Identifier::new("bool".to_string()).unwrap(),
                            actions: vec![
                                AlterColumnAction::SetType(DatabaseColumnType::String),
                                AlterColumnAction::SetNullability(true),
                            ]
                        }),
                        AlterTableAction::AlterColumn(AlterColumn {
                            column_name: Identifier::new("int".to_string()).unwrap(),
                            actions: vec![AlterColumnAction::SetType(DatabaseColumnType::Float),]
                        }),
                        AlterTableAction::AddColumn(TableColumn {
                            column_name: Identifier::new("new_column".to_string()).unwrap(),
                            column_type: DatabaseColumnType::String,
                            is_primary_key: false,
                            is_nullable: false,
                        }),
                        AlterTableAction::AlterColumn(AlterColumn {
                            column_name: Identifier::new("string_nullable".to_string()).unwrap(),
                            actions: vec![AlterColumnAction::SetNullability(false),]
                        }),
                        AlterTableAction::DropColumn(
                            Identifier::new("timestamp".to_string()).unwrap()
                        ),
                    ],
                },],
            }
        );

        println!("{}", migration.as_sql().unwrap());
    }
}
