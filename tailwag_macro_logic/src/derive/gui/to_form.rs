use proc_macro2::TokenStream;
use quote::quote;
use syn::{Data, DeriveInput};

fn build_function(input: &DeriveInput) -> TokenStream {
    let &DeriveInput {
        ident,
        data,
        ..
    } = &input;
    let table_name = tailwag_utils::strings::to_snake_case(&ident.to_string());
    let Data::Struct(data) = data else {
    panic!("Only Structs are supported.")
  };
    let syn::Fields::Named(fields) =  &data.fields else {
        panic!("Unnamed fields found in the struct.")
    };
    let names: Vec<String> = fields
        .named
        .iter()
        .map(|field| {
            field
                .ident
                .as_ref()
                .expect("Found named field with no value for ident")
                .to_string()
        })
        .collect();
    let tokens = quote!(
        fn render(
            &mut self,
            ui: &mut eframe::egui::Ui,
        ) -> eframe::egui::Response {
            ui.vertical(|ui| {
                #(ui.horizontal(
                    |ui| {
                        ui.label(#names);
                        // TODO: UUID this
                        ui.text_edit_singleline(&mut self.name);
                    }
                );)*
            })
            .response
        }
    );
    tokens
}
pub fn derive_struct(input: &DeriveInput) -> TokenStream {
    let &DeriveInput {
        ident,
        data,
        ..
    } = &input;
    let Data::Struct(data) = data else {
    panic!("Only Structs are supported")
  };
    match &data.fields {
        syn::Fields::Named(fields) => {
            let _field_names = fields.named.iter().map(|f| &f.ident);
            let functions: Vec<TokenStream> = vec![build_function(input)];
            let parse_args_impl_tokens = quote!(

                // TODO: Fully qualify this
                impl AsEguiForm for #ident {
                    #(#functions)*
                }

            );
            parse_args_impl_tokens.into()
        },
        syn::Fields::Unnamed(_) => unimplemented!("Unnamed fields not supported yet"),
        syn::Fields::Unit => unimplemented!("Unit fields not supported yet"),
    }
}
