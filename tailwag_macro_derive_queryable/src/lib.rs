use syn::parse_macro_input;

/// Wraps a function with inputs/outputs for a `syn` / `quote`
#[proc_macro_derive(Queryable, attributes(opts))]
pub fn derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input);

    // Store the logic separately from defining the macro - this gives an easy way to include things like in-library macros (mostly, via imported function), and
    let impl_trait_tokens = tailwag_macro_logic::derive::queryable::derive_struct(&input);

    impl_trait_tokens.into()
}
