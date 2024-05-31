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

    let views = input
        .get_attribute("views")
        .map(|attr| attr.parse_args_with(Punctuated::<Ident, Token![,]>::parse_terminated).unwrap())
        .map(|params| params.into_iter().collect::<Vec<_>>())
        .unwrap_or(Vec::new());
    // Is this really how the only way to inject tokens into quote! with surrounding quotes?
    let view_paths = views.iter().map(|s| s.to_string());

    // Default CRUD routes. Can be overridden with #[get(func_name), get("/", func_name), get_list("func_name")]
    let get_list_route_tokens = input
        .get_attribute("get")
        .map(|attr| attr.parse_args::<Ident>().unwrap())
        .map(|func_name| quote!(.get("/", #func_name)))
        // .map(|params| params.into_iter().collect::<Vec<_>>())
        // .and_then(|params|)
        .unwrap_or(quote!(
            .get("/",|provider: tailwag::orm::data_manager::PostgresDataProvider<Self>| async move {
                provider.all().await.unwrap().collect::<Vec<_>>()
            })
        ));

    let get_detail_route_tokens = input
        .get_attribute("get_id")
        .map(|attr| attr.parse_args::<Ident>().unwrap())
        .map(|func_name| quote!(.get("/{id}", #func_name)))
        // .map(|params| params.into_iter().collect::<Vec<_>>())
        // .and_then(|params|)
        .unwrap_or(quote!(
            .with_handler(tailwag::web::application::http::route::HttpMethod::Get, "/{id}",
                        |tailwag::web::application::http::route::PathVariable(id): tailwag::web::application::http::route::PathString, provider: tailwag::orm::data_manager::PostgresDataProvider<Self>|{
                        use tailwag::orm::queries::filterable_types::FilterEq;
                        async move {
                            // TODO: This whole macro just uses "Unwrap" for everything.
                            // I need to give it real error handling.
                            let id = uuid::Uuid::parse_str(&id).ok()?;
                            provider.get(|item|item.id.eq(id)).await.ok()
                            // match provider.get(|item|item.id.eq(id)).await {
                            //     Ok(Some(item)) => IntoResponse::into(item),
                            //     Ok(None) => Response::not_found(),
                            //     Err(_) => tailwag::web::application::http::route::Response::bad_request(),
                            // }
                            }
                        }
            )
        )
    );

    let post_create_route_tokens = input
        .get_attribute("post")
        .map(|attr| attr.parse_args::<Ident>().unwrap())
        .map(|func_name| quote!(.post("/", #func_name)))
        // .map(|params| params.into_iter().collect::<Vec<_>>())
        // .and_then(|params|)
        .unwrap_or(quote!(
            .post("/",|item: <Self as tailwag::orm::queries::Insertable>::CreateRequest, provider: tailwag::orm::data_manager::PostgresDataProvider<Self>| async move {
                provider.create(item.into()).await.unwrap()
            })
        )
    );
    let patch_edit_route_tokens = input
        .get_attribute("patch")
        .map(|attr| attr.parse_args::<Ident>().unwrap())
        .map(|func_name| quote!(.patch("/", #func_name)))
        // .map(|params| params.into_iter().collect::<Vec<_>>())
        // .and_then(|params|)
        .unwrap_or(quote!(
            .patch("/",|item: Self, provider: tailwag::orm::data_manager::PostgresDataProvider<Self>| async move {
                provider.update(&item).await.unwrap()
            })
        )
    );
    let delete_route_tokens  = input
        .get_attribute("delete")
        .map(|attr| attr.parse_args::<Ident>().unwrap())
        .map(|func_name| quote!(.delete("/", #func_name)))
        // .map(|params| params.into_iter().collect::<Vec<_>>())
        // .and_then(|params|)
        .unwrap_or(quote!(
            // TODO: Fix this to take /{id} instead of the whole item
            .delete("/",|item: Self, provider: tailwag::orm::data_manager::PostgresDataProvider<Self>| async move {
                provider.delete(item).await.unwrap()
            })
        )
    );

    // list.parse_args_with(Punctuated::<LitStr, Token![,]>::parse_terminated)
    // let args_parsed = syn::punctuated::Punctuated::<syn::Path, syn::Token![,]>::parse_terminated
    // .
    // .unwrap();

    match &data.fields {
        syn::Fields::Named(fields) => {
            use tailwag_utils::strings::ToSnakeCase;
            let _route_path = ident.to_string().to_snake_case().to_string();
            let parse_args_impl_tokens = quote!(
                impl tailwag::web::traits::rest_api::BuildRoutes<#ident> for #ident
                {
                    fn build_routes()  -> tailwag::web::application::http::route::Route {
                        use tailwag::orm::data_manager::traits::DataProvider;
                        tailwag::web::application::http::route::Route::new()
                            #get_list_route_tokens
                            #get_detail_route_tokens
                            #post_create_route_tokens
                            #patch_edit_route_tokens
                            #delete_route_tokens
                            #(.post(#action_paths, #actions))*
                            #(.get(#view_paths, #views))*
                    }
                }
            );

            parse_args_impl_tokens
        },
        syn::Fields::Unnamed(_) => unimplemented!("Unnamed fields not supported yet"),
        syn::Fields::Unit => unimplemented!("Unit fields not supported yet"),
    }
}
