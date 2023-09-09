/// Used for quick templating. Intended for use with auto-expanding tools, to generate boilerplate for a new Derive macro
#[macro_export]
macro_rules! quick_derive_struct {
    ($i:item) => {
        use proc_macro2::TokenStream;
        use quote::{format_ident, quote};
        use syn::{Data, DeriveInput};

        fn build_function_definition(input: &DeriveInput) -> TokenStream {
    let &DeriveInput {
        ident,
        data,
        ..
    } = &input;
            let table_name = tailwag_utils::strings::to_camel_case(&ident.to_string());

            // Panic with error message if we get a non-struct
            let Data::Struct(data) = data else { panic!("Only Structs are supported.") };
            let syn::Fields::Named(fields) = &data.fields else { panic!("Unnamed fields found in the struct.")};


            // !! START OF QUOTE
            let tokens = quote!(
                fn get_table_definition() -> tailwag::orm::database_definition::table_definition::DatabaseTableDefinition {
                    let table_def =
                        tailwag::orm::database_definition::table_definition::DatabaseTableDefinition::new(&#table_name)
                        .expect("Table name is invalid")
                        #(.column(#table_columns))*
                    //     // #(.constraint(#table_constraints)*) // TODO - weak constriants support currently
                        ;

                    table_def.into()
                }
            );
            // !! END OF QUOTE

            tokens
        }

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
                    let _field_names = fields.named.iter().map(|f| &f.ident);

                    let functions: Vec<TokenStream> = vec![
                        // todo!("Add functions here")
                        build_get_table_definition(input),
                    ];

                    // TODO: Think about how to handle Generics, when they end up being needed.
                    let parse_args_impl_tokens = quote!(
                        impl tailwag::web_service::traits::BuildRoutes for #ident {
                            #(#functions)*
                        }
                    );

                    parse_args_impl_tokens.into()
                },
                syn::Fields::Unnamed(_) => unimplemented!("Unnamed fields not supported yet"),
                syn::Fields::Unit => unimplemented!("Unit fields not supported yet"),
            }
        }
    };
}
