pub mod database_definition;
pub mod queries;

trait AsSql {
    fn as_sql(&self) -> Result<String, String>;
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_migration_for_table() {
        // let database = DatabaseDefinition {
        //     name: "TestDatabase",
        // };
        // let result = add(2, 2);
        // assert_eq!(result, 4);
    }
}
