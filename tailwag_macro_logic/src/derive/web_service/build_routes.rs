use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use syn::{Data, DeriveInput, Field, FieldsNamed, Ident};

use crate::util::{self, attribute_parsing::GetAttribute};

pub fn derive_struct(input: &DeriveInput) -> TokenStream {
    let &DeriveInput {
        ident,
        data,
        ..
    } = &input;
    // Panic with error message if we get a non-struct
    let Data::Struct(data) = data else { panic!("Only Structs are supported") };

    match &data.fields {
        syn::Fields::Named(fields) => {
            let fields_filtered: Vec<&Field> = fields
                .named
                .iter()
                .filter(|field| {
                    // TODO: Abstract this for easier filtering
                    field.get_attribute("request_ignore").is_some() // TODO: `request(ignore)`
                        || field.ident.as_ref().expect("Unexpected unnamed field found").to_string()
                            != "id" // Always include "id"
                })
                .collect();
            let field_names_filtered: Vec<&Ident> = fields_filtered
                .iter()
                .map(|f| f.ident.as_ref().expect("Found missing ident for field: {}"))
                .collect();

            let parse_args_impl_tokens = quote!(
                #[axum::async_trait]
                impl tailwag::web::traits::rest_api::BuildRoutes<#ident> for #ident
                {
                    async fn build_routes(data_manager: tailwag::orm::data_manager::PostgresDataProvider<#ident>) -> axum::Router {
                        #[derive(serde::Deserialize)]
                        pub struct Request {
                            #(#fields_filtered),*
                        }
                        impl Into<#ident> for Request {
                            fn into(self) -> #ident {
                                #ident {
                                    // TODO: Don't assume ID here
                                    id: uuid::Uuid::new_v4(),
                                    #(#field_names_filtered: self.#field_names_filtered),*
                                    // ..Default::default() // Todo: Require default? Or how to handle non-magic fields?
                                }
                            }
                        }

                        pub async fn post_item(
                            axum::extract::State(data_manager): axum::extract::State<tailwag::orm::data_manager::PostgresDataProvider<#ident>>,
                            axum::extract::Json(request): axum::extract::Json<Request>,
                        ) -> axum::extract::Json<#ident> {
                            let item: #ident = request.into();
                            data_manager.create(&item).await.expect("Unable to create object");
                            axum::extract::Json(item)
                        }

                        pub async fn get_items(
                            axum::extract::State(data_manager): axum::extract::State<tailwag::orm::data_manager::PostgresDataProvider<#ident>>
                        ) -> axum::extract::Json<Vec<#ident>> {
                            // TODO: Revisit this when authorization rules are in place.
                            // TODO: Add filtering via query params
                            axum::extract::Json(data_manager.all().execute().await.unwrap())
                        }

                        data_manager.run_migrations().await.expect("Failed to run migrations");
                        axum::Router::new()
                            .route("/", axum::routing::method_routing::post(post_item))
                            .route("/", axum::routing::method_routing::get(get_items))
                            .with_state(data_manager)
                    }
                }
            );

            parse_args_impl_tokens.into()
        },
        syn::Fields::Unnamed(_) => unimplemented!("Unnamed fields not supported yet"),
        syn::Fields::Unit => unimplemented!("Unit fields not supported yet"),
    }
}
