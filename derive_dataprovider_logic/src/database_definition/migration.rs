use std::collections::HashSet;

use crate::database_definition::table_definition::TableColumn;

use super::table_definition::{DatabaseColumnType, DatabaseTableDefinition};

struct Migration {}

struct RenameTable {
    before: String,
    after: String,
}

enum AlterTableAction {
    Rename(RenameTable),
    AddColumn(TableColumn), // TODO
    DropColumn(String),     // TODO
    AlterColumn(AlterColumn), // TODO
                            // TODO: Add the rest of the actions.
                            // Ref: https://www.postgresql.org/docs/current/sql-altertable.html
}

struct AlterTable {
    // Always add IF EXISTS, so we won't store it here.
    // Ignore ONLY keyword, no use cases for it at the time.
    table_name: String,
    actions: Vec<AlterTableAction>,
}

// enum StorageType {
//     Plain,
//     External,
//     Extended,
//     Main,
// }

enum AlterColumnAction {
    SetType(DatabaseColumnType), // TODO: Look at the other options here
    SetDefault(Option<String>), // TODO: Fix hardcoded "String" type. Call DropDefault if input is None
    SetNullability(bool),       // True if nullable, False if not.
                                // _DropExpression,            // TODO: Unsupported yet
                                // _AddGenerated(),            // TODO: Unsupported yet
                                // _SetGenerated(),            // TODO: Unsupported yet
                                // _DropIdentity,              // TODO: Unsupported yet Always use IF EXISTS
                                // _SetStatistics(i64),        // TODO: Unsupported yet
                                // _SetAttribute(),            // TODO: Unsupported yet
                                // _Reset(),                   // TODO: Unsupported yet
                                // _SetStorage(StorageType),   // TODO: Unsupported yet
                                // _SetCompression(_CompressionMethod), // TODO: Unsupported yet
}

struct AlterColumn {
    column_name: String,
    actions: Vec<AlterColumnAction>,
}

impl Migration {
    fn to_sql() -> String {
        todo!()
    }

    pub fn from(
        before: &DatabaseTableDefinition,
        after: &DatabaseTableDefinition,
    ) -> Self {
        let mut results = Vec::<AlterTableAction>::new();

        // Name changed
        if !(before.table_name == after.table_name) {
            results.push(AlterTableAction::Rename(RenameTable {
                before: before.table_name.to_string(),
                after: after.table_name.to_string(),
            }));
        }

        // Figure out added / removed / changed columns
        // First sort them, so we can iterate in one pass.
        let mut before_columns_sorted = before.columns.iter().collect::<Vec<&TableColumn>>();
        before_columns_sorted.sort_by(|l, r| l.column_name.cmp(&r.column_name));
        let mut after_columns_sorted = after.columns.iter().collect::<Vec<&TableColumn>>();
        after_columns_sorted.sort_by(|l, r| l.column_name.cmp(&r.column_name));

        // Second, iterate through both. Any out-of-sync issues indicate a column only in that set.
        let mut old_sorted_iter = before_columns_sorted.into_iter().peekable();
        let mut new_sorted_iter = after_columns_sorted.into_iter().peekable();

        let mut old_column = old_sorted_iter.next();
        let mut new_column = new_sorted_iter.next();

        while old_column.is_some() || new_column.is_some() {
            match (old_column, new_column) {
                (None, Some(new)) => {
                    // new = an entirely new column
                    results.push(AlterTableAction::AddColumn(new.clone()));
                    new_column = new_sorted_iter.next();
                },
                (Some(old), None) => {
                    // old = removed (we never found a match)
                    results.push(AlterTableAction::DropColumn(old.column_name.to_string()));
                    old_column = new_sorted_iter.next();
                },
                (Some(old), Some(new)) => match old.column_name.cmp(&new.column_name) {
                    std::cmp::Ordering::Less => {
                        results.push(AlterTableAction::DropColumn(old.column_name.to_string()));
                        old_column = new_sorted_iter.next();
                    },
                    std::cmp::Ordering::Greater => {
                        results.push(AlterTableAction::AddColumn(new.clone()));
                        new_column = new_sorted_iter.next();
                    },
                    std::cmp::Ordering::Equal => {
                        // TODO move this logic into TableColumn? Or AlterColumnAction?
                        let mut alter_column_actions = Vec::new();
                        if !old.column_type.eq(&new.column_type) {
                            alter_column_actions
                                .push(AlterColumnAction::SetType(new.column_type.clone()));
                        }
                        if old.is_nullable != new.is_nullable {
                            alter_column_actions
                                .push(AlterColumnAction::SetNullability(new.is_nullable));
                        }

                        results.push(AlterTableAction::AlterColumn(AlterColumn {
                            column_name: new.column_name.to_string(),
                            actions: alter_column_actions,
                        }));

                        new_column = new_sorted_iter.next();
                        old_column = old_sorted_iter.next();
                    },
                },
                (None, None) => break, // Shouldn't reach here, but in case we change the loop structure
            }
        }

        Self {
            // before,
            // after,
        }
    }
}
