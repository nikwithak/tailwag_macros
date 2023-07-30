pub use derive_dataprovider_macros::PostgresDataProvider;

pub fn add(
    left: usize,
    right: usize,
) -> usize {
    left + right
}

pub trait PostgresDataProvider {
    // fn get(id: &str) -> Self;
    // fn get<T: ?Sized>(receiver: &mut Vec<Box<T>>) -> Self
    // where
    //     T: Into<Uuid>;
    fn build_create_table_query() -> String;
    fn build_list_table_query() -> String;
    fn build_insert_table_query() -> String;
    fn make_migrations();
    // fn get_by_id(id: &Uuid);
    // fn list(id: &Uuid);
    // fn update(); // OR self.save()?
    // save();
    // fn bulk_save(Vec<T>);
    // fn get_with_filter(filter: Filter);
    // fn query_builder() -> QueryBuilder;
}

#[derive(PostgresDataProvider)]
struct Foo {
    //     // #[opts(long = "--long")]
    my_string: String,
    //     // #[opts(short = "-s")]
    //     my_option: std::option::Option<String>,
    //     my_int: std::option::Option<u32>,
    //     my_float: Option<f64>,
    //     // vec: Vec<String>,
    //     // mybool: bool,
}

// impl Foo {
//     pub fn parse_args(_args: Vec<String>) {}
// }

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(1 + 1, 2,);
    }
}
