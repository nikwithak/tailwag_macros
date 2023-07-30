mod migration;
pub use migration::*;

#[cfg(test)]
mod tests {
    use migration::{AlterColumn, AlterColumnAction, AlterTable, AlterTableAction};

    use crate::database_definition::table_definition::{
        DatabaseColumnType, DatabaseTableDefinition, TableColumn,
    };

    use super::{migration, Migration};

    fn get_before() -> DatabaseTableDefinition {
        DatabaseTableDefinition {
            table_name: "my_table".to_string(),
            columns: vec![
                TableColumn {
                    column_name: "string_nullable".to_string(),
                    column_type: DatabaseColumnType::String,
                    is_primary_key: false,
                    is_nullable: true,
                },
                TableColumn {
                    column_name: "bool".to_string(),
                    column_type: DatabaseColumnType::Boolean,
                    is_primary_key: false,
                    is_nullable: false,
                },
                TableColumn {
                    column_name: "int".to_string(),
                    column_type: DatabaseColumnType::Int,
                    is_primary_key: false,
                    is_nullable: false,
                },
                TableColumn {
                    column_name: "float".to_string(),
                    column_type: DatabaseColumnType::Float,
                    is_primary_key: false,
                    is_nullable: false,
                },
                TableColumn {
                    column_name: "timestamp".to_string(),
                    column_type: DatabaseColumnType::Timestamp,
                    is_primary_key: false,
                    is_nullable: false,
                },
                TableColumn {
                    column_name: "uuid".to_string(),
                    column_type: DatabaseColumnType::Uuid,
                    is_primary_key: false,
                    is_nullable: false,
                },
            ],
        }
    }

    #[test]
    fn new_from_table_definitions_modifies() {
        // Arrange
        let before = get_before();
        let mut after = before.clone();
        after
            .columns
            .iter_mut()
            .find(|c| c.column_name.eq("int"))
            // Tests Type changes
            .map(|c| c.column_type = DatabaseColumnType::Float);
        after.columns.iter_mut().find(|c| c.column_name.eq("string_nullable")).map(|c| {
            // Tests Nullability changes
            c.is_nullable = false;
        });
        after.columns.iter_mut().find(|c| c.column_name.eq("bool")).map(|c| {
            // Tests a mix of the two changes
            c.column_type = DatabaseColumnType::String;
            c.is_nullable = true;
        });
        after.columns.push(TableColumn {
            column_name: "new_column".to_string(),
            column_type: DatabaseColumnType::String,
            is_primary_key: false,
            is_nullable: false,
        });
        // Delete "timestamp" column
        after.columns =
            after.columns.into_iter().filter(|c| !c.column_name.eq("timestamp")).collect();

        // Act
        let migration = Migration::new_from_table_definitions(&before, &after).unwrap();

        // Assert
        assert_eq!(
            migration,
            Migration {
                table_actions: vec![AlterTable {
                    table_name: "my_table".to_string(),
                    actions: vec![
                        AlterTableAction::AlterColumn(AlterColumn {
                            column_name: "bool".to_string(),
                            actions: vec![
                                AlterColumnAction::SetType(DatabaseColumnType::String),
                                AlterColumnAction::SetNullability(true),
                            ]
                        }),
                        AlterTableAction::AlterColumn(AlterColumn {
                            column_name: "int".to_string(),
                            actions: vec![AlterColumnAction::SetType(DatabaseColumnType::Float),]
                        }),
                        AlterTableAction::AddColumn(TableColumn {
                            column_name: "new_column".to_string(),
                            column_type: DatabaseColumnType::String,
                            is_primary_key: false,
                            is_nullable: false,
                        }),
                        AlterTableAction::AlterColumn(AlterColumn {
                            column_name: "string_nullable".to_string(),
                            actions: vec![AlterColumnAction::SetNullability(false),]
                        }),
                        AlterTableAction::DropColumn("timestamp".to_string()),
                    ],
                },],
            }
        );
    }
}
