# NeetCode 150: Category 14 — 2-D Dynamic Programming

Multi-dimensional DP tracks state transitions across grids, string comparisons (LCS, Edit Distance), and multiple resource constraints.

---

## 1. Unique Paths (LeetCode 62) — Medium
There is a robot on an $m \times n$ grid. The robot can only move either down or right at any point in time. How many possible unique paths are there to reach the bottom-right corner?

```rust
pub fn unique_paths(m: i32, n: i32) -> i32 {
    // TODO: Implement solution
    todo!()
}
```

---

## 2. Longest Common Subsequence (LeetCode 1143) — Medium
Given two strings `text1` and `text2`, return the length of their longest common subsequence.

```rust
pub fn longest_common_subsequence(text1: String, text2: String) -> i32 {
    // TODO: Implement solution
    todo!()
}
```

---

## 3. Best Time to Buy and Sell Stock with Cooldown (LeetCode 309) — Medium
You are given an array `prices` where `prices[i]` is the price of a given stock on the $i$-th day. After you sell your stock, you cannot buy stock on the next day (cooldown 1 day). Maximize profit.

```rust
pub fn max_profit_cooldown(prices: Vec<i32>) -> i32 {
    // TODO: Implement solution
    todo!()
}
```

---

## 4. Coin Change II (LeetCode 518) — Medium
You are given an integer array `coins` and an integer `amount`. Return the number of combinations that make up that amount (Unbounded Knapsack DP).

```rust
pub fn change(amount: i32, coins: Vec<i32>) -> i32 {
    // TODO: Implement solution
    todo!()
}
```

---

## 5. Target Sum (LeetCode 494) — Medium
You are given an integer array `nums` and an integer `target`. You want to build an expression out of nums by adding one of the symbols `'+'` and `'-'` before each integer. Return the number of different expressions that evaluate to `target`.

```rust
pub fn find_target_sum_ways(nums: Vec<i32>, target: i32) -> i32 {
    // TODO: Implement solution
    todo!()
}
```

---

## 6. Interleaving String (LeetCode 97) — Medium
Given strings `s1`, `s2`, and `s3`, find whether `s3` is formed by an interleaving of `s1` and `s2`.

```rust
pub fn is_interleave(s1: String, s2: String, s3: String) -> bool {
    // TODO: Implement solution
    todo!()
}
```

---

## 7. Longest Increasing Path in a Matrix (LeetCode 329) — Hard
Given an $m \times n$ integers matrix, return the length of the longest increasing path in matrix using DFS with memoization.

```rust
pub fn longest_increasing_path(matrix: Vec<Vec<i32>>) -> i32 {
    // TODO: Implement solution
    todo!()
}
```

---

## 8. Distinct Subsequences (LeetCode 115) — Hard
Given two strings `s` and `t`, return the number of distinct subsequences of `s` which equals `t`.

```rust
pub fn num_distinct(s: String, t: String) -> i32 {
    // TODO: Implement solution
    todo!()
}
```

---

## 9. Edit Distance (LeetCode 72) — Medium
Given two strings `word1` and `word2`, return the minimum number of operations required to convert `word1` to `word2` (Insert, Delete, Replace).

```rust
pub fn min_distance(word1: String, word2: String) -> i32 {
    // TODO: Implement solution
    todo!()
}
```

---

## 10. Burst Balloons (LeetCode 312) — Hard
You are given `n` balloons, indexed from 0 to `n - 1`. If you burst the $i$-th balloon, you will get `nums[i - 1] * nums[i] * nums[i + 1]` coins. Return the maximum coins you can collect using Interval DP.

```rust
pub fn max_coins(nums: Vec<i32>) -> i32 {
    // TODO: Implement solution
    todo!()
}
```

---

## 11. Regular Expression Matching (LeetCode 10) — Hard
Given an input string `s` and a pattern `p`, implement regular expression matching with support for `'.'` and `'*'`.

```rust
pub fn is_match(s: String, p: String) -> bool {
    // TODO: Implement solution
    todo!()
}
```
