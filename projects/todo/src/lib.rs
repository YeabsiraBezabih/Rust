//! Core business logic for the CLI Todo App.
//!
//! This module acts as the "Engine" (Library).
//! It has zero knowledge of the terminal, formatting, or command line arguments,
//! making it 100% testable and reusable for web APIs or GUIs.

/// Represents a single task item.
///
/// `#[derive(Debug, Clone, PartialEq)]` automatically generates:
/// - `Debug`: allows printing with `{:?}` in tests or error logs.
/// - `Clone`: allows making a duplicate copy of a Todo when needed.
/// - `PartialEq`: allows comparing two Todos using `==`.
#[derive(Debug, Clone, PartialEq)]
pub struct Todo {
    pub id: u64,
    pub title: String,
    pub completed: bool,
}

/// Represents the status filter when querying tasks.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Filter {
    All,
    Completed,
    Pending,
}

/// The collection managing all todos and tracking ID generation.
#[derive(Debug, Default)]
pub struct TodoList {
    items: Vec<Todo>,
    next_id: u64,
}

impl TodoList {
    /// Creates a new, empty `TodoList`.
    ///
    /// Concept: Associated Function (Constructor)
    /// - Notice there is NO `&self` here because no list exists yet.
    /// - Returns `Self` (shorthand for `TodoList`).
    pub fn new() -> Self {
        TodoList {
            items: Vec::new(),
            next_id: 0,
        }
    }

    /// Returns the number of tasks currently in the list.
    ///
    /// Concept: Borrowing (`&self`)
    /// - `&self` means we only need to read data from `self`, not modify it.
    /// - We return `self.items.len()` (actual current count) instead of `next_id`,
    ///   because if an item gets deleted, `items.len()` shrinks, but `next_id` does not!
    pub fn len(&self) -> usize {
        self.items.len()
    }

    /// Returns `true` if the list has zero tasks.
    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }

    /// Adds a new task to the list with the given title.
    ///
    /// Concept: `&mut self` and string slices (`&str`)
    /// - `&mut self`: We need mutable access to add to `self.items` and increment `self.next_id`.
    /// - `title: &str`: Accepting a string slice allows callers to pass string literals (`"Buy milk"`),
    ///   borrowed Strings (`&my_string`), or formatted strings without forcing them to allocate first.
    /// - `Result<u64, String>`: Returns `Ok(new_id)` on success, or `Err(reason)` on failure.
    pub fn add(&mut self, title: &str) -> Result<u64, String> {
        // Validation: .trim() removes leading/trailing whitespace.
        // If the trimmed string is empty, the user gave us "" or "   ".
        if title.trim().is_empty() {
            return Err("Todo title cannot be empty".to_string());
        }

        // Increment ID counter: ensures every task gets a unique, increasing ID.
        self.next_id += 1;

        let new_todo = Todo {
            id: self.next_id,
            // .to_string() converts the borrowed `&str` into an owned `String` stored in the struct.
            title: title.to_string(),
            completed: false,
        };

        // Push the new struct into our Vec.
        self.items.push(new_todo);

        // Return the assigned ID so the caller can reference it.
        Ok(self.next_id)
    }

    /// Retrieves an immutable reference to a task by its ID.
    ///
    /// Concept: Borrowing instead of cloning (`Option<&Todo>`)
    /// - `.iter()` borrows each element as `&Todo`.
    /// - `.find(|t| t.id == id)` searches and returns `Option<&Todo>`.
    /// - By returning a reference, we avoid copying/cloning memory.
    pub fn get(&self, id: u64) -> Option<&Todo> {
        self.items.iter().find(|todo| todo.id == id)
    }

    /// Marks a task as completed by ID.
    ///
    /// Concept: Mutable iteration (`iter_mut()`)
    /// - `.iter_mut()` gives us a mutable reference (`&mut Todo`).
    /// - This lets us modify `todo.completed = true` in-place inside the vector.
    pub fn complete(&mut self, id: u64) -> Result<(), String> {
        match self.items.iter_mut().find(|todo| todo.id == id) {
            Some(todo) => {
                todo.completed = true;
                Ok(())
            }
            None => Err(format!("Todo with ID {} not found", id)),
        }
    }

    /// Marks a task back to incomplete/pending.
    pub fn uncomplete(&mut self, id: u64) -> Result<(), String> {
        match self.items.iter_mut().find(|todo| todo.id == id) {
            Some(todo) => {
                todo.completed = false;
                Ok(())
            }
            None => Err(format!("Todo with ID {} not found", id)),
        }
    }

    /// Updates the title of an existing task.
    ///
    /// Validates that the new title is not blank, then updates in-place.
    pub fn update_title(&mut self, id: u64, new_title: &str) -> Result<(), String> {
        if new_title.trim().is_empty() {
            return Err("New title cannot be empty".to_string());
        }

        match self.items.iter_mut().find(|todo| todo.id == id) {
            Some(todo) => {
                todo.title = new_title.to_string();
                Ok(())
            }
            None => Err(format!("Todo with ID {} not found", id)),
        }
    }

    /// Removes a task by ID and returns the removed task.
    ///
    /// Concept: `.position()` and `Vec::remove()`
    /// - `.position()` finds the index `usize` of the matching item.
    /// - `self.items.remove(index)` removes it from the vector and moves ownership to the caller.
    pub fn remove(&mut self, id: u64) -> Result<Todo, String> {
        match self.items.iter().position(|todo| todo.id == id) {
            Some(index) => {
                let removed = self.items.remove(index);
                Ok(removed)
            }
            None => Err(format!("Todo with ID {} not found", id)),
        }
    }

    /// Returns a filtered list of references to tasks based on status.
    ///
    /// Concept: Pattern matching and iterator `.filter()`
    /// - Returns `Vec<&Todo>`: references to the items inside the list (no cloning!).
    pub fn filter(&self, filter: Filter) -> Vec<&Todo> {
        match filter {
            // All items regardless of status:
            Filter::All => self.items.iter().collect(),

            // Only completed items:
            Filter::Completed => self.items.iter().filter(|t| t.completed).collect(),

            // Only pending items (not completed):
            Filter::Pending => self.items.iter().filter(|t| !t.completed).collect(),
        }
    }

    /// Removes all completed tasks from the list.
    ///
    /// Concept: `Vec::retain()`
    /// - `.retain(|item| condition)` keeps items where condition is TRUE,
    ///   and drops items where condition is FALSE.
    /// - Since we want to drop completed tasks, we keep tasks where `!t.completed`.
    pub fn clear_completed(&mut self) {
        self.items.retain(|todo| !todo.completed);
    }
}