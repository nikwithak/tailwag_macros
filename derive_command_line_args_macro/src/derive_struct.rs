use proc_macro2::TokenStream;
use quote::{format_ident, quote, quote_spanned};
use syn::{spanned::Spanned, AttrStyle, Data, DeriveInput, Field, FieldsNamed, LitStr};

fn build_create_table_query(
    DatabaseQueryBuilder {
        table_name,
        columns,
        ..
    }: &DatabaseQueryBuilder
) -> TokenStream {
    let column_toks = columns.iter().map(
        |TableColumn {
             column_name,
             column_type,
             is_primary_key,
             is_nullable,
             ..
         }| {
            let name = format_ident!("{}", column_name);
            let column_type = format_ident!("{}", column_type.as_str());
            // From #[column(primary_key, pk, pk=true)]
            let primary_key = match is_primary_key {
                true => quote!(PRIMARY KEY),
                false => quote!(),
            };
            // If Option
            let nullable = match is_nullable {
                true => quote!(),
                false => quote!(NOT NULL),
            };
            // TODO
            // let default = (if trait impls default)

            quote!(#name #column_type #primary_key #nullable)
        },
    );
    let table_name = format_ident!("{}", table_name);

    let query = quote!(
        CREATE TABLE IF NOT EXISTS #table_name {
            #(#column_toks),*
        }
    );

    query
}

enum DatabaseTableType {
    Boolean, // BOOL or BOOLEAN
    Int,     // INT
    Float,   // FLOAT
    String,  // VARCHAR or TEXT
    #[cfg(timestamp)]
    Timestamp, // TIMESTAMP
    #[cfg(uuid)]
    Uuid, // UUID
}

impl DatabaseTableType {
    fn as_str(&self) -> &str {
        match self {
            DatabaseTableType::Boolean => "BOOL",
            DatabaseTableType::Int => "INT",
            DatabaseTableType::Float => "FLOAT",
            DatabaseTableType::String => "VARCHAR",
            #[cfg(timestamp)]
            DatabaseTableType::Timestamp => "TIMESTAMP",
            #[cfg(uuid)]
            DatabaseTableType::Uuid => "UUID",
        }
    }
}

struct TableColumn {
    column_name: String,
    column_type: DatabaseTableType,
    is_primary_key: bool,
    is_nullable: bool,
    default: Option<String>,
}

impl From<&Field> for DatabaseTableType {
    fn from(field: &Field) -> Self {
        println!("{:?}", &field.ty);
        Self::String
        // match &field.ty {
        //     syn::Type::Array(_) => todo!(),
        //     syn::Type::BareFn(_) => todo!(),
        //     syn::Type::Group(_) => todo!(),
        //     syn::Type::ImplTrait(_) => todo!(),
        //     syn::Type::Infer(_) => todo!(),
        //     syn::Type::Macro(_) => todo!(),
        //     syn::Type::Never(_) => todo!(),
        //     syn::Type::Paren(_) => todo!(),
        //     syn::Type::Path(path) => {
        //         println!("{:?}", &path);
        //         Self::String
        //     },
        //     syn::Type::Ptr(_) => todo!(),
        //     syn::Type::Reference(_) => todo!(),
        //     syn::Type::Slice(_) => todo!(),
        //     syn::Type::TraitObject(_) => todo!(),
        //     syn::Type::Tuple(_) => todo!(),
        //     syn::Type::Verbatim(_) => todo!(),
        //     _ => {
        //         println!("{:?}", &path);
        //         Self::String
        //     },
        // }
    }
}

struct DatabaseQueryBuilder {
    table_name: String,
    columns: Vec<TableColumn>,
}

impl From<&FieldsNamed> for DatabaseQueryBuilder {
    fn from(fields: &FieldsNamed) -> Self {
        let columns = fields.named.iter().map(|f| TableColumn {
            column_name: format!("{}", f.ident.as_ref().expect("Found unnamed field in struct")),
            column_type: DatabaseTableType::from(f),
            is_primary_key: true,
            is_nullable: true,
            default: None,
        });

        DatabaseQueryBuilder {
            table_name: "TestTable".into(),
            columns: columns.collect(),
        }
    }
}

