use self::table_definition::DatabaseTableDefinition;

pub mod migration;
pub mod table_definition;

pub struct DatabaseDefiniton {
    pub name: String,
    pub tables: Vec<DatabaseTableDefinition>,
}
