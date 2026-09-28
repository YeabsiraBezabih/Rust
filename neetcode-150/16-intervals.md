# NeetCode 150: Category 16 — Intervals

Interval problems involve sorting start/end coordinates and merging or finding intersections in $O(n \log n)$ time.

---

## 1. Insert Interval (LeetCode 57) — Medium
You are given an array of non-overlapping intervals `intervals` sorted in ascending order by start time, insert `newInterval` into `intervals` such that it is still sorted and non-overlapping.

```rust
pub fn insert(intervals: Vec<Vec<i32>>, new_interval: Vec<i32>) -> Vec<Vec<i32>> {
    // TODO: Implement solution
    todo!()
}
```

---

## 2. Merge Intervals (LeetCode 56) — Medium
Given an array of `intervals`, merge all overlapping intervals and return an array of the non-overlapping intervals.

```rust
pub fn merge(intervals: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    // TODO: Implement solution
    todo!()
}
```

---

## 3. Non-overlapping Intervals (LeetCode 435) — Medium
Given an array of intervals `intervals`, return the minimum number of intervals you need to remove to make the rest of the intervals non-overlapping (Greedy Interval Scheduling).

```rust
pub fn erase_overlap_intervals(intervals: Vec<Vec<i32>>) -> i32 {
    // TODO: Implement solution
    todo!()
}
```

---

## 4. Meeting Rooms (LeetCode 252) — Easy
Given an array of meeting time `intervals` where `intervals[i] = [start_i, end_i]`, determine if a person could attend all meetings.

```rust
pub fn can_attend_meetings(intervals: Vec<Vec<i32>>) -> bool {
    // TODO: Implement solution
    todo!()
}
```

---

## 5. Meeting Rooms II (LeetCode 253) — Medium
Given an array of meeting time intervals, return the minimum number of conference rooms required using a Min-Heap of end times or Two Pointers.

```rust
pub fn min_meeting_rooms(intervals: Vec<Vec<i32>>) -> i32 {
    // TODO: Implement solution
    todo!()
}
```

---

## 6. Minimum Interval to Include Each Query (LeetCode 1851) — Hard
You are given a 2D integer array `intervals` and an integer array `queries`. Return an array containing the length of the smallest interval that contains each query using offline sorting with a Min-Heap.

```rust
pub fn min_interval(intervals: Vec<Vec<i32>>, queries: Vec<i32>) -> Vec<i32> {
    // TODO: Implement solution
    todo!()
}
```
