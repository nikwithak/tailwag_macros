use crate::{
    database_definition::table_definition::{
        DatabaseColumnType, DatabaseTableDefinition, Identifier, TableColumn,
    },
    AsSql,
};

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum AlterTableAction {
    Rename(Identifier),
    AddColumn(TableColumn), // TODO
    DropColumn(Identifier), // TODO
    AlterColumn(AlterColumn), // TODO
                            // TODO: Add the rest of the actions.
                            // Ref: https://www.postgresql.org/docs/current/sql-altertable.html
}

impl AsSql for AlterTableAction {
    fn as_sql(&self) -> Result<String, String> {
        match self {
            AlterTableAction::Rename(ident) => Ok(format!("RENAME TO {}", ident)),
            AlterTableAction::AddColumn(table_column) => {
                Ok(format!("ADD COLUMN IF NOT EXISTS {}", table_column.as_sql()?))
            },
            AlterTableAction::DropColumn(ident) => Ok(format!("DROP COLUMN IF EXISTS {}", ident)),
            AlterTableAction::AlterColumn(alter_column) => alter_column.as_sql(),
        }
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct AlterTable {
    pub table_name: Identifier,
    pub actions: Vec<AlterTableAction>,
}

impl AsSql for AlterTable {
    fn as_sql(&self) -> Result<String, String> {
        // Validate table name. Expected snake_case. Does not allow invalid characters.
        if !self.table_name.chars().all(|c| match c {
            'a'..='z' | 'A'..='Z' | '0'..='9' | '_' => true,
            _ => false,
        }) {
            return Err("table_name contains invalid characters. Only alphabetic and _ characters are allowed".to_string());
        }

        let statements = self
            .actions
            .iter()
            .map(|action| action.as_sql())
            .collect::<Result<Vec<String>, _>>()?
            .iter()
            .map(|action_sql| {
                format!("ALTER TABLE IF EXISTS {} {};", self.table_name.as_str(), &action_sql)
            })
            .collect::<Vec<String>>()
            .join("\n");

        Ok(statements)
    }
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

impl AsSql for AlterColumnAction {
    fn as_sql(&self) -> Result<String, String> {
        match self {
            AlterColumnAction::SetType(t) => Ok(format!("TYPE {}", t.as_str())),
            AlterColumnAction::SetNullability(nullable) => {
                #[rustfmt::skip]
                let verb = if *nullable { "DROP" } else { "SET" };
                Ok(format!("{} NOT NULL", verb))
            },
        }
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct AlterColumn {
    pub column_name: Identifier,
    pub actions: Vec<AlterColumnAction>,
}

impl AsSql for AlterColumn {
    fn as_sql(&self) -> Result<String, String> {
        let actions_sql = self
            .actions
            .iter()
            .map(|action| action.as_sql())
            .collect::<Result<Vec<String>, _>>()?
            .iter()
            .map(|action| format!("ALTER COLUMN {} {}", &self.column_name, &action))
            .collect::<Vec<String>>()
            .join(", ");

        Ok(actions_sql)
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Migration {
    pub table_actions: Vec<AlterTable>,
}

impl AsSql for Migration {
    fn as_sql(&self) -> Result<String, String> {
        let mut sql_statments = Vec::new();
        for alter_table in &self.table_actions {
            let statement = alter_table.as_sql()?;

            sql_statments.push(statement);
        }

        let statement = sql_statments.join("\n");
        Ok(statement)
    }
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
    ) -> Result<Option<Self>, String> {
        let mut actions = Vec::<AlterTableAction>::new();

        // Name changed
        if !(before.table_name == after.table_name) {
            actions.push(AlterTableAction::Rename(after.table_name.clone()));
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
                    actions.push(AlterTableAction::DropColumn(old.column_name.clone()));
                    old_column = old_sorted_iter.next();
                },
                (Some(old), Some(new)) => match old.column_name.cmp(&new.column_name) {
                    std::cmp::Ordering::Less => {
                        actions.push(AlterTableAction::DropColumn(old.column_name.clone()));
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
                                column_name: Identifier::new(new.column_name.to_string())?,
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
            Ok(Some(Self {
                table_actions: vec![AlterTable {
                    actions,
                    table_name: Identifier::new(before.table_name.to_string())?,
                }],
            }))
        } else {
            Ok(None)
        }
    }
}
