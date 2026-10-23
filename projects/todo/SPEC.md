# CLI Todo App - Functional Specification

## 1. Overview
The CLI Todo application is a task management tool operated entirely from the terminal. It provides an interactive or command-driven way to track daily tasks, change their status, and manage task lifecycles.

This document describes **what** the application should do and its user-facing behavior, independent of specific implementation details.

---

## 2. Core Concepts & Data Model

### 2.1 Todo Item
Each Todo task has the following attributes:
- **Identifier (`id`)**: A unique unsigned integer identifier (e.g. `1, 2, 3...`).
- **Title / Description (`title`)**: A non-empty text string describing the task.
- **Status (`completed`)**: A boolean state indicating whether the task is pending or completed (`false` for pending, `true` for done).

---

## 3. Required Capabilities

### 3.1 Adding a Task
- **Action**: Add a new task with a specified title.
- **Rules**:
  - The title cannot be empty or solely whitespace.
  - Automatically assigns a unique, increasing numeric ID.
  - Default status is always **incomplete / pending**.
  - Returns a success confirmation containing the new task's ID and title.

### 3.2 Listing Tasks
- **Action**: View current tasks.
- **Views**:
  1. **List All**: Displays all tasks showing their ID, title, and current status (e.g., `[ ]` for pending, `[x]` for completed).
  2. **Filter by Status**:
     - Only pending tasks.
     - Only completed tasks.
  3. **Empty State**: If there are no tasks, display an informative message (e.g. `No tasks found`).

### 3.3 Completing a Task
- **Action**: Mark an existing task as completed using its ID.
- **Rules**:
  - If the ID exists, updates status to completed.
  - If the task is already completed, it can either inform the user or safely remain completed without error.
  - If the ID does not exist, returns an error message indicating that no task with that ID was found.

### 3.4 Uncompleting / Reopening a Task (Optional / Nice-to-have)
- **Action**: Mark a completed task back to pending using its ID.
- **Rules**:
  - If the ID does not exist, returns an appropriate not found error.

### 3.5 Editing / Updating a Task Title
- **Action**: Update the title of an existing task by ID.
- **Rules**:
  - Replaces old title with the new one.
  - New title cannot be empty.
  - Returns error if the ID is not found.

### 3.6 Removing / Deleting a Task
- **Action**: Delete a task by ID.
- **Rules**:
  - Removes the task from the list entirely.
  - Returns an error if the ID does not exist.
  - Removing a task does not change or re-order other tasks' existing IDs.

### 3.7 Clearing / Bulk Operations
- **Action**: Clear all completed tasks or clear all tasks entirely.

---

## 4. Error Handling & Edge Cases
The application must handle common error scenarios gracefully without crashing (no panics):
1. **Invalid ID**: Providing an ID that does not correspond to any known task.
2. **Empty Input**: Attempting to add or rename a task with empty text.
3. **Invalid Commands**: Entering an unknown option or command.
4. **Boundary Checks**: Handling empty list queries without errors.

---

## 5. Testing Contract
To allow extensive automated unit and integration tests, the core logic should expose operations that can be tested in isolation:
- `add_todo(title)` -> returns `Result<Todo, Error>` or assigns ID.
- `list_todos(filter)` -> returns a slice or list of tasks.
- `get_todo(id)` -> returns `Option<Todo>` or reference.
- `complete_todo(id)` -> returns `Result<(), Error>`.
- `remove_todo(id)` -> returns `Result<Todo, Error>`.
- `update_todo(id, new_title)` -> returns `Result<(), Error>`.
