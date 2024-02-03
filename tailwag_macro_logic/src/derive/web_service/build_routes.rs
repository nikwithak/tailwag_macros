use proc_macro2::TokenStream;
use quote::quote;
use syn::{Data, DeriveInput, Field, Ident};

use crate::util::attribute_parsing::GetAttribute;

pub fn derive_struct(input: &DeriveInput) -> TokenStream {
    let &DeriveInput {
        ident,
        data,
        ..
    } = &input;
    // Panic with error message if we get a non-struct
    let Data::Struct(data) = data else {
        panic!("Only Structs are supported")
    };

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
            let field_types_filtered: Vec<&syn::Type> =
                fields_filtered.iter().map(|f| &f.ty).collect();

            let parse_args_impl_tokens = quote!(
                #[axum::async_trait]
                impl tailwag::web::traits::rest_api::BuildRoutes<#ident> for #ident
                {
                    async fn build_routes(
                        data_manager: tailwag::orm::data_manager::PostgresDataProvider<#ident>,
                    ) -> axum::Router {
                        #[derive(serde::Deserialize)]
                        pub struct Request {
                            #(#field_names_filtered: #field_types_filtered),*
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
                            println!("In the function");
                            let item: #ident = request.into();
                            println!("before:");
                            let item = tailwag::orm::data_manager::traits::DataProvider::<#ident>::create(&data_manager, item).await.expect("Unable to create object");
                            // let item = data_manager.create(item).await.expect("Unable to create object");
                            println!("after:");
                            axum::extract::Json(item)
                        }

                        pub async fn get_items(
                            axum::extract::State(data_manager): axum::extract::State<tailwag::orm::data_manager::PostgresDataProvider<#ident>>,
                            // axum::extract::Extension(auth_token): axum::extract::Extension<String> // TODO: Remove This
                        ) -> axum::extract::Json<Vec<#ident>> {
                            // TODO: Revisit this when authorization rules are in place.
                            // TODO: Add filtering via query params
                            axum::extract::Json(tailwag::orm::data_manager::traits::DataProvider::<#ident>::all(&data_manager).await.unwrap().execute().await.unwrap())
                            // axum::extract::Json(data_manager.all().await.unwrap().execute().await.unwrap())
                        }

                        pub async fn update_item(
                            axum::extract::State(data_manager): axum::extract::State<tailwag::orm::data_manager::PostgresDataProvider<#ident>>,
                            axum::extract::Json(request): axum::extract::Json<#ident>,
                        ) -> axum::extract::Json<#ident> {
                            let item: #ident = request.into();
                            tailwag::orm::data_manager::traits::DataProvider::<#ident>::update(&data_manager, &item).await.expect("Unable to update object");
                            // data_manager.update( &item).await.expect("Unable to update object");
                            axum::extract::Json(item)
                        }

                        pub async fn delete_item(
                            axum::extract::State(data_manager): axum::extract::State<tailwag::orm::data_manager::PostgresDataProvider<#ident>>,
                            axum::extract::Json(request): axum::extract::Json<#ident>,
                        )  {
                            let item: #ident = request.into();
                            let item = tailwag::orm::data_manager::traits::DataProvider::<#ident>::delete(&data_manager, item).await.expect("Unable to delete object");
                            // let item = data_manager.delete( item).await.expect("Unable to delete object");
                        }

                        data_manager.run_migrations().await.expect("Failed to run migrations");
                        axum::Router::new()
                            .route("/", axum::routing::method_routing::post(post_item))
                            .route("/", axum::routing::method_routing::get(get_items))
                            .route("/", axum::routing::method_routing::patch(update_item))
                            .route("/", axum::routing::method_routing::delete(delete_item))
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
