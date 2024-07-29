use std::io;

use sqlx::prelude::FromRow;

use crate::db::get_db_pool;

// Todo構造体に対して#[derive(Debug, FromRow)]属性を追加し、SQLクエリの結果を直接Todo型にマッピングできるよう
#[derive(Debug, FromRow)]
struct Todo {
    uid: i64,
    value: String,
    done: bool,
}

async fn select_all() -> Vec<Todo> {
    let pool = get_db_pool().await;

    let q = "SELECT id AS uid, name AS value, done FROM todos ORDER BY id asc";

    let todos: Vec<Todo> = sqlx::query_as::<_, Todo>(q)
        .fetch_all(&pool)
        .await
        .expect("failed to fetch todos");

    todos
}

pub async fn show_all_todo() {
    println!("### タスク一覧 ###");

    let todos = select_all().await;

    todos.iter().for_each(|todo| {
        let done_text = if todo.done { "完了" } else { "未完了" };
        println!("ID:{} | {} ({})", todo.uid, todo.value, done_text);
    })
}

async fn create(value: &str) {
    let pool = get_db_pool().await;

    let query = "INSERT INTO todos (name) VALUES ($1)";

    sqlx::query(query)
        .bind(&value)
        .execute(&pool)
        .await
        .expect("failed inserted.");
}

pub async fn add_todo() -> Result<(), &'static str> {
    println!("Please input new todo.");

    let mut todo = String::new();

    io::stdin()
        .read_line(&mut todo)
        .expect("Failed to read line");

    todo = match todo.trim().parse() {
        Ok(string) => string,
        Err(_) => {
            return Err("add todo parse Err.");
        }
    };

    let _ = create(&todo).await;

    return Ok(());
}

async fn done(uid: i64) {
    let pool = get_db_pool().await;

    let query = "UPDATE todos SET done = 't' WHERE id = $1";

    sqlx::query(query)
        .bind(&uid)
        .execute(&pool)
        .await
        .expect("failed to updated todos.");
}

pub async fn done_todo() -> Result<(), &'static str> {
    println!("Please input done todo id.");

    let mut num = String::new();

    io::stdin()
        .read_line(&mut num)
        .expect("Failed to read line");

    let num = match num.trim().parse::<i64>() {
        Ok(num) => num,
        Err(_) => {
            return Err("done todo parse Err.");
        }
    };

    let _ = done(num).await;

    println!("updated!");

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::db::tests::setup_test_db;

    #[tokio::test]
    async fn test_select_all() {
        let pool = setup_test_db().await;

        // マイグレーション実行
        let _ = sqlx::migrate!("./migrations").run(&pool).await;

        // ここでテスト用データを挿入することができます
        sqlx::query("INSERT INTO todos (name, done) VALUES ('Test Todo', false)")
            .execute(&pool)
            .await
            .expect("Failed to insert test todo");

        let todos: Vec<Todo> =
            sqlx::query_as::<_, Todo>("SELECT id AS uid, name AS value, done FROM todos")
                .fetch_all(&pool)
                .await
                .expect("Failed to fetch todos");

        assert!(!todos.is_empty(), "No todos found");
    }

    #[tokio::test]
    async fn test_show_all_todo() {
        let pool = setup_test_db().await;

        // マイグレーション実行
        let _ = sqlx::migrate!("./migrations").run(&pool).await;

        // ここでテスト用データを挿入することができます
        sqlx::query("INSERT INTO todos (name, done) VALUES ('Test Todo', false)")
            .execute(&pool)
            .await
            .expect("Failed to insert test todo");

        // テスト対象の関数を呼び出します
        show_all_todo().await;
    }
}
