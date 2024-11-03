use std::io::stdin;
use std::vec;

static mut TODOS: Vec<String> = vec![];

fn main() {
    loop {
        println!("=====================");
        println!("Welcome to Rust todo app");
        println!("Please enter a command");
        println!("1. Add todo");
        println!("2. List todo");
        println!("3. Mark todo as done");
        println!("4. Delete todo");
        println!("5. Exit");

        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();
        let input = input.trim();

        match input {
            "1" => add_todo(),
            "2" => list_todo(),
            "3" => mark_todo_as_done(),
            "4" => delete_todo(),
            "5" => {
                println!("Goodbye");
                break;
            }
            _ => println!("Invalid command"),
        }
    }
}

fn add_todo() {
    // just save the todo in array for now
    println!("Please enter the todo");
    let mut todo = String::new();
    stdin().read_line(&mut todo).unwrap();
    let todo = todo.trim();

    unsafe {
        TODOS.push(todo.to_string());
    }
    println!("Todo added successfully");
}

fn list_todo() {
    println!("List of todos:");
    unsafe {
        for (index, todo) in TODOS.iter().enumerate() {
            println!("{}. {}", index + 1, todo);
        }
    }
}

fn mark_todo_as_done() {
    list_todo();
    println!("Please enter the todo number to mark as done");

    unsafe {
        if TODOS.len() == 0 {
            println!("No todos to mark as done");
            return;
        }
        let mut todo_number = String::new();
        stdin().read_line(&mut todo_number).unwrap();
        let todo_number: usize = match todo_number.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid todo number");
                return;
            }
        };

        if todo_number > TODOS.len() {
            println!("Invalid todo number");
        } else {
            TODOS.remove(todo_number - 1);
            println!("Todo marked as done successfully");
        }
    }
}

fn delete_todo() {
    list_todo();
    println!("Please enter the todo number to delete");
    unsafe {
        if TODOS.len() == 0 {
            println!("No todos to delete");
            return;
        }
        let mut todo_number = String::new();
        stdin().read_line(&mut todo_number).unwrap();
        let todo_number: usize = todo_number.trim().parse().unwrap();

        if todo_number > TODOS.len() {
            println!("Invalid todo number");
        } else {
            TODOS.remove(todo_number - 1);
            println!("Todo deleted successfully");
        }
    }
}
