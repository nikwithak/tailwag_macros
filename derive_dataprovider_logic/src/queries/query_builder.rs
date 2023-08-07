use chrono::Utc;
use uuid::Uuid;

use crate::{database_definition::table_definition::Identifier, AsSql};

struct Query<T: Queryable> {
    table_name: Identifier,
    filter: Filter,
    limit: Option<usize>,
    _t: T,
}

impl<T: Queryable> AsSql for Query<T> {
    fn as_sql(&self) -> String {
        // TODO: This needs to be unique, add millis or find a better way. What does SQLX do?
        let query_name = format!("{}_{}", Utc::now().format("%Y%m%d%H%M%S"), self.table_name);

        // TODO: Go through filters, build list of inputs ($1) -> values (literals)
        let mut preproc_stack = Vec::new();
        let mut postproc_stack = Vec::new();
        preproc_stack.push(&self.filter);
        while let Some(f) = preproc_stack.pop() {
            match f {
                Filter::And(children) | Filter::Or(children) => {
                    for child in children {
                        preproc_stack.push(child);
                    }
                },
                Filter::Comparison() => {}, // Ignore any comparisons - we'll catch it on the next pass, when we go backwards.
            }
            postproc_stack.push(f);
        }
        // Would this be easier if I just did it recursively instead of trying to build an iterative solution?
        // Depends if call stack will ever get deep enough (with a huuuuuuuuge amount of nested ands/ors)

        // TODO: Add support for joins with filters
        let comma_sep_input_list = "";
        let select_query = "SELECT * FROM {table_name} WHERE {filters}"; // TODO: Remove * in favor of a real type definition

        // Get the required inputs:

        let prepared_stmt = format!("PREPARE {} AS {};", query_name, select_query,);

        prepared_stmt
    }
}

impl<T: Queryable> Query<T> {
    fn execute() -> Vec<T> {
        todo!()
    }

    fn limit(
        mut self,
        limit: usize,
    ) -> Self {
        self.limit = Some(limit);
        self
    }

    fn filter(
        self,
        filter: Filter,
    ) -> Self {
        self
    }
}

/// struct  :: QueryBuilder  .fn
/// Products::all()          .filter(name.starts_with("...")).filter(create_date > chrono::now()).get();
///
///
///
trait Queryable {}

enum Filter {
    And(Vec<Filter>),
    Or(Vec<Filter>),
    Comparison(),
}

struct QueryBuilder<T: Queryable> {
    _t: T,
}

impl<T: Queryable> QueryBuilder<T> {
    fn get() -> Option<T> {
        todo!()
    }

    fn filter(filter: Filter) -> QueryBuilder<T> {
        todo!()
    }
}

struct Product {
    id: Uuid,
    name: String,
    description: String,
    create_time: chrono::DateTime<Utc>,
    modify_time: chrono::DateTime<Utc>,
}

enum FilterableTypes {
    Uuid,
    String,
    Boolean,
    DateTime,
}

impl PartialEq for FilterableTypes {
    fn eq(
        &self,
        other: &Self,
    ) -> bool {
        todo!()
    }

    fn ne(
        &self,
        other: &Self,
    ) -> bool {
        !self.eq(other)
    }
}

struct ProductFilters {
    id: Uuid,
    name: String,
    description: String,
    create_time: chrono::DateTime<Utc>,
    modify_time: chrono::DateTime<Utc>,
}

impl ProductFilters {}

impl Queryable for Product {}

impl Product {
    pub fn all() -> QueryBuilder<Product> {
        todo!()
    }
}

// #[cfg(test)]
mod tests {
    use chrono::DateTime;

    use super::Product;

    fn test() {
        // let query = Product::all().filter(create_date.before(DateTime::now()));
    }
}
