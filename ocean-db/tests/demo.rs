#[cfg(test)]
mod test {
    ///
    ///
    /// 数据库的名称
    ///
    const DB_NAME: &'static str = "ocean_todo.db";

    pub async fn open(db_name: &str) -> Result<SqlitePool, sqlx::Error> {
        let home_dir = format!("{}/{}", get_home_dir(), String::from("ocean"));
        let conn_str = format!("sqlite:{}/{}", home_dir, db_name);
        let _result = fs::create_dir_all(home_dir)?;
        let connection_options =
            SqliteConnectOptions::from_str(conn_str.as_str())?.create_if_missing(true);
        SqlitePoolOptions::new()
            .max_connections(5)
            .connect_with(connection_options)
            .await
    }

    pub async fn get_pool() -> SqlitePool {
        open(DB_NAME).await.unwrap()
    }

    pub async fn init_table() {
        let pool = get_pool().await;
        let task_result = pool
            .execute(
                r#"
    CREATE TABLE IF NOT EXISTS tb_task
    (
        id          TEXT,
        title       TEXT,
        remark      TEXT,
        create_time TEXT,
        update_time TEXT,
        display_order INTEGER
    )
    "#,
            )
            .await;
        if let Err(e) = task_result {
            println!("Failed to create table: {}", e.to_string());
        }
        let user_result = pool
            .execute(
                r#"
    CREATE TABLE IF NOT EXISTS tb_users
    (
        id          TEXT,
        username    TEXT,
        password    TEXT,
        email       TEXT,
        create_time TEXT,
        update_time TEXT,
        display_order INTEGER
    )
    "#,
            )
            .await;
        if let Err(e) = user_result {
            println!("Failed to create table: {}", e.to_string());
        }
    }

    ///
    /// FromRow为了支持分页映射，需要使用该宏
    ///
    #[derive(
        Default, Deserialize, Serialize, Clone, Debug, FromRow, ToFieldJsonValue, GetFieldList,
    )]
    pub struct Task {
        pub id: Option<String>,
        pub title: Option<String>,
        pub remark: Option<String>,
        pub create_time: Option<String>,
        pub update_time: Option<String>,
        pub display_order: Option<u64>,
    }

    use chrono::Local;
    use dirs::home_dir;
    use ocean_db::sqlite::execute::{insert, paginate, select};
    use ocean_db::sqlite::model::{PageRequest, PageResult};
    use ocean_macros::{GetFieldList, ToFieldJsonValue};
    use serde::{Deserialize, Serialize};
    use sqlx::sqlite::{SqliteConnectOptions, SqlitePoolOptions};
    use sqlx::{Executor, FromRow, SqlitePool};
    use std::fs;
    use std::str::FromStr;

    pub fn get_home_dir() -> String {
        if let Some(home_dir) = home_dir() {
            home_dir.to_str().unwrap().to_string()
        } else {
            ".".to_string()
        }
    }

    #[tokio::test]
    pub async fn test_home_dir() {
        let home_dir = get_home_dir();
        println!("{:?}", home_dir);
    }

    #[tokio::test]
    pub async fn test_page() {
        let pool = get_pool().await;
        let now = Local::now();
        let task = Task {
            id: None,
            title: Some("title".to_string()),
            remark: Some("remark".to_string()),
            create_time: Some(now.format("%Y-%m-%d %H:%M:%S").to_string()),
            update_time: Some(now.format("%Y-%m-%d %H:%M:%S").to_string()),
            display_order: Some(now.timestamp_millis() as u64),
        };
        insert(&pool, "insert into tb_task(id,title,remark,create_time,update_time,display_order) values(?1,?2,?3,?4,?5,?6)", task.to_json_value()).await.unwrap();
        // init_table().await;
        let request = PageRequest::new(1, 10);
        let result: PageResult<Task> = paginate(&pool, "select * from tb_task", &request).await.unwrap();
        println!("{:?}", result);
    }

    #[tokio::test]
    pub async fn test_find() {
        let pool = get_pool().await;
        let result: Vec<Task> = select(&pool, "select * from tb_task", vec![]).await.unwrap();
        println!("{:?}", result);
    }
}