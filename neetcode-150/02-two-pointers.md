# NeetCode 150: Category 02 — Two Pointers

Two pointer techniques optimize search space and avoid quadratic loops by moving indices inward or outward based on sorted invariants.

---

## 1. Valid Palindrome (LeetCode 125) — Easy
A phrase is a palindrome if, after converting all uppercase letters into lowercase letters and removing all non-alphanumeric characters, it reads the same forward and backward.

### Example 1
- **Input**: `s = "A man, a plan, a canal: Panama"`
- **Output**: `true`

```rust
pub fn is_palindrome(s: String) -> bool {
    // TODO: Implement solution
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_palindrome() {
        assert_eq!(is_palindrome("A man, a plan, a canal: Panama".into()), true);
        assert_eq!(is_palindrome("race a car".into()), false);
    }
}
```

---

## 2. Two Sum II - Input Array Is Sorted (LeetCode 167) — Medium
Given a 1-indexed array of integers `numbers` that is already sorted in non-decreasing order, find two numbers such that they add up to a specific `target` number. Return the indices `[index1, index2]` as 1-indexed. Run in $O(n)$ time and $O(1)$ extra space.

### Example 1
- **Input**: `numbers = [2,7,11,15]`, `target = 9`
- **Output**: `[1,2]`

```rust
pub fn two_sum(numbers: Vec<i32>, target: i32) -> Vec<i32> {
    // TODO: Implement solution
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_two_sum() {
        assert_eq!(two_sum(vec![2, 7, 11, 15], 9), vec![1, 2]);
    }
}
```

---

## 3. 3Sum (LeetCode 15) — Medium
Given an integer array `nums`, return all the triplets `[nums[i], nums[j], nums[k]]` such that `i != j`, `i != k`, and `j != k`, and `nums[i] + nums[j] + nums[k] == 0`. The solution set must not contain duplicate triplets.

### Example 1
- **Input**: `nums = [-1,0,1,2,-1,-4]`
- **Output**: `[[-1,-1,2],[-1,0,1]]`

```rust
pub fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
    // TODO: Implement solution
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_three_sum() {
        let res = three_sum(vec![-1, 0, 1, 2, -1, -4]);
        assert_eq!(res.len(), 2);
    }
}
```

---

## 4. Container With Most Water (LeetCode 11) — Medium
You are given an integer array `height` of length `n`. Find two lines that together with the x-axis form a container, such that the container contains the most water. Return the maximum amount of water a container can store.

### Example 1
- **Input**: `height = [1,8,6,2,5,4,8,3,7]`
- **Output**: `49`

```rust
pub fn max_area(height: Vec<i32>) -> i32 {
    // TODO: Implement solution
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_max_area() {
        assert_eq!(max_area(vec![1, 8, 6, 2, 5, 4, 8, 3, 7]), 49);
    }
}
```

---

## 5. Trapping Rain Water (LeetCode 42) — Hard
Given `n` non-negative integers representing an elevation map where the width of each bar is 1, compute how much water it can trap after raining in $O(n)$ time and $O(1)$ space.

### Example 1
- **Input**: `height = [0,1,0,2,1,0,1,3,2,1,2,1]`
- **Output**: `6`

```rust
pub fn trap(height: Vec<i32>) -> i32 {
    // TODO: Implement solution
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_trap() {
        assert_eq!(trap(vec![0, 1, 0, 2, 1, 0, 1, 3, 2, 1, 2, 1]), 6);
    }
}
```
