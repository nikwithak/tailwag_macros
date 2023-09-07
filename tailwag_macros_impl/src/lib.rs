use syn::parse_macro_input;

/// Wraps a function with inputs/outputs for a `syn` / `quote`
#[proc_macro_derive(Queryable, attributes(opts))]
pub fn derive_queryable(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input);

    // Store the logic separately from defining the macro - this gives an easy way to include things like in-library macros (mostly, via imported function), and
    let impl_trait_tokens = tailwag_macro_logic::derive::queryable::derive_struct(&input);

    impl_trait_tokens.into()
}

#[proc_macro_derive(GetTableDefinition, attributes(opts))]
pub fn derive_get_table_definition(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input);

    // Store the logic separately from defining the macro - this gives an easy way to include things like in-library macros (mostly, via imported function), and
    let impl_trait_tokens =
        tailwag_macro_logic::derive::get_table_definition::derive_struct(&input);

    impl_trait_tokens.into()
}

#[proc_macro_derive(Insertable, attributes(opts))]
pub fn derive_insertable(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input);

    // Store the logic separately from defining the macro - this gives an easy way to include things like in-library macros (mostly, via imported function), and
    let impl_trait_tokens = tailwag_macro_logic::derive::insertable::derive_struct(&input);

    impl_trait_tokens.into()
}
