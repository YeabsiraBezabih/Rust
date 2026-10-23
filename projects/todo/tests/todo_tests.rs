//! Comprehensive Test Suite for the CLI Todo App
//!
//! You can run these tests with:
//!     cargo test
//!
//! Your task is to implement the library types and functions in `src/lib.rs` (or modular code)
//! such that all of these tests compile and pass!

use todo::{TodoList, Filter};

#[test]
fn test_new_list_is_empty() {
    let list = TodoList::new();
    assert_eq!(list.len(), 0);
    assert!(list.is_empty());
}

#[test]
fn test_add_todo_success() {
    let mut list = TodoList::new();
    let id = list.add("Buy groceries").expect("Should successfully add a todo");
    
    assert_eq!(id, 1);
    assert_eq!(list.len(), 1);
    assert!(!list.is_empty());

    let todo = list.get(id).expect("Todo should exist");
    assert_eq!(todo.id, 1);
    assert_eq!(todo.title, "Buy groceries");
    assert!(!todo.completed);
}

#[test]
fn test_add_multiple_todos_increments_ids() {
    let mut list = TodoList::new();
    let id1 = list.add("Task 1").unwrap();
    let id2 = list.add("Task 2").unwrap();
    let id3 = list.add("Task 3").unwrap();

    assert_eq!(id1, 1);
    assert_eq!(id2, 2);
    assert_eq!(id3, 3);
    assert_eq!(list.len(), 3);
}

#[test]
fn test_add_empty_or_whitespace_title_fails() {
    let mut list = TodoList::new();

    let err_empty = list.add("");
    assert!(err_empty.is_err(), "Adding empty title should return an Err");

    let err_spaces = list.add("   ");
    assert!(err_spaces.is_err(), "Adding whitespace-only title should return an Err");

    assert_eq!(list.len(), 0);
}

#[test]
fn test_complete_todo_success() {
    let mut list = TodoList::new();
    let id = list.add("Learn Rust ownership").unwrap();

    let result = list.complete(id);
    assert!(result.is_ok());

    let item = list.get(id).unwrap();
    assert!(item.completed, "Task should now be completed");
}

#[test]
fn test_complete_nonexistent_todo_fails() {
    let mut list = TodoList::new();
    list.add("Existing item").unwrap();

    let result = list.complete(999);
    assert!(result.is_err(), "Completing non-existent ID should return an Err");
}

#[test]
fn test_uncomplete_todo() {
    let mut list = TodoList::new();
    let id = list.add("Do dishes").unwrap();
    list.complete(id).unwrap();

    let res = list.uncomplete(id);
    assert!(res.is_ok());

    let item = list.get(id).unwrap();
    assert!(!item.completed, "Task should now be incomplete");
}

#[test]
fn test_update_todo_title() {
    let mut list = TodoList::new();
    let id = list.add("Old title").unwrap();

    let res = list.update_title(id, "Updated title");
    assert!(res.is_ok());

    let item = list.get(id).unwrap();
    assert_eq!(item.title, "Updated title");
}

#[test]
fn test_update_todo_empty_title_fails() {
    let mut list = TodoList::new();
    let id = list.add("Valid title").unwrap();

    let res = list.update_title(id, "   ");
    assert!(res.is_err(), "Updating to whitespace title should return an Err");

    let item = list.get(id).unwrap();
    assert_eq!(item.title, "Valid title", "Title should remain unchanged");
}

#[test]
fn test_remove_todo() {
    let mut list = TodoList::new();
    let id1 = list.add("Keep me").unwrap();
    let id2 = list.add("Delete me").unwrap();

    let removed = list.remove(id2);
    assert!(removed.is_ok());
    assert_eq!(removed.unwrap().title, "Delete me");

    assert_eq!(list.len(), 1);
    assert!(list.get(id2).is_none(), "Deleted item should no longer exist");
    assert!(list.get(id1).is_some(), "Remaining item should still exist");
}

#[test]
fn test_remove_nonexistent_todo_fails() {
    let mut list = TodoList::new();
    let res = list.remove(42);
    assert!(res.is_err(), "Removing non-existent ID should return an Err");
}

#[test]
fn test_id_stability_after_removal() {
    let mut list = TodoList::new();
    let id1 = list.add("First").unwrap();
    let id2 = list.add("Second").unwrap();
    let _ = list.remove(id1);

    // Adding a new task shouldn't re-use or collide with existing active IDs
    let id3 = list.add("Third").unwrap();
    assert_ne!(id3, id2);
    assert!(list.get(id2).is_some());
    assert!(list.get(id3).is_some());
}

#[test]
fn test_filtering_todos() {
    let mut list = TodoList::new();
    let id1 = list.add("Item 1").unwrap();
    let id2 = list.add("Item 2").unwrap();
    let id3 = list.add("Item 3").unwrap();

    list.complete(id1).unwrap();
    list.complete(id3).unwrap();

    let all = list.filter(Filter::All);
    assert_eq!(all.len(), 3);

    let completed = list.filter(Filter::Completed);
    assert_eq!(completed.len(), 2);
    assert!(completed.iter().all(|t| t.completed));

    let pending = list.filter(Filter::Pending);
    assert_eq!(pending.len(), 1);
    assert_eq!(pending[0].id, id2);
    assert!(!pending[0].completed);
}

#[test]
fn test_clear_completed() {
    let mut list = TodoList::new();
    let id1 = list.add("Item 1").unwrap();
    let id2 = list.add("Item 2").unwrap();

    list.complete(id1).unwrap();

    list.clear_completed();
    assert_eq!(list.len(), 1);
    assert!(list.get(id1).is_none());
    assert!(list.get(id2).is_some());
}
