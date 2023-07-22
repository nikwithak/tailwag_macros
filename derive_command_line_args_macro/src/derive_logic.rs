use darling::FromDeriveInput;
use proc_macro2::TokenStream;
use quote::{format_ident, quote, quote_spanned, ToTokens, TokenStreamExt};
use syn::{
    parse_macro_input, spanned::Spanned, AttrStyle, Data, DataStruct, DeriveInput, Field, LitStr,
};

use crate::derive_struct::derive_struct;

#[derive(FromDeriveInput)]
#[darling(default, attributes(cli_args))]
struct Opts {
    message: String,
}

impl Default for Opts {
    fn default() -> Self {
        Opts {
            message: "Hello!".into(),
        }
    }
}

pub fn derive_impl(input: &DeriveInput) -> TokenStream {
    let &DeriveInput {
        ident,
        data,
        ..
    } = &&input;

    // Panic if we aren't using a Struct -- other types are not supported.
    let tokens = match data {
        Data::Struct(data) => derive_struct(input),
        Data::Enum(_) => unimplemented!("Not implemented for enums"),
        Data::Union(_) => unimplemented!("Not implemented for unions"),
    };

    tokens
}
