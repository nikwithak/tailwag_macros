use proc_macro2::TokenStream;
use quote::quote;
use syn::{
    punctuated::Punctuated, Data, DeriveInput, Expr, ExprLit, ExprPath, Ident, Lit, Path, Token,
};

use crate::util::attribute_parsing::GetAttribute;

/// Helper function for extracting the route paths from an attribute.
///
/// Matches the following patterns:
///
/// #[actions("/nested/path/to/", handler_fn)] // Creates a route at `/{item_name}/nested/path/to`
/// #[actions(func_name)] // Creates a route at `/{item_name}/func_name`
fn get_route_paths(expr: Expr) -> (String, Ident) {
    // Extract the actual attributes
    match expr {
        Expr::Path(path) => {
            let ident = path.path.get_ident().cloned().expect("Path is not a valid identifier");
            (ident.to_string(), ident)
        },
        Expr::Tuple(tuple) => {
            let mut items = tuple.elems.iter();
            let Some(Expr::Lit(ExprLit {
                lit: Lit::Str(path_lit),
                ..
            })) = items.next()
            else {
                todo!("Unable to parse route paths.");
            };
            let path = path_lit.value();
            let Some(Expr::Path(ExprPath {
                path: func_path,
                ..
            })) = items.next()
            else {
                todo!("No function name provided");
            };
            (path, func_path.get_ident().cloned().expect("Didn't get valid ident"))
        },
        _ => todo!(),
    }
}

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

    fn extract_routes_from_attribute(
        input: &DeriveInput,
        attr_name: &str,
    ) -> (Vec<String>, Vec<Ident>) {
        input
            .get_attribute(attr_name)
            .map(|attr| {
                attr.parse_args_with(Punctuated::<Expr, Token![,]>::parse_terminated).unwrap()
            })
            .map(|params| {
                params.into_iter().map(get_route_paths).fold(
                    (Vec::new(), Vec::new()),
                    |(mut paths, mut funcs), (path, func)| {
                        paths.push(path);
                        funcs.push(func);
                        (paths, funcs)
                    },
                )
            })
            .unwrap_or((Vec::new(), Vec::new()))
    }

    let (action_paths, actions): (Vec<String>, Vec<Ident>) =
        extract_routes_from_attribute(input, "actions");
    let (views_paths, views): (Vec<String>, Vec<Ident>) =
        extract_routes_from_attribute(input, "views");

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
                            //  }
                            }
                        }
            )
        )
    );

    let post_create_route_tokens = input
        .get_attribute("post")
        .map(|attr| attr.parse_args::<Path>().unwrap())
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
                            #(.get(#views_paths, #views))*
                    }
                }
            );

            parse_args_impl_tokens
        },
        syn::Fields::Unnamed(_) => unimplemented!("Unnamed fields not supported yet"),
        syn::Fields::Unit => unimplemented!("Unit fields not supported yet"),
    }
}
