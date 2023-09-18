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

// derive_struct!(BuildRoutes, build_routes);

#[proc_macro_derive(BuildRoutes)]
pub fn derive_lower_name(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
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
