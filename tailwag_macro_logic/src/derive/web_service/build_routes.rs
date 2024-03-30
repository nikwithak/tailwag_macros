use proc_macro2::TokenStream;
use quote::quote;
use syn::{punctuated::Punctuated, Data, DeriveInput, Field, Ident, Token};

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
    let actions = input
        .get_attribute("actions")
        .map(|attr| attr.parse_args_with(Punctuated::<Ident, Token![,]>::parse_terminated).unwrap())
        .map(|params| params.into_iter().collect::<Vec<_>>())
        .unwrap_or(Vec::new());
    // Is this really how the only way to inject tokens into quote! with surrounding quotes?
    let action_paths = actions.iter().map(|s| s.to_string());

    // list.parse_args_with(Punctuated::<LitStr, Token![,]>::parse_terminated)
    // let args_parsed = syn::punctuated::Punctuated::<syn::Path, syn::Token![,]>::parse_terminated
    // .
    // .unwrap();

    match &data.fields {
        syn::Fields::Named(fields) => {
            let fields_filtered: Vec<&Field> = fields
                .named
                .iter()
                .filter(|field| {
                    // TODO: Abstract this for easier filtering
                    field.get_attribute("request_ignore").is_some() // TODO: `request(ignore)`
                        || *field.ident.as_ref().expect("Unexpected unnamed field found")
                            != "id" // Always include "id" (for now)
                })
                .collect();
            let field_names_filtered: Vec<&Ident> = fields_filtered
                .iter()
                .map(|f| f.ident.as_ref().expect("Found missing ident for field: {}"))
                .collect();
            let field_types_filtered: Vec<&syn::Type> =
                fields_filtered.iter().map(|f| &f.ty).collect();
            use tailwag_utils::strings::ToSnakeCase;
            let _route_path = ident.to_string().to_snake_case().to_string();

            let parse_args_impl_tokens = quote!(
                impl tailwag::web::traits::rest_api::BuildRoutes<#ident> for #ident
                {
                    fn build_routes()  -> tailwag::web::application::http::route::Route {
                        use tailwag_orm::data_manager::traits::DataProvider;
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
                                }
                            }
                        }
                        tailwag::web::application::http::route::Route::new_unchecked("/")
                            .get(|provider: tailwag::orm::data_manager::PostgresDataProvider<Self>| async move {
                                provider.all().await.unwrap().collect::<Vec<_>>()
                            })
                            .post(|item: Request, provider: tailwag::orm::data_manager::PostgresDataProvider<Self>| async move {
                                provider.create(item.into()).await.unwrap()
                            })
                            .delete(|item: Self, provider: tailwag::orm::data_manager::PostgresDataProvider<Self>| async move {
                                provider.delete(item).await.unwrap()
                            })
                            .patch(|item: Self, provider: tailwag::orm::data_manager::PostgresDataProvider<Self>| async move {
                                provider.update(&item).await.unwrap()
                            })
                            #(.with_route(#action_paths.to_string(), tailwag::web::application::http::route::Route::new_unchecked("/").post(#actions)))*


                            // TODO: Get by ID
                            // TODO: Custom actions / webhooks
                    }
                }
            );

            parse_args_impl_tokens
        },
        syn::Fields::Unnamed(_) => unimplemented!("Unnamed fields not supported yet"),
        syn::Fields::Unit => unimplemented!("Unit fields not supported yet"),
    }
}
