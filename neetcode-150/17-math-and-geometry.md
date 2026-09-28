# NeetCode 150: Category 17 — Math & Geometry

Matrix rotations, spiral traversals, hashing coordinates, and mathematical precision algorithms.

---

## 1. Rotate Image (LeetCode 48) — Medium
You are given an $n \times n$ 2D matrix representing an image, rotate the image by 90 degrees (clockwise) in-place.

```rust
pub fn rotate(matrix: &mut Vec<Vec<i32>>) {
    // TODO: Implement solution
    todo!()
}
```

---

## 2. Spiral Matrix (LeetCode 54) — Medium
Given an $m \times n$ matrix, return all elements of the matrix in spiral order.

```rust
pub fn spiral_order(matrix: Vec<Vec<i32>>) -> Vec<i32> {
    // TODO: Implement solution
    todo!()
}
```

---

## 3. Set Matrix Zeroes (LeetCode 73) — Medium
Given an $m \times n$ integer matrix `matrix`, if an element is 0, set its entire row and column to 0's in-place with $O(1)$ extra space.

```rust
pub fn set_zeroes(matrix: &mut Vec<Vec<i32>>) {
    // TODO: Implement solution
    todo!()
}
```

---

## 4. Happy Number (LeetCode 202) — Easy
Write an algorithm to determine if a number `n` is happy (sum of squares of its digits equals 1) using Floyd's cycle detection.

```rust
pub fn is_happy(n: i32) -> bool {
    // TODO: Implement solution
    todo!()
}
```

---

## 5. Plus One (LeetCode 66) — Easy
You are given a large integer represented as an integer array `digits`. Increment the large integer by one and return the resulting array of digits.

```rust
pub fn plus_one(digits: Vec<i32>) -> Vec<i32> {
    // TODO: Implement solution
    todo!()
}
```

---

## 6. Pow(x, n) (LeetCode 50) — Medium
Implement `my_pow(x, n)`, which calculates $x$ raised to the power $n$ ($x^n$) in $O(\log n)$ using binary exponentiation.

```rust
pub fn my_pow(x: f64, n: i32) -> f64 {
    // TODO: Implement solution
    todo!()
}
```

---

## 7. Multiply Strings (LeetCode 43) — Medium
Given two non-negative integers `num1` and `num2` represented as strings, return the product of `num1` and `num2`, also represented as a string without using direct BigInt conversions.

```rust
pub fn multiply(num1: String, num2: String) -> String {
    // TODO: Implement solution
    todo!()
}
```

---

## 8. Detect Squares (LeetCode 2013) — Medium
Design a data structure that receives points on a 2D plane and counts the number of axis-aligned squares that can be formed.

```rust
use std::collections::HashMap;

pub struct DetectSquares {
    points: HashMap<(i32, i32), i32>,
}

impl DetectSquares {
    pub fn new() -> Self {
        DetectSquares { points: HashMap::new() }
    }

    pub fn add(&mut self, point: Vec<i32>) {
        todo!()
    }

    pub fn count(&self, point: Vec<i32>) -> i32 {
        todo!()
    }
}
```
