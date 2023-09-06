mod _derive_builder;
// mod database_table_definition;
mod derive_logic;
mod derive_struct;

mod database_table_definition;

use derive_logic::derive_impl;
use syn::parse_macro_input;

/// Wraps a function with inputs/outputs for a `syn` / `quote`
#[proc_macro_derive(Insertable, attributes(opts))]
pub fn derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input);

    // Store the logic separately from defining the macro - this gives an easy way to include things like in-library macros (mostly, via imported function), and
    let impl_trait_tokens = derive_impl(&input);

    impl_trait_tokens.into()
}
