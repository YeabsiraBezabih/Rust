# NeetCode 150: Category 01 — Arrays & Hashing

This section tests frequency counting, hash map lookups, hash sets, prefix calculations, and in-place array transformations in Rust.

---

## 1. Contains Duplicate (LeetCode 217) — Easy
Given an integer array `nums`, return `true` if any value appears at least twice in the array, and return `false` if every element is distinct.

### Example 1
- **Input**: `nums = [1,2,3,1]`
- **Output**: `true`

### Example 2
- **Input**: `nums = [1,2,3,4]`
- **Output**: `false`

### Constraints
- $1 \le \text{nums.length} \le 10^5$
- $-10^9 \le \text{nums}[i] \le 10^9$

```rust
use std::collections::HashSet;

pub fn contains_duplicate(nums: Vec<i32>) -> bool {
    // TODO: Implement solution
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_contains_duplicate() {
        assert_eq!(contains_duplicate(vec![1, 2, 3, 1]), true);
        assert_eq!(contains_duplicate(vec![1, 2, 3, 4]), false);
    }
}
```

---

## 2. Valid Anagram (LeetCode 242) — Easy
Given two strings `s` and `t`, return `true` if `t` is an anagram of `s`, and `false` otherwise.

### Example 1
- **Input**: `s = "anagram"`, `t = "nagaram"`
- **Output**: `true`

### Example 2
- **Input**: `s = "rat"`, `t = "car"`
- **Output**: `false`

### Constraints
- $1 \le s.\text{length}, t.\text{length} \le 5 \times 10^4$
- `s` and `t` consist of lowercase English letters.

```rust
pub fn is_anagram(s: String, t: String) -> bool {
    // TODO: Implement solution
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_anagram() {
        assert_eq!(is_anagram("anagram".into(), "nagaram".into()), true);
        assert_eq!(is_anagram("rat".into(), "car".into()), false);
    }
}
```

---

## 3. Two Sum (LeetCode 1) — Easy
Given an array of integers `nums` and an integer `target`, return indices of the two numbers such that they add up to `target`. You may assume that each input would have exactly one solution, and you may not use the same element twice.

### Example 1
- **Input**: `nums = [2,7,11,15]`, `target = 9`
- **Output**: `[0,1]`

### Example 2
- **Input**: `nums = [3,2,4]`, `target = 6`
- **Output**: `[1,2]`

### Constraints
- $2 \le \text{nums.length} \le 10^4$
- $-10^9 \le \text{nums}[i] \le 10^9$
- $-10^9 \le \text{target} \le 10^9$

```rust
use std::collections::HashMap;

pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    // TODO: Implement solution
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_two_sum() {
        assert_eq!(two_sum(vec![2, 7, 11, 15], 9), vec![0, 1]);
        assert_eq!(two_sum(vec![3, 2, 4], 6), vec![1, 2]);
    }
}
```

---

## 4. Group Anagrams (LeetCode 49) — Medium
Given an array of strings `strs`, group the anagrams together. You can return the answer in any order.

### Example 1
- **Input**: `strs = ["eat","tea","tan","ate","nat","bat"]`
- **Output**: `[["bat"],["nat","tan"],["ate","eat","tea"]]`

### Constraints
- $1 \le \text{strs.length} \le 10^4$
- $0 \le \text{strs}[i].\text{length} \le 100$

```rust
use std::collections::HashMap;

pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
    // TODO: Implement solution
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_group_anagrams() {
        let input: Vec<String> = vec!["eat","tea","tan","ate","nat","bat"].into_iter().map(String::from).collect();
        let result = group_anagrams(input);
        assert_eq!(result.len(), 3);
    }
}
```

---

## 5. Top K Frequent Elements (LeetCode 347) — Medium
Given an integer array `nums` and an integer `k`, return the `k` most frequent elements. You may return the answer in any order.

### Example 1
- **Input**: `nums = [1,1,1,2,2,3]`, `k = 2`
- **Output**: `[1,2]`

### Constraints
- $1 \le \text{nums.length} \le 10^5$
- $k$ is in the range $[1, \text{number of unique elements}]$.
- Guaranteed that the answer is unique.

```rust
use std::collections::HashMap;

pub fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
    // TODO: Implement solution
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_top_k_frequent() {
        let mut res = top_k_frequent(vec![1, 1, 1, 2, 2, 3], 2);
        res.sort();
        assert_eq!(res, vec![1, 2]);
    }
}
```

---

## 6. Product of Array Except Self (LeetCode 238) — Medium
Given an integer array `nums`, return an array `answer` such that `answer[i]` is equal to the product of all the elements of `nums` except `nums[i]`. You must write an algorithm that runs in $O(n)$ time and without using the division operation.

### Example 1
- **Input**: `nums = [1,2,3,4]`
- **Output**: `[24,12,8,6]`

### Constraints
- $2 \le \text{nums.length} \le 10^5$
- $-30 \le \text{nums}[i] \le 30$

```rust
pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
    // TODO: Implement solution
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_product_except_self() {
        assert_eq!(product_except_self(vec![1, 2, 3, 4]), vec![24, 12, 8, 6]);
    }
}
```

---

## 7. Valid Sudoku (LeetCode 36) — Medium
Determine if a $9 \times 9$ Sudoku board is valid. Only the filled cells need to be validated according to the following rules:
1. Each row must contain the digits `1-9` without repetition.
2. Each column must contain the digits `1-9` without repetition.
3. Each of the nine $3 \times 3$ sub-boxes must contain the digits `1-9` without repetition.

```rust
pub fn is_valid_sudoku(board: Vec<Vec<char>>) -> bool {
    // TODO: Implement solution
    todo!()
}
```

---

## 8. Encode and Decode Strings (LeetCode 271) — Medium
Design an algorithm to encode a list of strings to a single string. The encoded string is then sent over the network and is decoded back to the original list of strings.

```rust
pub struct Codec;

impl Codec {
    pub fn encode(&self, strs: Vec<String>) -> String {
        // TODO: Implement solution
        todo!()
    }

    pub fn decode(&self, s: String) -> Vec<String> {
        // TODO: Implement solution
        todo!()
    }
}
```

---

## 9. Longest Consecutive Sequence (LeetCode 128) — Medium
Given an unsorted array of integers `nums`, return the length of the longest consecutive elements sequence in $O(n)$ time.

### Example 1
- **Input**: `nums = [100,4,200,1,3,2]`
- **Output**: `4` (Sequence: `[1, 2, 3, 4]`)

```rust
use std::collections::HashSet;

pub fn longest_consecutive(nums: Vec<i32>) -> i32 {
    // TODO: Implement solution
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_longest_consecutive() {
        assert_eq!(longest_consecutive(vec![100, 4, 200, 1, 3, 2]), 4);
    }
}
```
