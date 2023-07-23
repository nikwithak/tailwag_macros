mod _derive_builder;
mod derive_logic;
mod derive_struct;
mod utils;

use derive_logic::derive_impl;
use quote::quote;
use syn::{parse_macro_input, Data, DataStruct, DeriveInput, Field};

#[proc_macro_derive(PostgresDataProvider, attributes(opts))]
pub fn derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input);

    // Reads each attribute in as the proper type

    let impl_trait_tokens = derive_impl(&input);

    quote!(
        #impl_trait_tokens
    )
    .into()
}
