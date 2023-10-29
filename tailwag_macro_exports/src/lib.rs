use syn::parse_macro_input;

// macro_rules! derive_trait {
//     ($TraitName:ident, $function:item) => {
//         #[proc_macro_derive($TraitName)]
//         pub fn derive_$TraitName(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
//             let input = parse_macro_input!(input);
//             let impl_trait_tokens = tailwag_macro_logic::derive::deref::$function(&input);
//             impl_trait_tokens.into()
//         }
//     };
// }
// TODO: Not working yet
// derive_trait!(Deref, derive_deref);

#[allow(unused)]
macro_rules! derive_struct {
    ($struct_name:ident, $lower_name:ident) => {
        #[proc_macro_derive($struct_name)]
        pub fn derive_$lower_name(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
            let input = parse_macro_input!(input);
            let impl_trait_tokens = tailwag_macro_logic::derive::$lower_name::derive_struct(&input);
            impl_trait_tokens.into()
        }
    };
}

#[proc_macro_derive(FromStr)]
pub fn derive_from_str(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input);
    let impl_trait_tokens = tailwag_macro_logic::derive::from_string::derive_trait(&input);
    impl_trait_tokens.into()
}

#[proc_macro_derive(AsEguiForm)]
pub fn derive_as_egui_form(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input);
    let impl_trait_tokens = tailwag_macro_logic::derive::gui::as_egui_form::derive_struct(&input);
    impl_trait_tokens.into()
}

#[proc_macro_derive(IntoForm)]
pub fn derive_to_form(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input);
    let impl_trait_tokens = tailwag_macro_logic::derive::gui::into_form::derive_struct(&input);
    impl_trait_tokens.into()
}

#[proc_macro_derive(Display)]
pub fn derive_display(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input);
    let impl_trait_tokens = tailwag_macro_logic::derive::display::derive_display(&input);
    impl_trait_tokens.into()
}

#[proc_macro_derive(BuildRoutes)]
pub fn derive_build_routes(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input);
    let impl_trait_tokens = tailwag_macro_logic::derive::build_routes::derive_struct(&input);
    impl_trait_tokens.into()
}

/// Wraps a function with inputs/outputs for a `syn` / `quote`
#[proc_macro_derive(Deref, attributes(deref))]
pub fn derive_deref(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input);
    let impl_trait_tokens = tailwag_macro_logic::derive::deref::derive_deref(&input);
    impl_trait_tokens.into()
}

/// Wraps a function with inputs/outputs for a `syn` / `quote`
#[cfg(feature = "orm")] // TODO: I should really just yank it to separate crates
#[proc_macro_derive(Queryable)]
pub fn derive_queryable(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input);
    let impl_trait_tokens = tailwag_macro_logic::derive::queryable::derive_struct(&input);
    impl_trait_tokens.into()
}

#[cfg(feature = "orm")] // TODO: I should really just yank it to separate crates
#[proc_macro_derive(GetTableDefinition)]
pub fn derive_get_table_definition(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input);
    let impl_trait_tokens =
        tailwag_macro_logic::derive::get_table_definition::derive_struct(&input);
    impl_trait_tokens.into()
}

#[cfg(feature = "orm")] // TODO: I should really just yank it to separate crates
#[proc_macro_derive(Deleteable)]
pub fn derive_deleteable(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input);
    let impl_trait_tokens = tailwag_macro_logic::derive::deleteable::derive_struct(&input);
    impl_trait_tokens.into()
}

#[cfg(feature = "orm")] // TODO: I should really just yank it to separate crates
#[proc_macro_derive(Updateable)]
pub fn derive_updateable(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input);
    let impl_trait_tokens = tailwag_macro_logic::derive::updateable::derive_struct(&input);
    impl_trait_tokens.into()
}

#[cfg(feature = "orm")] // TODO: I should really just yank it to separate crates
#[proc_macro_derive(Insertable)]
pub fn derive_insertable(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input);
    let impl_trait_tokens = tailwag_macro_logic::derive::insertable::derive_struct(&input);
    impl_trait_tokens.into()
}

#[cfg(feature = "orm")] // TODO: I should really just yank it to separate crates
#[proc_macro_derive(BuildCrudRoutes)]
pub fn derive_build_crud_routes(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input);
    let impl_trait_tokens = tailwag_macro_logic::derive::build_routes::derive_struct(&input);
    impl_trait_tokens.into()
}
