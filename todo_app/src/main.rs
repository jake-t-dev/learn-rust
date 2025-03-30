use std::io;

struct TodoItem {
    id: u64,
    title: String,
    completed: bool,
}

struct TodoList {
    items: Vec<TodoItem>,
}

impl TodoList {
    fn new() -> TodoList {
        TodoList {
            items: Vec::new()
        }
    }

    fn add_item(&mut self, title: String) {
        let id = self.items.len() as u64 + 1;
        let new_item = TodoItem {
            id,
            title: title.clone(),
            completed: false,
        };
        self.items.push(new_item);
        println!("Added item: {:?}", title);
    }

    fn list_items(&self) {
        if self.items.is_empty() {
            println!("No items in the list.");
        } else {
            for item in &self.items {
                println!("{}: {} [{}]", item.id, item.title, if item.completed { "X" } else { " " });
            }
        }
    }

    fn complete_item(&mut self, id: u64) {
        if let Some(item) = self.items.iter_mut().find(|i| i.id == id) {
            item.completed = true;
            println!("Completed item: {:?}", item.title);
        } else {
            println!("Item with ID {} not found.", id);
        }
    }
}

fn main() {
    let mut todo_list = TodoList::new();

    loop {
        println!("1. Add item");
        println!("2. List items");
        println!("3. Complete item");
        println!("4. Exit");

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).expect("Failed to read line");
        let choice: u32 = match choice.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid input, please enter a number.");
                continue;
            }
        };

        match choice {
            1 => {
                println!("Enter item title:");
                let mut title = String::new();
                std::io::stdin().read_line(&mut title).unwrap();
                todo_list.add_item(title.trim().to_string());
            }
            2 => todo_list.list_items(),
            3 => {
                println!("Enter item ID to complete:");
                let mut id = String::new();
                std::io::stdin().read_line(&mut id).unwrap();
                let id: u64 = id.trim().parse().unwrap();
                todo_list.complete_item(id);
            }
            4 => break,
            _ => println!("Invalid choice, please try again."),
        }
    }
}