pub(crate) fn derive_struct(
    DeriveInput {
        ident,
        data,
        ..
    }: &DeriveInput
) -> TokenStream {
    // Panic with error message if we get a non-struct
    let Data::Struct(data) = data else { panic!("Only Structs are supported") };

    // Process all of the fields into tokens
    match &data.fields {
        syn::Fields::Named(fields) => {
            let field_names = fields.named.iter().map(|f| &f.ident);

            // TODO: Use the types for special parsing (e.g. bool, vec, etc.)
            let field_types = fields.named.iter().map(|f| &f.ty);

            // Build the match statement - first we get each individual match branch for each field
            // let match_args = fields
            //     .named
            //     .iter()
            //     .map(|f| {
            //         // Only supports basic strings right now.
            //         // TODO: Boolean support
            //         // TODO: Subcommand support
            //         // TODO: Parameter support
            //         // TODO: Vec support
            //         let &Field {
            //             ident,
            //             attrs,
            //             ..
            //         } = &f;

            //         let mut option_names = Vec::new();

            //         // Parse #[opts(param1=value1, param2=value2)]
            //         let attrs = attrs.iter().filter(|a|a.style == AttrStyle::Outer).filter(|a|a.path().is_ident("opts"));
            //         for attr in attrs {
            //             attr.parse_nested_meta(|meta| {
            //                 // Handles #[opts(long = "--long")]
            //                 if meta.path.is_ident("long") || meta.path.is_ident("short") {
            //                     let long_name = meta.value()?.parse::<LitStr>()?.value();
            //                     option_names.push(long_name);
            //                 }
            //                 Ok(())
            //             }).ok();
            //         }

            //         let ident = ident.as_ref().expect("Missing identifier for struct field").to_string();
            //         if option_names.is_empty() { option_names.push(format!("--{}", &ident)); }

            //         let span = f.span();
            //         quote_spanned!(span=>
            //             #(#option_names) |* => { map.insert(#ident.into(), args.next().expect("Expected argument, found none").to_owned()); })
            //     })
            //     .collect::<Vec<_>>();

            // let match_statement = quote!(match &arg as &str {
            //     #(#match_args,)*
            //     _ => (),
            //     // _ => { panic!("Unsupported input" )}
            // });

            // Do I really have to create a new iterator for this?? Seems that way (when using `quote!()`)
            // TODO: See if there's an easier way to do this.
            let field_names_quoted =
                field_names.clone().map(|f| format!("{}", &f.as_ref().unwrap()));

            let trait_name = format_ident!("{}", "PostgresDataProvider");
            let table = DatabaseQueryBuilder::from(fields);
            let query = build_create_table_query(&table).to_string();

            // Build the actual implementation
            let parse_args_impl_tokens = quote!(
                impl #trait_name for #ident {
                    // fn get(id: &str) -> Self {
                    // fn get<T: ?Sized>(receiver: &mut Vec<Box<T>>) -> Self
                    //     where T: Into<Uuid> {
                    //     let mut map = std::collections::HashMap::<String, String>::new();
                    //     let mut args = args.into_iter();

                    //     // Parse all the args.
                    //     // TODO: Deal with sub-commands / parameters
                    //     // Right now we only are pulling in options
                    //     while let Some(arg) = args.next() {
                    //         #match_statement
                    //     }

                    //     // Build and return the final struct
                    //     #ident {
                    //         #(#field_names: map.remove(#field_names_quoted).unwrap_or_default(),)*
                    //     }
                    // }

                    fn build_create_table_query() {
                        // #println( #query);
                        println!("{}", #query);
                    }
                }
            );

            parse_args_impl_tokens.into()
        },
        syn::Fields::Unnamed(_) => unimplemented!("Unnamed fields not supported yet"),
        syn::Fields::Unit => unimplemented!("Unit fields not supported yet"),
    }
}
