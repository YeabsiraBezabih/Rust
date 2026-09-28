# NeetCode 150: Category 05 — Binary Search

Binary Search reduces exponential search spaces to $O(\log n)$ by bisecting sorted collections or monotonic answer ranges.

---

## 1. Binary Search (LeetCode 704) — Easy
Given an array of integers `nums` which is sorted in ascending order, and an integer `target`, write a function to search `target` in `nums`. If `target` exists, then return its index. Otherwise, return `-1`.

```rust
pub fn search(nums: Vec<i32>, target: i32) -> i32 {
    // TODO: Implement solution
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_search() {
        assert_eq!(search(vec![-1, 0, 3, 5, 9, 12], 9), 4);
        assert_eq!(search(vec![-1, 0, 3, 5, 9, 12], 2), -1);
    }
}
```

---

## 2. Search a 2D Matrix (LeetCode 74) — Medium
Write an efficient algorithm that searches for a value `target` in an $m \times n$ integer matrix `matrix`. This matrix has the following properties:
- Integers in each row are sorted from left to right.
- The first integer of each row is greater than the last integer of the previous row.

```rust
pub fn search_matrix(matrix: Vec<Vec<i32>>, target: i32) -> bool {
    // TODO: Implement solution
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_search_matrix() {
        let matrix = vec![
            vec![1, 3, 5, 7],
            vec![10, 11, 16, 20],
            vec![23, 30, 34, 60]
        ];
        assert_eq!(search_matrix(matrix, 3), true);
    }
}
```

---

## 3. Koko Eating Bananas (LeetCode 875) — Medium
Koko loves to eat bananas. There are `n` piles of bananas, the $i$-th pile has `piles[i]` bananas. Return the minimum integer `k` such that she can eat all the bananas within `h` hours.

```rust
pub fn min_eating_speed(piles: Vec<i32>, h: i32) -> i32 {
    // TODO: Implement solution
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_min_eating_speed() {
        assert_eq!(min_eating_speed(vec![3, 6, 7, 11], 8), 4);
        assert_eq!(min_eating_speed(vec![30, 11, 23, 4, 20], 5), 30);
    }
}
```

---

## 4. Find Minimum in Rotated Sorted Array (LeetCode 153) — Medium
Suppose an array of length `n` sorted in ascending order is rotated between 1 and `n` times. Find the minimum element in $O(\log n)$ time.

```rust
pub fn find_min(nums: Vec<i32>) -> i32 {
    // TODO: Implement solution
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_min() {
        assert_eq!(find_min(vec![3, 4, 5, 1, 2]), 1);
        assert_eq!(find_min(vec![4, 5, 6, 7, 0, 1, 2]), 0);
    }
}
```

---

## 5. Search in Rotated Sorted Array (LeetCode 33) — Medium
Given the array `nums` after the possible rotation and an integer `target`, return the index of `target` if it is in `nums`, or `-1` if it is not in `nums` in $O(\log n)$ time.

```rust
pub fn search_rotated(nums: Vec<i32>, target: i32) -> i32 {
    // TODO: Implement solution
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_search_rotated() {
        assert_eq!(search_rotated(vec![4, 5, 6, 7, 0, 1, 2], 0), 4);
        assert_eq!(search_rotated(vec![4, 5, 6, 7, 0, 1, 2], 3), -1);
    }
}
```

---

## 6. Time Based Key-Value Store (LeetCode 981) — Medium
Design a time-based key-value data structure that can store multiple values for the same key at different time stamps and retrieve the key's value at a certain timestamp using binary search.

```rust
use std::collections::HashMap;

pub struct TimeMap {
    // TODO: Define fields
}

impl TimeMap {
    pub fn new() -> Self {
        // TODO: Implement solution
        todo!()
    }

    pub fn set(&mut self, key: String, value: String, timestamp: i32) {
        // TODO: Implement solution
        todo!()
    }

    pub fn get(&self, key: String, timestamp: i32) -> String {
        // TODO: Implement solution
        todo!()
    }
}
```

---

## 7. Median of Two Sorted Arrays (LeetCode 4) — Hard
Given two sorted arrays `nums1` and `nums2` of size `m` and `n` respectively, return the median of the two sorted arrays in $O(\log(\min(m, n)))$ runtime complexity.

```rust
pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
    // TODO: Implement solution
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_median_sorted_arrays() {
        assert_eq!(find_median_sorted_arrays(vec![1, 3], vec![2]), 2.0);
        assert_eq!(find_median_sorted_arrays(vec![1, 2], vec![3, 4]), 2.5);
    }
}
```
