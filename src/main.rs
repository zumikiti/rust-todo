use std::{env, io};

use dotenv::dotenv;
use sqlx::{postgres::PgPoolOptions, prelude::FromRow};

#[derive(Debug, FromRow)]
struct Todo {
    uid: i64,
    value: String,
    done: bool,
}

#[tokio::main]
async fn main() {
    println!("start todo list");

    let _ = init().await;

    let _ = show_all_todo().await;

    loop {
        println!("Please input action. [add / done]");

        let mut action = String::new();

        io::stdin()
            .read_line(&mut action)
            .expect("Failed to read line");

        action = match action.trim().parse() {
            Ok(string) => string,
            Err(_) => continue,
        };

        if action == String::from("add") {
            let _ = add_todo().await;
        // } else if action == String::from("done") {
            // todo_list = done_todo(todo_list);
        } else {
            println!("non match action.");
        }

        let _ = show_all_todo().await;
    }
}

async fn db() -> sqlx::Pool<sqlx::Postgres> {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("Failed to connect to the database");

    pool
}

async fn init() -> Result<(), sqlx::Error> {
    let pool = db().await;
    sqlx::migrate!("./migrations").run(&pool).await?;

    Ok(())
}

async fn select_all() -> Vec<Todo> {
    let pool = db().await;

    let q = "SELECT id AS uid, name AS value, done FROM todos ORDER BY id asc";

    let todos: Vec<Todo> = sqlx::query_as::<_, Todo>(q)
        .fetch_all(&pool)
        .await
        .expect("failed to fetch todos");

    todos
}

async fn create(value: &str) {
    let pool = db().await;

    let query = "INSERT INTO todos (name) VALUES ($1)";

    sqlx::query(query)
        .bind(&value)
        .execute(&pool)
        .await
        .expect("failed inserted.");
}

async fn show_all_todo() {
    println!("### タスク一覧 ###");

    let todos = select_all().await;

    todos.iter().for_each(|todo| {
        let done_text = if todo.done { "完了" } else { "未完了" };
        println!("ID:{} | {} ({})", todo.uid, todo.value, done_text);
    })
}

async fn add_todo() -> Result<(), &'static str> {
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

fn done_todo(mut todo_list: Vec<Todo>) -> Vec<Todo> {
    println!("Please input done todo id.");

    let mut num = String::new();

    io::stdin()
        .read_line(&mut num)
        .expect("Failed to read line");

    let num = match num.trim().parse::<i64>() {
        Ok(num) => num,
        Err(_) => {
            return todo_list;
        }
    };

    let mut found = false;
    for todo in &mut todo_list {
        if todo.uid == num {
            todo.done = true;
            found = true;
            break;
        }
    }

    if found {
        println!("updated!");
    } else {
        println!("undefined todo. id: {}..", num);
    }

    return todo_list;
}
