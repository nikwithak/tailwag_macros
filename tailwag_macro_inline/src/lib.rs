pub mod template_macros;

#[macro_export]
macro_rules! derive_magic {
    ($i:item) => {
        #[derive(
            Clone, // Needed to be able to create an editable version from an Arc<Brewery> without affecting the saved data.
            Debug,
            Default,
            serde::Deserialize,                  // Needed for API de/serialization
            serde::Serialize,                    // Needed for API de/serialization
            sqlx::FromRow,                       // Needed for DB connectivity
            tailwag::macros::GetTableDefinition, // Creates the data structure needed for the ORM to work.
            tailwag::macros::Insertable,
            tailwag::macros::Updateable,
            tailwag::macros::Deleteable,
            tailwag::macros::Filterable,
            tailwag::macros::BuildRoutes, // Creates the functions needed for a REST service (full CRUD)
            tailwag::macros::Id,
            // tailwag::macros::AsEguiForm, // Renders the object into an editable form for an egui application.
            tailwag::macros::Display,
            tailwag::forms::macros::GetForm,
        )]
        $i
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

#[macro_export]
macro_rules! time_exec {
    ($block:expr) => {{
        let now = std::time::Instant::now();
        let result = $block;
        let elapsed = now.elapsed();
        println!("Timed at {:.2?}", elapsed);
        result
    }};
    ($name:literal, $block:expr) => {{
        let now = std::time::Instant::now();
        let result = $block;
        let elapsed = now.elapsed();
        println!("[{}]: Timed at {:.2?}", $name, elapsed);
        result
    }};
}
