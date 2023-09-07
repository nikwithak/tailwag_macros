use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use syn::{Data, DeriveInput};

const TRAIT_NAME: &'static str = "GetTableDefinition";

fn build_get_table_definition(input: &DeriveInput) -> TokenStream {
    // We build the table definition, and use that to (re-)build the creation logic.
    // Other derives use this same function, so it creates a single place to make changes
    // if the logic on how tables are built changes.
    // Additional changes to support any syntax changes must still be added here
    let input_table_definition =
        crate::util::database_table_definition::build_table_definition(input);

    // Build columns
    let table_columns = input_table_definition.columns.iter().map(|column| {
        let column_name: &str = &column.column_name;
        let column_type = match column.column_type {
            tailwag_orm::database_definition::table_definition::DatabaseColumnType::Boolean=>quote!(tailwag::orm::database_definition::table_definition::DatabaseColumnType::Boolean),
            tailwag_orm::database_definition::table_definition::DatabaseColumnType::Int=>quote!(tailwag::orm::database_definition::table_definition::DatabaseColumnType::Int),
            tailwag_orm::database_definition::table_definition::DatabaseColumnType::Float=>quote!(tailwag::orm::database_definition::table_definition::DatabaseColumnType::Float),
            tailwag_orm::database_definition::table_definition::DatabaseColumnType::String=>quote!(tailwag::orm::database_definition::table_definition::DatabaseColumnType::String),
            tailwag_orm::database_definition::table_definition::DatabaseColumnType::Timestamp=>quote!(tailwag::orm::database_definition::table_definition::DatabaseColumnType::Timestamp),
            tailwag_orm::database_definition::table_definition::DatabaseColumnType::Uuid=>quote!(tailwag::orm::database_definition::table_definition::DatabaseColumnType::Uuid),
        };
        let constraints = column.constraints.iter().map(|constraint| {
            match *constraint.detail {
                tailwag_orm::database_definition::table_definition::TableColumnConstraintDetail::NotNull => quote!(.non_null()),
                tailwag_orm::database_definition::table_definition::TableColumnConstraintDetail::PrimaryKey(_) => quote!(.pk()),
                tailwag_orm::database_definition::table_definition::TableColumnConstraintDetail::References(_) => todo!(),
                tailwag_orm::database_definition::table_definition::TableColumnConstraintDetail::Unique(_) => todo!(),
                tailwag_orm::database_definition::table_definition::TableColumnConstraintDetail::Null => quote!(),
            }
        });

        quote!(
            TableColumn::new(&#column_name, #column_type, Vec::new()).expect("Invalid column name")
            #(#constraints)*
        )
    });

    let table_name = input_table_definition.table_name.as_str();

    // !! START OF QUOTE
    let tokens = quote!(
        fn get_table_definition() -> tailwag::orm::database_definition::table_definition::DatabaseTableDefinition {
            let table_def =
                tailwag::orm::database_definition::table_definition::DatabaseTableDefinition::new(&#table_name)
                .expect("Table name is invalid")
                // #(.column(#table_columns))*
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
            /////////////////////////////////////////
            // GENERIC stuff. Part of the template //
            /////////////////////////////////////////
            let trait_name = format_ident!("{}", TRAIT_NAME);

            //////////////////////////////////////////////////////////////////////////////////////////
            //   SPECIFIC stuff - this is where you derive useful objects for your implementation   //
            //////////////////////////////////////////////////////////////////////////////////////////
            // let table = build_table_definition(&input);

            /////////////////////////////////////////
            //         Functions Exported          //
            /////////////////////////////////////////
            let functions: Vec<TokenStream> = vec![
                // todo!("Add functions here")
                build_get_table_definition(input),
            ];

            ////////////////////////////////////////
            // The actual output is defined here. //
            ////////////////////////////////////////

            // TODO: Think about how to handle Generics, when they end up being needed.
            let parse_args_impl_tokens = quote!(
                impl tailwag::orm::data_manager::GetTableDefinition for #ident {
                    #(#functions)*
                }
            );

            parse_args_impl_tokens.into()
        },
        syn::Fields::Unnamed(_) => unimplemented!("Unnamed fields not supported yet"),
        syn::Fields::Unit => unimplemented!("Unit fields not supported yet"),
    }
}
