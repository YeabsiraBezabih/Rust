# NeetCode 150: Category 09 — Heap / Priority Queue

Binary Heaps (`std::collections::BinaryHeap` and `std::cmp::Reverse`) provide $O(\log n)$ extraction of minimum or maximum priorities.

---

## 1. Kth Largest Element in a Stream (LeetCode 703) — Easy
Design a class to find the $k$-th largest element in a stream.

```rust
use std::cmp::Reverse;
use std::collections::BinaryHeap;

pub struct KthLargest {
    min_heap: BinaryHeap<Reverse<i32>>,
    k: usize,
}

impl KthLargest {
    pub fn new(k: i32, nums: Vec<i32>) -> Self {
        todo!()
    }

    pub fn add(&mut self, val: i32) -> i32 {
        todo!()
    }
}
```

---

## 2. Last Stone Weight (LeetCode 1046) — Easy
You are given an array of integers `stones` where `stones[i]` is the weight of the $i$-th stone. Each turn, choose the two heaviest stones and smash them. Return the weight of the last remaining stone.

```rust
use std::collections::BinaryHeap;

pub fn last_stone_weight(stones: Vec<i32>) -> i32 {
    // TODO: Implement solution
    todo!()
}
```

---

## 3. K Closest Points to Origin (LeetCode 973) — Medium
Given an array of `points` where `points[i] = [xi, yi]` represents a point on the X-Y plane and an integer `k`, return the `k` closest points to the origin `(0, 0)`.

```rust
use std::collections::BinaryHeap;

pub fn k_closest(points: Vec<Vec<i32>>, k: i32) -> Vec<Vec<i32>> {
    // TODO: Implement solution
    todo!()
}
```

---

## 4. Kth Largest Element in an Array (LeetCode 215) — Medium
Given an integer array `nums` and an integer `k`, return the $k$-th largest element in the array in $O(n \log k)$ or $O(n)$ average quickselect.

```rust
pub fn find_kth_largest(nums: Vec<i32>, k: i32) -> i32 {
    // TODO: Implement solution
    todo!()
}
```

---

## 5. Task Scheduler (LeetCode 621) — Medium
You are given an array of CPU `tasks` represented by letters A to Z and a cooling interval `n`. Return the least number of CPU intervals that the CPU will take to finish all the given tasks.

```rust
pub fn least_interval(tasks: Vec<char>, n: i32) -> i32 {
    // TODO: Implement solution
    todo!()
}
```

---

## 6. Design Twitter (LeetCode 355) — Medium
Design a simplified version of Twitter where users can post tweets, follow/unfollow another user, and see the 10 most recent tweets in the user's news feed using a multi-way merge heap.

```rust
pub struct Twitter {
    // TODO: Define fields
}

impl Twitter {
    pub fn new() -> Self {
        todo!()
    }

    pub fn post_tweet(&mut self, user_id: i32, tweet_id: i32) {
        todo!()
    }

    pub fn get_news_feed(&self, user_id: i32) -> Vec<i32> {
        todo!()
    }

    pub fn follow(&mut self, follower_id: i32, followee_id: i32) {
        todo!()
    }

    pub fn unfollow(&mut self, follower_id: i32, followee_id: i32) {
        todo!()
    }
}
```

---

## 7. Find Median from Data Stream (LeetCode 295) — Hard
The median is the middle value in an ordered integer list. Implement the `MedianFinder` class with two balanced heaps (Max-Heap for lower half, Min-Heap for upper half).

```rust
use std::cmp::Reverse;
use std::collections::BinaryHeap;

pub struct MedianFinder {
    small: BinaryHeap<i32>,          // Max-heap
    large: BinaryHeap<Reverse<i32>>, // Min-heap
}

impl MedianFinder {
    pub fn new() -> Self {
        MedianFinder {
            small: BinaryHeap::new(),
            large: BinaryHeap::new(),
        }
    }

    pub fn add_num(&mut self, num: i32) {
        todo!()
    }

    pub fn find_median(&self) -> f64 {
        todo!()
    }
}
```
