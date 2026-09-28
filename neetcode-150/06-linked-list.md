# NeetCode 150: Category 06 — Linked List

Node manipulation in Rust challenges ownership, pointer swapping, and fast-and-slow pointer mechanics using `Option<Box<ListNode>>`.

```rust
// Standard LeetCode Linked List Node Definition in Rust
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}
```

---

## 1. Reverse Linked List (LeetCode 206) — Easy
Given the head of a singly linked list, reverse the list, and return the reversed list.

```rust
pub fn reverse_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    // TODO: Implement solution
    todo!()
}
```

---

## 2. Merge Two Sorted Lists (LeetCode 21) — Easy
You are given the heads of two sorted linked lists `list1` and `list2`. Merge the two lists into one sorted list.

```rust
pub fn merge_two_lists(
    list1: Option<Box<ListNode>>,
    list2: Option<Box<ListNode>>,
) -> Option<Box<ListNode>> {
    // TODO: Implement solution
    todo!()
}
```

---

## 3. Reorder List (LeetCode 143) — Medium
You are given the head of a singly linked-list. The list can be represented as:
$L_0 \to L_1 \to \dots \to L_{n - 1} \to L_n$
Reorder the list to be on the following form:
$L_0 \to L_n \to L_1 \to L_{n - 1} \to L_2 \to L_{n - 2} \to \dots$

```rust
pub fn reorder_list(head: &mut Option<Box<ListNode>>) {
    // TODO: Implement solution
    todo!()
}
```

---

## 4. Remove Nth Node From End of List (LeetCode 19) — Medium
Given the head of a linked list, remove the $n$-th node from the end of the list and return its head in one pass.

```rust
pub fn remove_nth_from_end(head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
    // TODO: Implement solution
    todo!()
}
```

---

## 5. Copy List with Random Pointer (LeetCode 138) — Medium
A linked list of length `n` is given such that each node contains an additional random pointer, which could point to any node in the list, or null. Construct a deep copy of the list.

---

## 6. Add Two Numbers (LeetCode 2) — Medium
You are given two non-empty linked lists representing two non-negative integers. The digits are stored in reverse order. Add the two numbers and return the sum as a linked list.

```rust
pub fn add_two_numbers(
    l1: Option<Box<ListNode>>,
    l2: Option<Box<ListNode>>,
) -> Option<Box<ListNode>> {
    // TODO: Implement solution
    todo!()
}
```

---

## 7. Linked List Cycle (LeetCode 141) — Easy
Given head, the head of a linked list, determine if the linked list has a cycle in it using Floyd's Tortoise and Hare algorithm.

---

## 8. Find the Duplicate Number (LeetCode 287) — Medium
Given an array of integers `nums` containing `n + 1` integers where each integer is in the range `[1, n]` inclusive. There is only one repeated number in `nums`, return this repeated number without modifying the array and using only $O(1)$ extra space.

```rust
pub fn find_duplicate(nums: Vec<i32>) -> i32 {
    // TODO: Implement solution (Cycle Detection)
    todo!()
}
```

---

## 9. LRU Cache (LeetCode 146) — Medium
Design a data structure that follows the constraints of a Least Recently Used (LRU) cache with $O(1)$ `get` and `put`.

```rust
pub struct LRUCache {
    // TODO: Define fields
}

impl LRUCache {
    pub fn new(capacity: i32) -> Self {
        todo!()
    }

    pub fn get(&mut self, key: i32) -> i32 {
        todo!()
    }

    pub fn put(&mut self, key: i32, value: i32) {
        todo!()
    }
}
```

---

## 10. Merge k Sorted Lists (LeetCode 23) — Hard
You are given an array of `k` linked-lists `lists`, each linked-list is sorted in ascending order. Merge all the linked-lists into one sorted linked-list in $O(N \log k)$ time.

```rust
pub fn merge_k_lists(lists: Vec<Option<Box<ListNode>>>) -> Option<Box<ListNode>> {
    // TODO: Implement solution
    todo!()
}
```

---

## 11. Reverse Nodes in k-Group (LeetCode 25) — Hard
Given the head of a linked list, reverse the nodes of the list `k` at a time, and return the modified list.

```rust
pub fn reverse_k_group(head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
    // TODO: Implement solution
    todo!()
}
```
