struct Todo {
    uid: i64,
    value: String,
    done: bool,
}

fn main() {
    println!("start todo list");

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

    show_all_todo(todo_list);
}

fn show_all_todo(todos: Vec<Todo>) {
    println!("### タスク一覧 ###");

    for todo in todos {
        let done_text = if todo.done { "完了" } else { "未完了" };
        println!("ID:{} | {} ({})", todo.uid, todo.value, done_text);
    }
}
