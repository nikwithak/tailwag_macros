use syn::{Data, DeriveInput, Field, GenericArgument, PathArguments, TypePath};

#[derive(Clone, PartialEq, Eq)]
pub enum DatabaseColumnType {
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
    pub fn as_str(&self) -> &str {
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

#[derive(Clone)]
pub struct TableColumn {
    pub column_name: String,
    pub column_type: DatabaseColumnType,
    // TODO: If this grows too big, add a new type for "ColumnModifiers" or similar
    pub is_primary_key: bool,
    pub is_nullable: bool,
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
pub struct DatabaseTableDefinition {
    pub table_name: String,
    pub columns: Vec<TableColumn>,
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

        let columns = fields.named.iter().map(|f| {
            println!("{:?}", &f);
            println!("FIELDNAME{}", f.ident.as_ref().expect("Found unnamed field in struct"));
            TableColumn {
                column_name: format!(
                    "{}",
                    f.ident.as_ref().expect("Found unnamed field in struct")
                ),
                column_type: DatabaseColumnType::from(f),
                is_primary_key: true,
                is_nullable: true,
                _default: None,
            }
        });

        DatabaseTableDefinition {
            table_name: ident.to_string(),
            columns: columns.collect(),
        }
    }
}
