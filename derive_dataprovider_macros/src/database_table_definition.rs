/// TODO: Move the contents of this file outside, into a macro logic crate.
///
/// That was the original point of this crate, but it has evolved into being used as ORM.
use syn::{Data, DeriveInput, Field, GenericArgument, PathArguments, TypePath};

use tailwag_orm::database_definition::table_definition::{
    DatabaseColumnType, DatabaseTableDefinition, Identifier, TableColumn,
};

pub fn build_table_definition(input: &DeriveInput) -> DatabaseTableDefinition {
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
        println!("FIELDNAME{}", f.ident.as_ref().expect("Found unnamed field in struct"));
        TableColumn {
            parent_table_name: table_name.clone(),
            column_name: Identifier::new(format!(
                "{}",
                f.ident.as_ref().expect("Found unnamed field in struct")
            ))
            .expect("Invalid column name: {}"),
            column_type: get_type_from_field(&f),
            is_primary_key: true,
            is_nullable: true,
        }
    });

    DatabaseTableDefinition {
        table_name: table_name.clone(),
        columns: columns.collect(),
    }
}

fn get_type_from_field(field: &Field) -> DatabaseColumnType {
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
