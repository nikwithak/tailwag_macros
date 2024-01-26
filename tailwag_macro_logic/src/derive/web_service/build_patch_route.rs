use proc_macro2::TokenStream;
use quote::quote;
use syn::{Data, DeriveInput};

pub fn derive_struct(input: &DeriveInput) -> TokenStream {
    let &DeriveInput {
        ident,
        data,
        ..
    } = &input;
    // Panic with error message if we get a non-struct
    let Data::Struct(data) = data else { panic!("Only Structs are supported") };

    match &data.fields {
        syn::Fields::Named(_fields) => {
            let parse_args_impl_tokens = quote!(
                impl tailwag::web::traits::rest_api::BuildListPatchRoute for #ident
                {
                    fn build_list_get_route() -> axum::Router {
                        pub async fn list_items(
                            axum::extract::State(data_manager): axum::extract::State<tailwag::orm::data_manager::PostgresDataProvider<#ident>>
                        ) -> axum::extract::Json<Vec<#ident>> {
                            // TODO: Revisit this when authorization rules are in place.
                            // TODO: Add filtering via query params
                            axum::extract::Json(data_manager.all().execute().await.unwrap())
                        }

                        axum::Router::new()
                            .route("/", axum::routing::method_routing::get(list_items))
                    }
                }
            );

            parse_args_impl_tokens.into()
        },
        syn::Fields::Unnamed(_) => unimplemented!("Unnamed fields not supported yet"),
        syn::Fields::Unit => unimplemented!("Unit fields not supported yet"),
    }
}
