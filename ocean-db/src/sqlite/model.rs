use serde::{Deserialize, Serialize};

///
///
///
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PageRequest {
    pub page_no: u64,                              // 当前页码，从1开始
    pub page_size: u64,                            // 每页大小
    pub sort_by: Option<String>,                   // 排序字段
    pub order: Option<String>,                     // 排序顺序：'asc' 或 'desc'
    pub conditions: Option<Vec<(String, String)>>, // 查询条件列表 (列名, 值)
}

impl PageRequest {
    pub fn new(page_no: u64, page_size: u64) -> Self {
        PageRequest {
            page_no,
            page_size,
            sort_by: None,
            order: None,
            conditions: None,
        }
    }
}

///
///
///
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PageResult<T> {
    pub items: Vec<T>, // 当前页的数据项
    pub total: u64,    // 总数据条数
    pub page_no: u64,  // 当前页码
    pub page_size: u64,
    pub total_pages: u64, // 总页数
}

impl<T> PageResult<T> {
    pub fn new(items: Vec<T>, total: u64, page_no: u64, page_size: u64, total_pages: u64) -> Self {
        PageResult {
            items,
            total,
            page_no,
            page_size,
            total_pages,
        }
    }
    pub fn empty(page_no: u64, page_size: u64) -> Self {
        Self::new(Vec::new(), 0, page_no, page_size, 0)
    }
}