mod struct_field_helper;
mod sql_columns;
mod utils;

use crate::sql_columns::make_all_columns;
use crate::struct_field_helper::{internal_get_all_fields_str, internal_to_json_value};
use proc_macro::TokenStream;

///
/// 实现字段拼接为字符串,便于后续组装SQL语句
/// pub fn get_all_fields_str ->String{
///     String::from("id, title, remark")
/// }
/// let data = Demo::default().get_all_fields_str();
///
///
///
#[proc_macro_derive(GetFieldList, attributes(db_field_name))]
pub fn get_all_fields_str_derive(item: TokenStream) -> TokenStream {
    internal_get_all_fields_str(item)
}


///
/// 将结构体按照结构写的顺序，添加json序列化方法
/// pub fn  to_json_value() -> Vec<Value> {
///  vec![serde_json::Value::from(self.id.clone()),serde_json::Value::from(self.title.clone()),serde_json::Value::from(self.remark.clone())]
/// }
///
/// let demo = Demo {
///         id: Some("1324123".to_string()),
///         title: Some("title".to_string()),
///         remark: Some("remark".to_string()),
///  } ;
///  let json = demo.to_json_value();
///
///
#[proc_macro_derive(ToFieldJsonValue, attributes(db_field_name))]
pub fn to_json_value_derive(item: TokenStream) -> TokenStream {
    internal_to_json_value(item)
}

#[proc_macro_attribute]
pub fn all_columns(_attr: TokenStream, item: TokenStream) -> TokenStream {
    make_all_columns(_attr, item)
}