use derive_command_line_args_macro::PostgresDataProvider;
use std::env::Args;
use uuid::Uuid;

pub fn add(
    left: usize,
    right: usize,
) -> usize {
    left + right
}

trait PostgresDataProvider {
    // fn get(id: &str) -> Self;
    fn get<T: ?Sized>(receiver: &mut Vec<Box<T>>) -> Self
    where
        T: Into<Uuid>;
    fn build_create_table_query();
}

#[derive(PostgresDataProvider)]
struct Foo {
    // #[opts(long = "--long")]
    test: String,
    // #[opts(short = "-s")]
    not_test: String,
    // vec: Vec<String>,
    // mybool: bool,
}

impl Foo {
    pub fn parse_args(args: Vec<String>) {}
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn custom_short_and_long() {
        let args = vec!["--long", "arggggg1", "-s", "arg2"]
            .iter()
            .map(|s| s.to_string())
            .collect();

        let foo = Foo::parse_args(args);
        // assert_eq!(foo.test, "arggggg1");
        // assert_eq!(foo.not_test, "arg2");
    }

    #[test]
    fn default_short_and_long_options() {
        let args = vec!["--test", "arggggg1", "-n", "arg2"]
            .iter()
            .map(|s| s.to_string())
            .collect();

        let foo = Foo::parse_args(args);
        // assert_eq!(foo.test, "arggggg1");
        // assert_eq!(foo.not_test, "arg2");
    }
}
