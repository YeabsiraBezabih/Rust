# NeetCode 150: Category 18 — Bit Manipulation

Bitwise arithmetic (`&`, `|`, `^`, `!`, `<<`, `>>`) enables constant-time operations at the hardware register level.

---

## 1. Single Number (LeetCode 136) — Easy
Given a non-empty array of integers `nums`, every element appears twice except for one. Find that single one using XOR ($a \oplus a = 0$ and $a \oplus 0 = a$) in $O(n)$ time and $O(1)$ space.

```rust
pub fn single_number(nums: Vec<i32>) -> i32 {
    // TODO: Implement solution
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_single_number() {
        assert_eq!(single_number(vec![2, 2, 1]), 1);
        assert_eq!(single_number(vec![4, 1, 2, 1, 2]), 4);
    }
}
```

---

## 2. Number of 1 Bits (LeetCode 191) — Easy
Write a function that takes the binary representation of a positive integer and returns the number of set bits (Hamming weight) using Brian Kernighan's bit-trick ($n \ \& \ (n - 1)$).

```rust
pub fn hamming_weight(n: u32) -> i32 {
    // TODO: Implement solution
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hamming_weight() {
        assert_eq!(hamming_weight(11), 3); // 11 is 1011 in binary
        assert_eq!(hamming_weight(128), 1);
    }
}
```

---

## 3. Counting Bits (LeetCode 338) — Easy
Given an integer `n`, return an array `ans` of length `n + 1` such that for each `i` ($0 \le i \le n$), `ans[i]` is the number of `1`'s in the binary representation of `i` in $O(n)$ time.

```rust
pub fn count_bits(n: i32) -> Vec<i32> {
    // TODO: Implement solution
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_bits() {
        assert_eq!(count_bits(2), vec![0, 1, 1]);
        assert_eq!(count_bits(5), vec![0, 1, 1, 2, 1, 2]);
    }
}
```

---

## 4. Reverse Bits (LeetCode 190) — Easy
Reverse bits of a given 32 bits unsigned integer.

```rust
pub fn reverse_bits(x: u32) -> u32 {
    // TODO: Implement solution
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reverse_bits() {
        assert_eq!(reverse_bits(43261596), 964176192);
    }
}
```

---

## 5. Missing Number (LeetCode 268) — Easy
Given an array `nums` containing `n` distinct numbers in the range `[0, n]`, return the only number in the range that is missing from the array using XOR or Gauss sum.

```rust
pub fn missing_number(nums: Vec<i32>) -> i32 {
    // TODO: Implement solution
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_missing_number() {
        assert_eq!(missing_number(vec![3, 0, 1]), 2);
        assert_eq!(missing_number(vec![0, 1]), 2);
    }
}
```

---

## 6. Sum of Two Integers (LeetCode 371) — Medium
Given two integers `a` and `b`, return the sum of the two integers without using the operators `+` and `-` using bitwise carry addition.

```rust
pub fn get_sum(a: i32, b: i32) -> i32 {
    // TODO: Implement solution
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_sum() {
        assert_eq!(get_sum(1, 2), 3);
        assert_eq!(get_sum(2, 3), 5);
    }
}
```

---

## 7. Reverse Integer (LeetCode 7) — Medium
Given a signed 32-bit integer `x`, return `x` with its digits reversed. If reversing `x` causes the value to go outside the signed 32-bit integer range $[-2^{31}, 2^{31} - 1]$, then return 0.

```rust
pub fn reverse(x: i32) -> i32 {
    // TODO: Implement solution
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reverse() {
        assert_eq!(reverse(123), 321);
        assert_eq!(reverse(-123), -321);
        assert_eq!(reverse(120), 21);
    }
}
```
