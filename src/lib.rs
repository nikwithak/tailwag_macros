pub use tailwag_macro_inline::*;

#[cfg(feature = "orm")]
#[cfg(features = "no_orm")]
panic!("Cannot have both orm & no_orm features enabled");

// #[cfg(feature = "orm")]
pub use tailwag_orm_macros;
// #[cfg(feature = "orm")]
mod orm {
    pub use tailwag_orm_macros::*;
}
// #[cfg(feature = "orm")]
pub use orm::*;

// #[cfg(feature = "gui")]
mod gui {
    pub use tailwag_macro_exports::AsEguiForm;
    pub use tailwag_macro_exports::IntoForm;
}
// #[cfg(feature = "gui")]
pub use gui::*;

pub use tailwag_macro_exports::Deref;
pub use tailwag_macro_exports::DerefMut;
pub use tailwag_macro_exports::Display;
pub use tailwag_macro_exports::FromStr;

// pub use tailwag_macro_exports::BuildCreateRoute;
// pub use tailwag_macro_exports::BuildListGetRoute;
pub use tailwag_macro_exports::BuildRoutes;
