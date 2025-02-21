use ocean_macros::{GetFieldList, ToFieldJsonValue};
use serde::{Deserialize, Serialize};

// #[to_ordered_vec]
#[derive(Default, Deserialize, Serialize, Clone, Debug, GetFieldList, ToFieldJsonValue)]
pub struct Demo {
    pub id: Option<String>,
    #[db_field_name(rename = "title_field")]
    pub title: Option<String>,
    #[db_field_name("remark_field")]
    pub remark: Option<String>,
}


#[cfg(test)]
#[test]
pub fn test_ordered_fields() {
    let demo = Demo {
        id: Some("1324123".to_string()),
        title: Some("title".to_string()),
        remark: Some("remark".to_string()),
    };
    // let json = vec![Value::from(demo.id), Value::from(demo.remark), Value::from(demo.title)];
    let json = demo.to_json_value();
    println!("{:?}", json)
}

#[test]
pub fn test_ordered_vec() {
    let vec = vec!["Hello".to_string(), "world".to_string(), "Rust".to_string()];
    let joined: String = vec.iter().map(|s| s.as_str()).collect::<Vec<&str>>().join(", ");
    println!("{}", joined); // 输出: Hello, world, Rust
}
