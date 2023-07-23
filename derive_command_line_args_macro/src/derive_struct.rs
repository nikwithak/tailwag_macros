use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use syn::{Data, DeriveInput, Field, GenericArgument, PathArguments, TypePath};

fn build_create_table_query(
    DatabaseTableDefinition {
        table_name,
        columns,
        ..
    }: &DatabaseTableDefinition
) -> String {
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

    let create_query = quote!(
        CREATE TABLE IF NOT EXISTS #table_name {
            #(#column_toks),*
        }
    )
    .to_string();
    create_query.replace(", ", ",\n")
}

enum DatabaseColumnType {
    Boolean, // BOOL or BOOLEAN
    Int,     // INT
    Float,   // FLOAT
    String,  // VARCHAR or TEXT
    #[cfg(timestamp)]
    Timestamp, // TIMESTAMP
    #[cfg(uuid)]
    Uuid, // UUID
}

impl DatabaseColumnType {
    fn as_str(&self) -> &str {
        match self {
            DatabaseColumnType::Boolean => "BOOL",
            DatabaseColumnType::Int => "INT",
            DatabaseColumnType::Float => "FLOAT",
            DatabaseColumnType::String => "VARCHAR",
            #[cfg(timestamp)]
            DatabaseColumnType::Timestamp => "TIMESTAMP",
            #[cfg(uuid)]
            DatabaseColumnType::Uuid => "UUID",
        }
    }
}

struct TableColumn {
    column_name: String,
    column_type: DatabaseColumnType,
    is_primary_key: bool,
    is_nullable: bool,
    _default: Option<String>,
}

impl From<&Field> for DatabaseColumnType {
    fn from(field: &Field) -> Self {
        match &field.ty {
            syn::Type::Path(typepath) => {
                fn get_qualified_path(typepath: &TypePath) -> String {
                    let qualified_path =
                        typepath.path.segments.iter().fold(String::new(), |mut acc, p| {
                            acc.push_str(&p.ident.to_string());
                            acc.push_str("::");
                            acc
                        });
                    qualified_path.trim_end_matches("::").to_string()
                }

                // Match the type - if it's a supported type, we map it to the DatabaseColumnType. If it's not, we either fail (MVP), or we add support for joins via another trait (must impl DatabaseColumnSubType or something).
                let mut qualified_path = get_qualified_path(typepath);
                qualified_path = match qualified_path.as_str() {
                    "std::option::Option"
                    | "core::option::Option"
                    | "option::Option"
                    | "Option" => {
                        let type_params = &typepath
                            .path
                            .segments
                            .last()
                            .expect("Option should have an inner type")
                            .arguments;
                        match &type_params {
                            PathArguments::AngleBracketed(params) => {
                                let arg =
                                    params.args.first().expect("No type T found for Option<T>");
                                match arg {
                                    GenericArgument::Type(syn::Type::Path(t)) => {
                                        Some(get_qualified_path(t))
                                    },
                                    _ => panic!("no type T found for Option<T>"),
                                }
                            },
                            _ => panic!("No type T found for Option<T>"),
                        }
                    },
                    _ => None,
                }
                .unwrap_or(qualified_path);

                let db_type = match qualified_path.as_str() {
                    "std::string::String" | "string::String" | "String" => {
                        DatabaseColumnType::String
                    },
                    "bool" => DatabaseColumnType::Boolean,
                    "u32" | "u64" | "i32" | "i64" | "usize" | "isize" => DatabaseColumnType::Int,
                    "f32" | "f64" | "fsize" => DatabaseColumnType::Float,
                    #[cfg(timestamp)]
                    "chrono::_" => DatabaseColumnType::Timestamp,
                    #[cfg(uuid)]
                    "uuid::Uuid" | "Uuid" => DatabaseColumnType::Uuid,
                    _ => {
                        unimplemented!("{} not a supported type.", qualified_path)
                    },
                };
                db_type
            },
            _ => {
                unimplemented!("Not a supported data type")
            },
        }
    }
}

// The details of the Database table. Used to generate the queries for setting up and iteracting with the database.
struct DatabaseTableDefinition {
    table_name: String,
    columns: Vec<TableColumn>,
}

impl From<&DeriveInput> for DatabaseTableDefinition {
    fn from(input: &DeriveInput) -> Self {
        let &DeriveInput {
            ident,
            data,
            ..
        } = &input;

        // Panic with error message if we get a non-struct
        let Data::Struct(data) = data else { panic!("Only Structs are supported.") };
        let syn::Fields::Named(fields) = &data.fields else { panic!("Unnamed fields found in the struct.")};

        let columns = fields.named.iter().map(|f| TableColumn {
            column_name: format!("{}", f.ident.as_ref().expect("Found unnamed field in struct")),
            column_type: DatabaseColumnType::from(f),
            is_primary_key: true,
            is_nullable: true,
            _default: None,
        });

        DatabaseTableDefinition {
            table_name: ident.to_string(),
            columns: columns.collect(),
        }
    }
}

pub(crate) fn derive_struct(input: &DeriveInput) -> TokenStream {
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

            // TODO: Use the types for special parsing (e.g. bool, vec, etc.)
            let _field_types = fields.named.iter().map(|f| &f.ty);

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
            let trait_name = format_ident!("{}", "PostgresDataProvider");
            let table = DatabaseTableDefinition::from(input);

            let query_create = build_create_table_query(&table).to_string();

            // Build the actual implementation
            let parse_args_impl_tokens = quote!(
                impl #trait_name for #ident {
                    fn build_create_table_query() {
                        println!("{}", #query_create);
                    }
                }
            );

            parse_args_impl_tokens.into()
        },
        syn::Fields::Unnamed(_) => unimplemented!("Unnamed fields not supported yet"),
        syn::Fields::Unit => unimplemented!("Unit fields not supported yet"),
    }
}
