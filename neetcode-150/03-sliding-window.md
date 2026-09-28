# NeetCode 150: Category 03 — Sliding Window

Sliding window techniques track dynamic contiguous subarrays or substrings to achieve $O(n)$ linear runtime.

---

## 1. Best Time to Buy and Sell Stock (LeetCode 121) — Easy
You are given an array `prices` where `prices[i]` is the price of a given stock on the $i$-th day. You want to maximize your profit by choosing a single day to buy one stock and choosing a different day in the future to sell that stock.

```rust
pub fn max_profit(prices: Vec<i32>) -> i32 {
    // TODO: Implement solution
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_max_profit() {
        assert_eq!(max_profit(vec![7, 1, 5, 3, 6, 4]), 5);
        assert_eq!(max_profit(vec![7, 6, 4, 3, 1]), 0);
    }
}
```

---

## 2. Longest Substring Without Repeating Characters (LeetCode 3) — Medium
Given a string `s`, find the length of the longest substring without repeating characters.

```rust
use std::collections::HashSet;

pub fn length_of_longest_substring(s: String) -> i32 {
    // TODO: Implement solution
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_length_of_longest_substring() {
        assert_eq!(length_of_longest_substring("abcabcbb".into()), 3);
        assert_eq!(length_of_longest_substring("bbbbb".into()), 1);
        assert_eq!(length_of_longest_substring("pwwkew".into()), 3);
    }
}
```

---

## 3. Longest Repeating Character Replacement (LeetCode 424) — Medium
You are given a string `s` and an integer `k`. You can choose any character of the string and change it to any other uppercase English character at most `k` times. Return the length of the longest substring containing the same letter you can get after performing at most `k` operations.

```rust
pub fn character_replacement(s: String, k: i32) -> i32 {
    // TODO: Implement solution
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_character_replacement() {
        assert_eq!(character_replacement("ABAB".into(), 2), 4);
        assert_eq!(character_replacement("AABABBA".into(), 1), 4);
    }
}
```

---

## 4. Permutation in String (LeetCode 567) — Medium
Given two strings `s1` and `s2`, return `true` if `s2` contains a permutation of `s1`, or `false` otherwise.

```rust
pub fn check_inclusion(s1: String, s2: String) -> bool {
    // TODO: Implement solution
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_check_inclusion() {
        assert_eq!(check_inclusion("ab".into(), "eidbaooo".into()), true);
        assert_eq!(check_inclusion("ab".into(), "eidboaoo".into()), false);
    }
}
```

---

## 5. Minimum Window Substring (LeetCode 76) — Hard
Given two strings `s` and `t` of lengths `m` and `n` respectively, return the minimum window substring of `s` such that every character in `t` (including duplicates) is included in the window. If there is no such substring, return the empty string `""`.

```rust
pub fn min_window(s: String, t: String) -> String {
    // TODO: Implement solution
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_min_window() {
        assert_eq!(min_window("ADOBECODEBANC".into(), "ABC".into()), "BANC");
    }
}
```

---

## 6. Sliding Window Maximum (LeetCode 239) — Hard
You are given an array of integers `nums`, there is a sliding window of size `k` which is moving from the very left of the array to the very right. Return the max sliding window in $O(n)$ time using a monotonic deque.

```rust
use std::collections::VecDeque;

pub fn max_sliding_window(nums: Vec<i32>, k: i32) -> Vec<i32> {
    // TODO: Implement solution
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_max_sliding_window() {
        assert_eq!(
            max_sliding_window(vec![1, 3, -1, -3, 5, 3, 6, 7], 3),
            vec![3, 3, 5, 5, 6, 7]
        );
    }
}
```
