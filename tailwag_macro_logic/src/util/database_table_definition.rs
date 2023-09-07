use proc_macro2::TokenStream;
/// TODO: Move the contents of this file outside, into a macro logic crate.
///
/// That was the original point of this crate, but it has evolved into being used as ORM.
use syn::{Data, DeriveInput, Field, GenericArgument, PathArguments, TypePath};

use tailwag_orm::database_definition::table_definition::{
    DatabaseColumnType, DatabaseTableDefinition, Identifier, TableColumn,
};

/// Builds each function implementation for a given struct. You should have a separate function for each function required as part of the #[derive(_)] impl.
pub fn functions(input: &DeriveInput) -> Vec<TokenStream> {
    vec![]
}

fn build_fn_get_table_definition(input: &DeriveInput) -> TokenStream {
    let &DeriveInput {
        ident,
        data,
        ..
    } = &input;
    let table_name = Identifier::new(ident.to_string()).expect("Invalid identifier");

    // Panic with error message if we get a non-struct
    let Data::Struct(data) = data else { panic!("Only Structs are supported.") };
    let syn::Fields::Named(fields) = &data.fields else { panic!("Unnamed fields found in the struct.")};

    let columns = fields.named.iter().map(|f| {
        let field_name = f.ident.as_ref().expect("Found unnamed field in struct");

        let mut column =
            TableColumn::new(&field_name.to_string(), get_type_from_field(&f), Vec::new())
                .expect("Invalid table_name");
        // TODO: Wrap in logic for "if is pk" - for now go off of "if field is named `id`, later on using annotations"
        if &field_name.to_string() == "id" {
            column = column.pk();
        }
        if !is_option(&f) {
            column = column.non_null();
        }
        column
    });

    let tokens = quote::quote!(fn);

    let mut table = DatabaseTableDefinition::new(&table_name).expect("Table name is invalid");
    for column in columns {
        table.add_column(column);
    }
    // table.into();
    todo!()
}

fn get_qualified_path(typepath: &TypePath) -> String {
    let qualified_path = typepath.path.segments.iter().fold(String::new(), |mut acc, p| {
        acc.push_str(&p.ident.to_string());
        acc.push_str("::");
        acc
    });
    qualified_path.trim_end_matches("::").to_string()
}

fn is_option(field: &Field) -> bool {
    if let syn::Type::Path(typepath) = &field.ty {
        match get_qualified_path(typepath).as_str() {
            "std::option::Option" | "core::option::Option" | "option::Option" | "Option" => true,
            _ => false,
        }
    } else {
        false
    }
}

fn get_type_from_field(field: &Field) -> DatabaseColumnType {
    match &field.ty {
        syn::Type::Path(typepath) => {
            // Match the type - if it's a supported type, we map it to the DatabaseColumnType. If it's not, we either fail (MVP), or we add support for joins via another trait (must impl DatabaseColumnSubType or something).
            // TODO: DRY this out using the `is_option` fn above
            let mut qualified_path = get_qualified_path(typepath);
            qualified_path = match qualified_path.as_str() {
                "std::option::Option" | "core::option::Option" | "option::Option" | "Option" => {
                    let type_params = &typepath
                        .path
                        .segments
                        .last()
                        .expect("Option should have an inner type")
                        .arguments;
                    match &type_params {
                        PathArguments::AngleBracketed(params) => {
                            let arg = params.args.first().expect("No type T found for Option<T>");
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
                "std::string::String" | "string::String" | "String" => DatabaseColumnType::String,
                "bool" => DatabaseColumnType::Boolean,
                "u32" | "u64" | "i32" | "i64" | "usize" | "isize" => DatabaseColumnType::Int,
                "f32" | "f64" | "fsize" => DatabaseColumnType::Float,
                "chrono::_" => DatabaseColumnType::Timestamp, // TODO
                "uuid::Uuid" | "Uuid" => DatabaseColumnType::Uuid,
                _ => {
                    // TODO: Impl for joinable tables
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
