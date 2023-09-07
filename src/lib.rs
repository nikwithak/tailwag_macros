pub use tailwag_macros_impl::GetTableDefinition;
pub use tailwag_macros_impl::Insertable;
pub use tailwag_macros_impl::Queryable;

#[macro_export]
macro_rules! derive_magic {
    ($i:item) => {
        #[derive(
            serde::Deserialize,
            serde::Serialize,
            sqlx::FromRow,
            Clone,
            tailwag::macros::Queryable,
            tailwag::macros::GetTableDefinition,
            tailwag::macros::Insertable,
        )]
        $i
    };
}
