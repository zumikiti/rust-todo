use std::{env, io};

use dotenv::dotenv;
use sqlx::postgres::PgPoolOptions;

struct Todo {
    uid: i64,
    value: String,
    done: bool,
}

fn main() {
    println!("start todo list");

    let _ = select_all_db();

    let mut todo_list: Vec<Todo> = Vec::new();

    todo_list.push(Todo {
        uid: 1,
        value: String::from("hoge"),
        done: false,
    });
    todo_list.push(Todo {
        uid: 2,
        value: String::from("fuga"),
        done: true,
    });

    show_all_todo(&todo_list);

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
            todo_list = add_todo(todo_list);
        } else if action == String::from("done") {
            todo_list = done_todo(todo_list);
        } else {
            println!("non match action.");
        }

        show_all_todo(&todo_list);
    }
}

#[tokio::main]
async fn select_all_db() -> Result<(), sqlx::Error> {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await?;
    
    sqlx::migrate!("./migrations").run(&pool).await?;

    let mut todo_list: Vec<Todo> = Vec::new();

    todo_list.push(Todo {
        uid: 1,
        value: String::from("hoge"),
        done: false,
    });
    todo_list.push(Todo {
        uid: 2,
        value: String::from("fuga"),
        done: true,
    });

    for todo in todo_list {
        create(&todo, &pool).await?;
    }

    Ok(())
}

async fn create(todo: &Todo, pool: &sqlx::PgPool) -> Result<(), sqlx::Error> {
    let query = "INSERT INTO todos (name) VALUES ($1)";

    sqlx::query(query)
        .bind(&todo.value)
        .execute(pool)
        .await?;

    Ok(())
}

fn show_all_todo(todos: &Vec<Todo>) {
    println!("### タスク一覧 ###");

    for todo in todos {
        let done_text = if todo.done { "完了" } else { "未完了" };
        println!("ID:{} | {} ({})", todo.uid, todo.value, done_text);
    }
}

fn add_todo(mut todo_list: Vec<Todo>) -> Vec<Todo> {
    println!("Please input new todo.");

    let mut todo = String::new();

    io::stdin()
        .read_line(&mut todo)
        .expect("Failed to read line");

    todo = match todo.trim().parse() {
        Ok(string) => string,
        Err(_) => {
            return todo_list;
        }
    };

    let num: i64 = (todo_list.len() + 1).try_into().unwrap();

    todo_list.push(Todo {
        uid: num,
        value: todo,
        done: false,
    });

    return todo_list;
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
