use crate::utils::utils::{get_all_fields, get_fields_stream};
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

pub fn make_all_columns(_attr: TokenStream, item: TokenStream) -> TokenStream {
    // 解析输入的结构体
    let input = parse_macro_input!(item as DeriveInput);

    // 获取结构体的名称
    let struct_name = &input.ident;
    eprintln!("结构体名称: {:?}", struct_name); // 打印输入的结构体信息
    // 检查是否为结构体
    let fields = get_all_fields(&input);

    // eprintln!("结构字段名: {:?}", fields); // 打印输入的结构体信息
    // 提取字段名称
    let field_names = get_fields_stream(fields);
    eprintln!("字段名: {:?}", field_names);
    // 生成代码
    let expanded = quote! {
        // 原始结构体定义
        #input
        // 为结构体实现方法
        impl #struct_name {
            pub fn all_columns(&self) -> String {
                String::from("all_column")
            }
        }
    };
    eprintln!("生成的代码: {:?}", TokenStream::from(expanded.clone()).to_string()); // 打印输入的结构体信息
    // 返回生成的代码
    TokenStream::from(expanded)
}