mod db;
mod todo;

use std::io;

use db::init;
use todo::{add_todo, done_todo, show_all_todo};

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
        } else if action == String::from("done") {
            let _ = done_todo().await;
        } else {
            println!("non match action.");
        }

        let _ = show_all_todo().await;
    }
}
