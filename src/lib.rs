pub use tailwag_macro_inline::*;

#[cfg(feature = "orm")]
#[cfg(features = "no_orm")]
panic!("Cannot have both orm & no_orm features enabled");

#[cfg(feature = "orm")]
mod orm {
    pub use tailwag_macro_exports::Deleteable;
    pub use tailwag_macro_exports::GetTableDefinition;
    pub use tailwag_macro_exports::Insertable;
    pub use tailwag_macro_exports::Queryable;
    pub use tailwag_macro_exports::Updateable;
}
#[cfg(feature = "orm")]
pub use orm::*;

#[cfg(feature = "orm")]
mod gui {
    pub use tailwag_macro_exports::AsEguiForm;
}
#[cfg(feature = "orm")]
pub use gui::*;

pub use tailwag_macro_exports::Deref;
pub use tailwag_macro_exports::Display;
pub use tailwag_macro_exports::FromStr;

// pub use tailwag_macro_exports::BuildCreateRoute;
// pub use tailwag_macro_exports::BuildListGetRoute;
pub use tailwag_macro_exports::BuildRoutes;
