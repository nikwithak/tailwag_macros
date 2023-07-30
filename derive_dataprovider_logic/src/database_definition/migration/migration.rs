use std::collections::HashSet;

use crate::database_definition::table_definition::{
    DatabaseColumnType, DatabaseTableDefinition, TableColumn,
};

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct RenameTable {
    before: String,
    after: String,
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum AlterTableAction {
    Rename(RenameTable),
    AddColumn(TableColumn), // TODO
    DropColumn(String),     // TODO
    AlterColumn(AlterColumn), // TODO
                            // TODO: Add the rest of the actions.
                            // Ref: https://www.postgresql.org/docs/current/sql-altertable.html
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct AlterTable {
    // TODO: Always add IF EXISTS, so we won't store it here.
    // Ignore ONLY keyword, no use cases for it at the time.
    pub table_name: String,
    pub actions: Vec<AlterTableAction>,
}

// TODO: Part of _SetStorage() requirement
// enum StorageType {
//     Plain,
//     External,
//     Extended,
//     Main,
// }

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum AlterColumnAction {
    SetType(DatabaseColumnType), // TODO: Look at the other options here
    SetNullability(bool),        // True if nullable, False if not.
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

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct AlterColumn {
    pub column_name: String,
    pub actions: Vec<AlterColumnAction>,
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Migration {
    pub table_actions: Vec<AlterTable>,
}

impl Migration {
    /// Returns `Some<Migration>` representing the steps required to go from `before` to `after`, or None if the inputs are the same.
    ///
    /// # Arguments
    ///
    /// * `before` - The existing table definition, before the new changes are applied.
    /// * `after` - The new table definition, currently in use.
    ///
    /// # Returns
    ///
    pub fn new_from_table_definitions(
        before: &DatabaseTableDefinition,
        after: &DatabaseTableDefinition,
    ) -> Option<Self> {
        let mut actions = Vec::<AlterTableAction>::new();

        // Name changed
        if !(before.table_name == after.table_name) {
            actions.push(AlterTableAction::Rename(RenameTable {
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

        for c in &before_columns_sorted {
            println!("{:?}", c);
        }
        println!("=========================");
        for c in &after_columns_sorted {
            println!("{:?}", c);
        }

        // Second, iterate through both. Any out-of-sync issues indicate a column only in that set.
        let mut old_sorted_iter = before_columns_sorted.into_iter().peekable();
        let mut new_sorted_iter = after_columns_sorted.into_iter().peekable();

        let mut old_column = old_sorted_iter.next();
        let mut new_column = new_sorted_iter.next();

        while old_column.is_some() || new_column.is_some() {
            match (old_column, new_column) {
                (None, Some(new)) => {
                    // new = an entirely new column
                    actions.push(AlterTableAction::AddColumn(new.clone()));
                    new_column = new_sorted_iter.next();
                },
                (Some(old), None) => {
                    // old = removed (we never found a match)
                    actions.push(AlterTableAction::DropColumn(old.column_name.to_string()));
                    old_column = old_sorted_iter.next();
                },
                (Some(old), Some(new)) => match old.column_name.cmp(&new.column_name) {
                    std::cmp::Ordering::Less => {
                        actions.push(AlterTableAction::DropColumn(old.column_name.to_string()));
                        old_column = old_sorted_iter.next();
                    },
                    std::cmp::Ordering::Greater => {
                        actions.push(AlterTableAction::AddColumn(new.clone()));
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

                        if alter_column_actions.len() > 0 {
                            actions.push(AlterTableAction::AlterColumn(AlterColumn {
                                column_name: new.column_name.to_string(),
                                actions: alter_column_actions,
                            }));
                        }

                        new_column = new_sorted_iter.next();
                        old_column = old_sorted_iter.next();
                    },
                },
                (None, None) => panic!("Should not ever reach here"),
            }
        }

        if actions.len() > 0 {
            Some(Self {
                table_actions: vec![AlterTable {
                    actions,
                    table_name: before.table_name.to_string(),
                }],
            })
        } else {
            None
        }
    }
}
