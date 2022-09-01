use proc_macro::TokenStream;
use quote::quote;
use syn::{
    parse::{self, Parser},
    parse_macro_input, ItemStruct,
};

// #[proc_macro_attribute]
// pub fn show_streams(attr: TokenStream, item: TokenStream) -> TokenStream {
//     println!("attr: \"{}\"", attr.to_string());
//     println!("item: \"{}\"", item.to_string());
//     item
// }

#[proc_macro_attribute]
pub fn add_field(args: TokenStream, input: TokenStream) -> TokenStream {
    let mut item_struct = parse_macro_input!(input as ItemStruct);
    let _ = parse_macro_input!(args as parse::Nothing);

    if let syn::Fields::Named(ref mut fields) = item_struct.fields {
        fields.named.push(
            syn::Field::parse_named
                .parse2(quote! {
                    pub created_date: String,
                    pub updated_date: String
                })
                .unwrap(),
        );
    }

    return quote! {
        #item_struct
    }
    .into();
}
