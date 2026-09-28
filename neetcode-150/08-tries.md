# NeetCode 150: Category 08 — Tries (Prefix Trees)

Tries optimize dynamic prefix searches, auto-complete, and spell-checking dictionary operations in $O(L)$ time where $L$ is word length.

---

## 1. Implement Trie (Prefix Tree) (LeetCode 208) — Medium
A trie (pronounced as "try") or prefix tree is a tree data structure used to efficiently store and retrieve keys in a dataset of strings. Implement `insert`, `search`, and `starts_with`.

```rust
use std::collections::HashMap;

#[derive(Default)]
pub struct Trie {
    children: HashMap<char, Trie>,
    is_end_of_word: bool,
}

impl Trie {
    pub fn new() -> Self {
        Trie::default()
    }

    pub fn insert(&mut self, word: String) {
        // TODO: Implement solution
        todo!()
    }

    pub fn search(&self, word: String) -> bool {
        // TODO: Implement solution
        todo!()
    }

    pub fn starts_with(&self, prefix: String) -> bool {
        // TODO: Implement solution
        todo!()
    }
}
```

---

## 2. Design Add and Search Words Data Structure (LeetCode 211) — Medium
Design a data structure that supports adding new words and finding if a string matches any previously added string, where `.` can match any letter.

```rust
use std::collections::HashMap;

#[derive(Default)]
pub struct WordDictionary {
    children: HashMap<char, WordDictionary>,
    is_end: bool,
}

impl WordDictionary {
    pub fn new() -> Self {
        WordDictionary::default()
    }

    pub fn add_word(&mut self, word: String) {
        // TODO: Implement solution
        todo!()
    }

    pub fn search(&self, word: String) -> bool {
        // TODO: Implement solution with '.' wildcard recursion
        todo!()
    }
}
```

---

## 3. Word Search II (LeetCode 212) — Hard
Given an $m \times n$ `board` of characters and a list of strings `words`, return all words on the board by backtracking combined with a Prefix Trie.

```rust
pub fn find_words(board: Vec<Vec<char>>, words: Vec<String>) -> Vec<String> {
    // TODO: Implement solution
    todo!()
}
```
