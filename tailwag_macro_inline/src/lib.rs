pub mod template_macros;

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

// #[macro_export]
// macro_rules! deref_arc {
//     () => {
//         // TODO
//     };
// }

// Just a quick macro for macroing macros
#[macro_export]
macro_rules! m {
    ($name:ident) => {
        macro_rules! $name {
            ($i:item) => {};
            ($ident:ident) => {};
        }
    };
}

#[macro_export]
macro_rules! impl_deref {
    ($target:ident for $struct:ident) => {
        impl Deref for $struct {
            type Target = $target;

            fn deref(&self) -> &Self::Target {
                &self.$target
            }
        }
    };
}
