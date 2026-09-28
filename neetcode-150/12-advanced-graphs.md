# NeetCode 150: Category 12 — Advanced Graphs

Advanced graph algorithms cover Shortest Paths (Dijkstra, Bellman-Ford), Minimum Spanning Trees (Kruskal, Prim), Eulerian Circuits (Hierholzer), and DAG Topological Ordering.

---

## 1. Reconstruct Itinerary (LeetCode 332) — Hard
Given a list of airline tickets represented by pairs of departure and arrival airports `[from, to]`, reconstruct the itinerary in order using Hierholzer's algorithm for Eulerian paths.

```rust
pub fn find_itinerary(tickets: Vec<Vec<String>>) -> Vec<String> {
    // TODO: Implement solution
    todo!()
}
```

---

## 2. Min Cost to Connect All Points (LeetCode 1584) — Medium
You are given an array `points` representing integer coordinates of some points on a 2D-plane. Return the minimum cost to make all points connected using Prim's or Kruskal's Minimum Spanning Tree algorithm.

```rust
pub fn min_cost_connect_points(points: Vec<Vec<i32>>) -> i32 {
    // TODO: Implement solution
    todo!()
}
```

---

## 3. Network Delay Time (LeetCode 743) — Medium
You are given a network of `n` nodes, labeled from 1 to `n`. You are also given `times`, a list of travel times as directed edges `[u, v, w]`. We will send a signal from a given node `k`. Return the minimum time it takes for all the `n` nodes to receive the signal using **Dijkstra's Algorithm**.

```rust
pub fn network_delay_time(times: Vec<Vec<i32>>, n: i32, k: i32) -> i32 {
    // TODO: Implement solution
    todo!()
}
```

---

## 4. Swim in Rising Water (LeetCode 778) — Hard
You are given an $n \times n$ integer matrix `grid` where each term represents the elevation. Return the least time until you can reach the bottom right square `(n - 1, n - 1)` from `(0, 0)` using a Min-Priority Queue (Dijkstra variant) or Binary Search with BFS.

```rust
pub fn swim_in_water(grid: Vec<Vec<i32>>) -> i32 {
    // TODO: Implement solution
    todo!()
}
```

---

## 5. Alien Dictionary (LeetCode 269) — Hard
There is a new alien language that uses the English alphabet. Given a list of strings `words` from the alien language's dictionary sorted lexicographically by the rules of this new language, derive the order of letters in the language using Topological Sort on a directed character graph.

```rust
pub fn alien_order(words: Vec<String>) -> String {
    // TODO: Implement solution
    todo!()
}
```

---

## 6. Cheapest Flights Within K Stops (LeetCode 787) — Medium
There are `n` cities connected by some number of flights. Find the cheapest price from `src` to `dst` with at most `k` stops using Bellman-Ford or Dijkstra with state `(cost, node, stops)`.

```rust
pub fn find_cheapest_price(
    n: i32,
    flights: Vec<Vec<i32>>,
    src: i32,
    dst: i32,
    k: i32,
) -> i32 {
    // TODO: Implement solution
    todo!()
}
```
