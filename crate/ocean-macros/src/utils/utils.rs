use quote::quote;
use syn::punctuated::Punctuated;
use syn::{Data, DeriveInput, Field, Fields, Token};

///
/// 获取所有结构体的字段对象Field
///
pub(crate) fn get_all_fields(derive_input: &DeriveInput) -> &Punctuated<Field, Token![,]> {
    match &derive_input.data {
        Data::Struct(data_struct) => match &data_struct.fields {
            Fields::Named(fields_named) => &fields_named.named,
            _ => panic!("Only structs with named fields are supported"),
        },
        _ => panic!("Only structs are supported"),
    }
}

///
/// 获取所有结构体的字段流结构
///
pub(crate) fn get_fields_stream(fields: &Punctuated<Field, Token![,]>) -> Vec<proc_macro2::TokenStream> {
    // 提取字段名称
    let field_names: Vec<_> = fields
        .iter()
        .map(|field| {
            let field_name = field.ident.as_ref().unwrap();
            quote! { self.#field_name.clone() }
        })
        .collect();
    field_names
}