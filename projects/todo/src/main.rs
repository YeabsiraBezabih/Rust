//! Command Line Interface (CLI) for the Todo App.
//!
//! Concept: Binary vs Library
//! - `src/lib.rs` is our engine: it holds our data and business rules.
//! - `src/main.rs` is the interface: it reads commands from the terminal,
//!   calls methods on `TodoList`, and prints pretty messages to the user.

use std::io::{self, Write};
use todo::{Filter, Todo, TodoList};

fn main() {
    let mut todo_list = TodoList::new();

    println!("========================================");
    println!("   🦀 Welcome to Rusty Todo CLI! 🦀    ");
    println!("========================================");
    print_help();

    // The Main Event Loop: Keeps running until the user types "exit" or "quit"
    loop {
        // 1. Print the prompt symbol
        print!("\ntodo> ");
        // In Rust, stdout is line-buffered by default. Since print! doesn't include a newline,
        // we must explicitly flush stdout so "todo> " appears on the screen immediately.
        io::stdout().flush().expect("Failed to flush stdout");

        // 2. Read a line of text from standard input (the keyboard)
        let mut input = String::new();
        if io::stdin().read_line(&mut input).is_err() {
            println!("❌ Failed to read input line.");
            continue;
        }

        // 3. Clean up input: remove leading/trailing whitespace & newlines ('\n')
        let trimmed = input.trim();
        if trimmed.is_empty() {
            continue;
        }

        // 4. Split command name and arguments
        // Example: "add Buy groceries for tonight"
        // command = "add", args = "Buy groceries for tonight"
        let mut parts = trimmed.splitn(2, ' ');
        let command = parts.next().unwrap_or("").to_lowercase();
        let args = parts.next().unwrap_or("").trim();

        // 5. Match the command and perform the action
        match command.as_str() {
            "add" => handle_add(&mut todo_list, args),
            "list" | "ls" => handle_list(&todo_list, args),
            "done" | "complete" => handle_complete(&mut todo_list, args),
            "undo" => handle_undo(&mut todo_list, args),
            "edit" => handle_edit(&mut todo_list, args),
            "rm" | "delete" => handle_remove(&mut todo_list, args),
            "clear" => handle_clear(&mut todo_list),
            "help" => print_help(),
            "exit" | "quit" | "q" => {
                println!("👋 Goodbye!");
                break;
            }
            unknown => {
                println!("❓ Unknown command: '{}'. Type 'help' for options.", unknown);
            }
        }
    }
}

/// Prints available commands and usage guide.
fn print_help() {
    println!("\nAvailable Commands:");
    println!("  add <title>            - Add a new task (e.g. 'add Buy milk')");
    println!("  list [all|done|todo]   - List tasks (default: all)");
    println!("  done <id>              - Mark task as completed (e.g. 'done 1')");
    println!("  undo <id>              - Mark task as incomplete (e.g. 'undo 1')");
    println!("  edit <id> <new_title>  - Rename a task (e.g. 'edit 1 Buy oat milk')");
    println!("  rm <id>                - Delete a task by ID");
    println!("  clear                  - Remove all completed tasks");
    println!("  help                   - Show this help message");
    println!("  exit / quit            - Exit the application");
}

/// Handles the "add <title>" command.
fn handle_add(list: &mut TodoList, title: &str) {
    if title.is_empty() {
        println!("⚠️  Usage: add <title>");
        return;
    }

    match list.add(title) {
        Ok(id) => println!("✅ Added task #{} -> \"{}\"", id, title),
        Err(err) => println!("❌ Error: {}", err),
    }
}

/// Handles "list [all|done|todo]" with formatted display.
fn handle_list(list: &TodoList, filter_arg: &str) {
    if list.is_empty() {
        println!("📭 Your todo list is empty!");
        return;
    }

    let filter = match filter_arg.to_lowercase().as_str() {
        "done" | "completed" => Filter::Completed,
        "todo" | "pending" => Filter::Pending,
        _ => Filter::All,
    };

    let items = list.filter(filter);
    if items.is_empty() {
        println!("No tasks match the '{}' filter.", filter_arg);
        return;
    }

    println!("\n📋 Tasks:");
    for todo in items {
        print_todo_item(todo);
    }
    println!("Total: {} items", list.len());
}

/// Helper function to render a task cleanly with checkbox status.
fn print_todo_item(todo: &Todo) {
    let (checkbox, status_color) = if todo.completed {
        ("[✓]", "\x1b[32m") // Green for completed
    } else {
        ("[ ]", "\x1b[33m") // Yellow for pending
    };
    let reset_color = "\x1b[0m";

    println!(
        "  {color}{checkbox}{reset} {bold}#{:<3}{reset} {}",
        todo.id,
        todo.title,
        color = status_color,
        checkbox = checkbox,
        reset = reset_color,
        bold = "\x1b[1m"
    );
}

/// Handles "done <id>".
fn handle_complete(list: &mut TodoList, id_str: &str) {
    match parse_id(id_str) {
        Some(id) => match list.complete(id) {
            Ok(()) => println!("🎉 Task #{} marked as completed!", id),
            Err(err) => println!("❌ {}", err),
        },
        None => println!("⚠️  Please specify a valid numeric ID (e.g. 'done 1')"),
    }
}

/// Handles "undo <id>".
fn handle_undo(list: &mut TodoList, id_str: &str) {
    match parse_id(id_str) {
        Some(id) => match list.uncomplete(id) {
            Ok(()) => println!("🔄 Task #{} marked back to pending!", id),
            Err(err) => println!("❌ {}", err),
        },
        None => println!("⚠️  Please specify a valid numeric ID (e.g. 'undo 1')"),
    }
}

/// Handles "edit <id> <new_title>".
fn handle_edit(list: &mut TodoList, args: &str) {
    let mut parts = args.splitn(2, ' ');
    let id_part = parts.next().unwrap_or("");
    let new_title = parts.next().unwrap_or("").trim();

    if new_title.is_empty() {
        println!("⚠️  Usage: edit <id> <new_title>");
        return;
    }

    match parse_id(id_part) {
        Some(id) => match list.update_title(id, new_title) {
            Ok(()) => println!("✏️  Task #{} updated to \"{}\"", id, new_title),
            Err(err) => println!("❌ {}", err),
        },
        None => println!("⚠️  Please specify a valid numeric ID (e.g. 'edit 1 New Title')"),
    }
}

/// Handles "rm <id>".
fn handle_remove(list: &mut TodoList, id_str: &str) {
    match parse_id(id_str) {
        Some(id) => match list.remove(id) {
            Ok(removed) => println!("🗑️  Removed task #{}: \"{}\"", removed.id, removed.title),
            Err(err) => println!("❌ {}", err),
        },
        None => println!("⚠️  Please specify a valid numeric ID (e.g. 'rm 1')"),
    }
}

/// Handles "clear".
fn handle_clear(list: &mut TodoList) {
    let before = list.len();
    list.clear_completed();
    let removed_count = before - list.len();
    println!("🧹 Cleared {} completed task(s)!", removed_count);
}

/// Helper function to parse user input into a u64 ID.
/// Uses `str::parse::<u64>()` and converts the `Result` into an `Option`.
fn parse_id(s: &str) -> Option<u64> {
    s.trim().parse::<u64>().ok()
}