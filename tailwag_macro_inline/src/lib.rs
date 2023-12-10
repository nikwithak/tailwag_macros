pub mod template_macros;

#[macro_export]
macro_rules! derive_magic {
    ($i:item) => {
        #[derive(
            serde::Deserialize,
            serde::Serialize,
            sqlx::FromRow,
            Clone,
            tailwag::macros::GetTableDefinition,
            tailwag::macros::Updateable,
            tailwag::macros::Deleteable,
            tailwag::macros::Insertable,
            Debug,
            tailwag::macros::BuildRoutes, // Creates the functions needed for a REST service (full CRUD)
            tailwag::macros::AsEguiForm, // Renders the object into an editable form for an egui application.
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
