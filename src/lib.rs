pub use tailwag_macro_inline::*;

#[cfg(feature = "orm")]
#[cfg(features = "no_orm")]
panic!("Cannot have both orm & no_orm features enabled");

#[cfg(feature = "orm")]
pub use tailwag_macro_exports::Deleteable;
#[cfg(feature = "orm")]
pub use tailwag_macro_exports::GetTableDefinition;
#[cfg(feature = "orm")]
pub use tailwag_macro_exports::Insertable;
#[cfg(feature = "orm")]
pub use tailwag_macro_exports::Queryable;
#[cfg(feature = "orm")]
pub use tailwag_macro_exports::Updateable;

pub use tailwag_macro_exports::Deref;
pub use tailwag_macro_exports::Display;
pub use tailwag_macro_exports::FromStr;

// pub use tailwag_macro_exports::BuildCreateRoute;
// pub use tailwag_macro_exports::BuildListGetRoute;
pub use tailwag_macro_exports::BuildRoutes;
