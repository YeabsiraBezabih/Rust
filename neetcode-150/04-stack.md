# NeetCode 150: Category 04 — Stack

Stack (LIFO) structures solve matching, parsing, monotonic range tracking, and postfix expression evaluation.

---

## 1. Valid Parentheses (LeetCode 20) — Easy
Given a string `s` containing just the characters `'('`, `')'`, `'{'`, `'}'`, `'['` and `']'`, determine if the input string is valid.

```rust
pub fn is_valid(s: String) -> bool {
    // TODO: Implement solution
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_valid() {
        assert_eq!(is_valid("()[]{}".into()), true);
        assert_eq!(is_valid("(]".into()), false);
    }
}
```

---

## 2. Min Stack (LeetCode 155) — Medium
Design a stack that supports push, pop, top, and retrieving the minimum element in constant $O(1)$ time.

```rust
pub struct MinStack {
    // TODO: Define fields
}

impl MinStack {
    pub fn new() -> Self {
        // TODO: Implement solution
        todo!()
    }

    pub fn push(&mut self, val: i32) {
        // TODO: Implement solution
        todo!()
    }

    pub fn pop(&mut self) {
        // TODO: Implement solution
        todo!()
    }

    pub fn top(&self) -> i32 {
        // TODO: Implement solution
        todo!()
    }

    pub fn get_min(&self) -> i32 {
        // TODO: Implement solution
        todo!()
    }
}
```

---

## 3. Evaluate Reverse Polish Notation (LeetCode 150) — Medium
Evaluate the value of an arithmetic expression in Reverse Polish Notation. Valid operators are `+`, `-`, `*`, and `/`.

```rust
pub fn eval_rpn(tokens: Vec<String>) -> i32 {
    // TODO: Implement solution
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_eval_rpn() {
        let tokens: Vec<String> = vec!["2","1","+","3","*"].into_iter().map(String::from).collect();
        assert_eq!(eval_rpn(tokens), 9);
    }
}
```

---

## 4. Generate Parentheses (LeetCode 22) — Medium
Given `n` pairs of parentheses, write a function to generate all combinations of well-formed parentheses.

```rust
pub fn generate_parenthesis(n: i32) -> Vec<String> {
    // TODO: Implement solution
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_parenthesis() {
        let res = generate_parenthesis(3);
        assert_eq!(res.len(), 5);
    }
}
```

---

## 5. Daily Temperatures (LeetCode 739) — Medium
Given an array of integers `temperatures` represents the daily temperatures, return an array `answer` such that `answer[i]` is the number of days you have to wait after the $i$-th day to get a warmer temperature. Use a monotonic stack in $O(n)$ time.

```rust
pub fn daily_temperatures(temperatures: Vec<i32>) -> Vec<i32> {
    // TODO: Implement solution
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_daily_temperatures() {
        assert_eq!(
            daily_temperatures(vec![73, 74, 75, 71, 69, 72, 76, 73]),
            vec![1, 1, 4, 2, 1, 1, 0, 0]
        );
    }
}
```

---

## 6. Car Fleet (LeetCode 853) — Medium
There are `n` cars at given miles away from the starting mile 0, traveling to a destination mile `target`. Return the number of non-empty car fleets that will arrive at the destination.

```rust
pub fn car_fleet(target: i32, position: Vec<i32>, speed: Vec<i32>) -> i32 {
    // TODO: Implement solution
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_car_fleet() {
        assert_eq!(car_fleet(12, vec![10, 8, 0, 5, 3], vec![2, 4, 1, 1, 3]), 3);
    }
}
```

---

## 7. Largest Rectangle in Histogram (LeetCode 84) — Hard
Given an array of integers `heights` representing the histogram's bar height where the width of each bar is 1, return the area of the largest rectangle in the histogram in $O(n)$ time using a monotonic stack.

```rust
pub fn largest_rectangle_area(heights: Vec<i32>) -> i32 {
    // TODO: Implement solution
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_largest_rectangle_area() {
        assert_eq!(largest_rectangle_area(vec![2, 1, 5, 6, 2, 3]), 10);
    }
}
```
