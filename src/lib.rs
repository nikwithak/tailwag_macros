pub use tailwag_macro_inline::*;

#[cfg(feature = "orm")]
#[cfg(features = "no_orm")]
panic!("Cannot have both orm & no_orm features enabled");

#[cfg(feature = "orm")]
pub use tailwag_macro_exports::GetTableDefinition;
#[cfg(feature = "orm")]
pub use tailwag_macro_exports::Insertable;
#[cfg(feature = "orm")]
pub use tailwag_macro_exports::Queryable;

pub use tailwag_macro_exports::Deref;

pub use tailwag_macro_exports::BuildCrudRoutes;
