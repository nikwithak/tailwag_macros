use crate::{
    database_definition::table_definition::{DatabaseColumnType, Identifier, TableColumn},
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
