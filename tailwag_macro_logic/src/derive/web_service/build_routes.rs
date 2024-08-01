use proc_macro2::TokenStream;
use quote::quote;
use syn::{punctuated::Punctuated, DeriveInput, Expr, ExprLit, ExprPath, Ident, Lit, Path, Token};
use tailwag_utils::strings::ToSnakeCase;

use crate::util::attribute_parsing::GetAttribute;

/// Helper function for extracting the route paths from an attribute.
///
/// Matches the following patterns:
///
/// #[actions("/nested/path/to/", handler_fn)] // Creates a route at `/{item_name}/nested/path/to`
/// #[actions(func_name)] // Creates a route at `/{item_name}/func_name`
fn get_route_paths(expr: Expr) -> (String, Ident, TokenStream) {
    // Extract the actual attributes
    fn get_next_ident(items: &mut syn::punctuated::Iter<Expr>) -> Option<Ident> {
        items.next().and_then(|item| match item {
            Expr::Path(ExprPath {
                path: ident_path,
                ..
            }) => ident_path.get_ident().cloned(),
            _ => None,
        })
    }
    fn get_next_string(items: &mut syn::punctuated::Iter<Expr>) -> Option<String> {
        items.next().and_then(|item| match item {
            Expr::Lit(ExprLit {
                lit: Lit::Str(extracted_literal),
                ..
            }) => Some(extracted_literal.value()),
            _ => None,
        })
    }
    let default_policy = quote!(tailwag::web::application::http::route::RoutePolicy::default());
    match expr {
        Expr::Path(path) => {
            let ident = path.path.get_ident().cloned().expect("Path is not a valid identifier");
            (ident.to_string(), ident, default_policy)
        },
        Expr::Tuple(tuple) => {
            let mut items = tuple.elems.iter();
            let path = get_next_string(&mut items).expect("No route path found.");
            let func_path =
                get_next_ident(&mut items).expect("Invalid function provided for route.");
            let route_policy: TokenStream =
                get_next_ident(&mut items).map_or(default_policy, |policy| quote!(#policy));
            (path, func_path, route_policy)
        },
        _ => todo!(),
    }
}

pub fn derive_struct(input: &DeriveInput) -> TokenStream {
    let &DeriveInput {
        ident,
        ..
    } = &input;

    fn extract_routes_from_attribute(
        input: &DeriveInput,
        attr_name: &str,
    ) -> (Vec<String>, Vec<Ident>, Vec<TokenStream>) {
        input
            .get_attribute(attr_name)
            .map(|attr| {
                attr.parse_args_with(Punctuated::<Expr, Token![,]>::parse_terminated).unwrap()
            })
            .map(|params| {
                params.into_iter().map(get_route_paths).fold(
                    (Vec::new(), Vec::new(), Vec::new()),
                    |(mut paths, mut funcs, mut policies), (path, func, route_policy)| {
                        paths.push(path);
                        funcs.push(func);
                        policies.push(route_policy);
                        (paths, funcs, policies)
                    },
                )
            })
            .unwrap_or((Vec::new(), Vec::new(), Vec::new()))
    }

    let (action_paths, actions, action_policies) = extract_routes_from_attribute(input, "actions");
    let (views_paths, views, view_policies) = extract_routes_from_attribute(input, "views");

    macro_rules! extract_policy {
        ($name:literal) => {
            input
                .get_attribute($name)
                .map(|attr| attr.parse_args::<syn::Path>().expect("Invalid list policy"))
                .map_or(
                    quote!(tailwag::web::application::http::route::RoutePolicy::default()),
                    |ident| quote!(#ident),
                )
        };
    }
    let list_policy = extract_policy!("list_policy");

    // Default CRUD routes. Can be overridden with #[get(func_name)]
    let get_list_route_tokens = input
        .get_attribute("get")
        .map(|attr| attr.parse_args::<Ident>().expect("Unable to pare get attribute"))
        .map(|func_name| quote!(.get_with_policy("/", #func_name, #list_policy)))
        .unwrap_or(quote!(
            .get_with_policy(
                "/",
                |provider: tailwag::orm::data_manager::PostgresDataProvider<Self>| async move {
                    provider.all().await.unwrap().collect::<Vec<_>>()
                },
                #list_policy
            )
        ));

    let get_policy = extract_policy!("get_policy");
    let get_detail_route_tokens = input
        .get_attribute("get_id")
        .map(|attr| attr.parse_args::<Ident>().unwrap())
        .map(|func_name| quote!(.get_with_policy("/{id}", #func_name, #get_policy)))
        .unwrap_or(quote!(
            .with_handler(
                tailwag::web::application::http::route::HttpMethod::Get,
                "/{id}",
                |tailwag::web::application::http::route::PathVariable(id): tailwag::web::application::http::route::PathString, provider: tailwag::orm::data_manager::PostgresDataProvider<Self>|{
                    use tailwag::orm::queries::filterable_types::FilterEq;
                    async move {
                        let id = uuid::Uuid::parse_str(&id).ok()?;
                        provider.get(|item|item.id.eq(id)).await.ok()
                    }
                },
                #get_policy
            )
        )
    );

    let post_policy = extract_policy!("post_policy");
    let post_create_route_tokens = input
        .get_attribute("post")
        .map(|attr| attr.parse_args::<Path>().unwrap())
        .map(|func_name| quote!(.post_with_policy("/", #func_name, #post_policy)))
        .unwrap_or(quote!(
            .post_with_policy(
                "/",
                |item: <Self as tailwag::orm::queries::Insertable>::CreateRequest, provider: tailwag::orm::data_manager::PostgresDataProvider<Self>| async move {
                    provider.create(item.into()).await.unwrap()
                },
                #post_policy
            )
        )
    );

    let patch_policy = extract_policy!("patch_policy");
    let patch_edit_route_tokens = input
        .get_attribute("patch")
        .map(|attr| attr.parse_args::<Ident>().unwrap())
            // TODO: Fix this to take /{id} instead of the whole item
        .map(|func_name| quote!(.patch_with_policy("/", #func_name, #patch_policy)))
        .unwrap_or(quote!(
            .patch_with_policy(
                "/",
                |item: Self, provider: tailwag::orm::data_manager::PostgresDataProvider<Self>| async move {
                    provider.update(&item).await.unwrap()
                },
                #patch_policy
            )
        )
    );

    let delete_policy = extract_policy!("delete_policy");
    let delete_route_tokens  = input
        .get_attribute("delete")
        .map(|attr| attr.parse_args::<Ident>().unwrap())
        .map(|func_name| quote!(.delete_with_policy("/", #func_name, #delete_policy)))
        .unwrap_or(quote!(
            // TODO: Fix this to take /{id} instead of the whole item
            .delete_with_policy(
                "/",
                |item: Self, provider: tailwag::orm::data_manager::PostgresDataProvider<Self>| async move {
                    provider.delete(item).await.unwrap()
                },
                #delete_policy
            )
        )
    );

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
                    #(.post_with_policy(#action_paths, #actions, #action_policies))*
                    #(.get_with_policy(#views_paths, #views, #view_policies))*
            }
        }
    );

    parse_args_impl_tokens
}
