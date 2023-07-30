pub mod database_definition;

#[cfg(test)]
mod tests {
    use crate::database_definition::DatabaseDefiniton;

    use super::*;

    #[test]
    fn test_migration_for_table() {
        let database = DatabaseDefinition {
            name: "TestDatabase",
        };
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
