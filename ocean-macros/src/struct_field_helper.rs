use crate::utils::utils;
use proc_macro::TokenStream;
use quote::{quote, ToTokens};
use syn::punctuated::Punctuated;
use syn::{parse_macro_input, DeriveInput, Field, Token};

///
/// 实现属性宏
///
///
pub(crate) fn internal_get_all_fields_str(item: TokenStream) -> TokenStream {
    // 解析输入的结构体
    let input = parse_macro_input!(item as DeriveInput);
    // 获取结构体的名称
    let struct_name = &input.ident;
    // eprintln!("结构体名称: {:?}", struct_name); // 打印输入的结构体信息
    // 检查是否为结构体
    let fields: &Punctuated<Field, Token![,]> = utils::get_all_fields(&input);

    let mut field_list: Vec<String> = Vec::new();
    for field in fields {
        field_list.push(field.ident.as_ref().unwrap().to_string());
    }
    let all_columns = field_list.join(", ");
    // 生成代码
    let expanded = quote! {
        // 为结构体实现方法
        impl #struct_name {
            pub fn get_all_fields_str(&self) -> String {
                String::from(#all_columns)
            }
        }
    };
    eprintln!("list_fix_fields生成的代码: {:?}", TokenStream::from(expanded.clone()).to_string()); // 打印输入的结构体信息
    // 返回生成的代码
    TokenStream::from(expanded)
}

///
/// 按照结构体中排列的顺序，将结构体的数据转换为对应的Vec<JsonValue>
///
pub(crate) fn internal_to_json_value(item: TokenStream) -> TokenStream {
    // 解析输入的结构体
    let input: DeriveInput = parse_macro_input!(item as DeriveInput);
    // 获取结构体的名称
    let struct_name = &input.ident;
    // eprintln!("结构体名称: {:?}", struct_name); // 打印输入的结构体信息
    // 检查是否为结构体
    let fields: &Punctuated<Field, Token![,]> = utils::get_all_fields(&input);

    // 提取字段名称
    let field_names = utils::get_fields_stream(fields);

    // 生成代码
    let expanded = quote! {
        // 原始结构体定义
        // 为结构体实现方法
        impl #struct_name {
            pub fn to_json_value(&self) -> Vec<serde_json::Value> {
                vec![
                    #(serde_json::Value::from(#field_names)),*
                ]
            }
        }
    };
    eprintln!("生成的代码: {:?}", TokenStream::from(expanded.clone()).to_string()); // 打印输入的结构体信息
    // 返回生成的代码
    TokenStream::from(expanded)
}
