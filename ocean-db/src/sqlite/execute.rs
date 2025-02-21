use crate::sqlite::model::{PageRequest, PageResult};
use serde_json::Number;
use sqlx::query::{Query, QueryAs};
use sqlx::sqlite::{SqliteArguments, SqliteRow, SqliteStatement};
use sqlx::types::JsonValue;
use sqlx::{Database, Executor, FromRow, Row, Sqlite, SqlitePool, Statement};

///
/// 支持分页查询
/// 分页代码如下:
/// let result: PageResult<Task> = paginate(
//             &PageRequest {
//                 page_no: 1,
//                 page_size: 10,
//                 sort_by: None,
//                 order: None,
//                 conditions: None,
//             },
//             "select * from tb_task",
//         )
//             .await
//             .unwrap();
//         println!("{:?}", result);
///
///
pub async fn paginate<'q, T>(
    pool: &SqlitePool,
    base_query: &'q str,
    page_request: &PageRequest,
) -> Result<PageResult<T>, String>
where
    T: for<'a> FromRow<'a, SqliteRow> + Send + Unpin,
{
    let mut query = format!("SELECT COUNT(1) as cnt FROM ({})", base_query);
    let mut condition_clauses = Vec::new();
    let mut params: Vec<JsonValue> = Vec::new();

    if page_request.page_no < 1 {
        return Ok(PageResult::empty(
            page_request.page_no,
            page_request.page_size,
        ));
    }

    if let Some(conditions) = &page_request.conditions {
        for (index, (column, value)) in conditions.iter().enumerate() {
            condition_clauses.push(format!("{} = ?{}", column, index + 1));
            params.push(JsonValue::String(value.into()));
        }
    }

    if !condition_clauses.is_empty() {
        query.push_str(" WHERE ");
        query.push_str(&condition_clauses.join(" AND "));
    }

    let stmt = pool.prepare(&query).await;
    if let Err(e) = stmt {
        return Err(e.to_string());
    }
    let statement: SqliteStatement = stmt.unwrap();
    let query = statement.query();
    let search_cond = params.clone();
    let total_bind_query = bind_cond_json_list(query, search_cond);

    let count_row = pool.fetch_one(total_bind_query).await;
    if let Err(e) = count_row {
        return Err(e.to_string());
    }
    let row = count_row.unwrap();
    let total: u64 = row.get(0);
    //
    if total == 0 {
        return Ok(PageResult::empty(
            page_request.page_no,
            page_request.page_size,
        ));
    }

    let total_pages: u64 = (total + page_request.page_size - 1) / page_request.page_size;
    if page_request.page_no > total_pages {
        return Ok(PageResult::empty(
            page_request.page_no,
            page_request.page_size,
        ));
    }
    //
    let offset = (page_request.page_no - 1) * page_request.page_size;
    let mut paginated_query = base_query.to_string();

    // if !condition_clauses.is_empty() {
    //     paginated_query.push_str(" WHERE ");
    //     paginated_query.push_str(&condition_clauses.join(" AND "));
    // }

    // if let Some(sort_by) = &page_request.sort_by {
    //     paginated_query.push_str(&format!(" ORDER BY {} {}", sort_by, page_request.order.clone().unwrap_or_else(|| "asc".to_string())));
    // }
    //
    paginated_query.push_str(&" LIMIT ? OFFSET ?".to_string());
    println!("{}", paginated_query);
    params.push(JsonValue::Number(Number::from(page_request.page_size)));
    params.push(JsonValue::Number(Number::from(offset)));
    println!("查询条件:{:?}", params);
    let query_temp = sqlx::query_as::<Sqlite, T>(&*paginated_query);
    let bind_query_as = bind_query_as_json_list(query_temp, params);
    let result = bind_query_as.fetch_all(pool).await;
    if let Err(e) = result {
        return Err(e.to_string());
    }
    //查询结果
    Ok(PageResult::new(
        result.unwrap(),
        total,
        page_request.page_no,
        page_request.page_size,
        total_pages,
    ))
}

///
/// 查询数据
///
pub async fn select<'q, T>(pool: &SqlitePool, sql: &str, values: Vec<JsonValue>) -> Result<Vec<T>, ()>
where
    T: for<'a> FromRow<'a, SqliteRow> + Send + Unpin,
{
    let query_temp = sqlx::query_as::<Sqlite, T>(sql);
    let bind_query_as = bind_query_as_json_list(query_temp, values);
    let result = bind_query_as.fetch_all(pool).await.unwrap();
    Ok(result)
}

///
/// 保存数据
///
pub async fn insert(pool: &SqlitePool, sql: &str, _values: Vec<JsonValue>) -> Result<u64, String> {
    let query = sqlx::query(&sql);
    let insert_json = bind_cond_json_list(query, _values);
    let result = pool.execute(insert_json).await;
    match result {
        Ok(_) => Ok(result.unwrap().rows_affected()),
        Err(_) => Err(result.unwrap_err().to_string()),
    }
}

///
/// 执行写操作
///
pub async fn execute(pool: &SqlitePool, sql: &str, _values: Vec<JsonValue>) -> Result<u64, String> {
    println!("保存数据:{:?}", _values.clone());
    let query = sqlx::query(&sql);
    let bind_query = bind_cond_json_list(query, _values);
    let result = pool.execute(bind_query).await;
    match result {
        Ok(_) => Ok(result.unwrap().rows_affected()),
        Err(_) => Err(result.unwrap_err().to_string()),
    }
}

///
/// 将公用的代码逻辑抽象为方法
/// query:查询对象，按照固定参数传入
///
/// 返回值：将query赋值至可变对象,赋值完毕做为返回值返回
///
fn bind_cond_json_list<'a>(
    query: Query<'a, Sqlite, SqliteArguments<'a>>,
    values: Vec<JsonValue>,
) -> Query<'a, Sqlite, SqliteArguments<'a>>
where
    Sqlite: Database,
{
    let mut query_temp: Query<'a, Sqlite, SqliteArguments<'a>> = query;
    for value in values {
        if value.is_null() {
            query_temp = query_temp.bind(None::<JsonValue>);
        } else if value.is_string() {
            query_temp = query_temp.bind(value.as_str().unwrap().to_string());
        } else if let Some(number) = value.as_number() {
            query_temp = query_temp.bind(number.as_f64());
        } else {
            query_temp = query_temp.bind(value.to_string());
        }
    }
    query_temp
}

///
/// 将公用的代码逻辑抽象为方法
/// query:查询对象，按照固定参数传入
///
/// 返回值：将query赋值至可变对象,赋值完毕做为返回值返回
///
fn bind_query_as_json_list<'a, T>(
    query: QueryAs<'a, Sqlite, T, SqliteArguments<'a>>,
    values: Vec<JsonValue>,
) -> QueryAs<'a, Sqlite, T, SqliteArguments<'a>>
where
    Sqlite: Database,
{
    let mut query_temp: QueryAs<'a, Sqlite, T, SqliteArguments<'a>> = query;
    for value in values {
        if value.is_null() {
            query_temp = query_temp.bind(None::<JsonValue>);
        } else if value.is_string() {
            query_temp = query_temp.bind(value.as_str().unwrap().to_string());
        } else if let Some(number) = value.as_number() {
            query_temp = query_temp.bind(number.as_f64());
        } else {
            query_temp = query_temp.bind(value.to_string());
        }
    }
    query_temp
}
