# NeetCode 150: Category 11 — Graphs

Graph algorithms cover 2D grid explorations (flood fill), Adjacency Lists, Breadth-First Search (BFS), Depth-First Search (DFS), Connected Components, and Topological Sorts (Kahn's algorithm / DFS 3-coloring).

---

## 1. Number of Islands (LeetCode 200) — Medium
Given an $m \times n$ 2D binary grid `grid` which represents a map of `'1'`s (land) and `'0'`s (water), return the number of islands.

```rust
pub fn num_islands(grid: Vec<Vec<char>>) -> i32 {
    // TODO: Implement solution (DFS / BFS flood fill)
    todo!()
}
```

---

## 2. Max Area of Island (LeetCode 695) — Medium
Given an $m \times n$ binary matrix `grid`, return the maximum area of an island in `grid`.

```rust
pub fn max_area_of_island(grid: Vec<Vec<i32>>) -> i32 {
    // TODO: Implement solution
    todo!()
}
```

---

## 3. Clone Graph (LeetCode 133) — Medium
Given a reference of a node in a connected undirected graph, return a deep copy (clone) of the graph.

---

## 4. Walls and Gates (LeetCode 286) — Medium
Fill each empty room in an $m \times n$ grid with the distance to its nearest gate using multi-source BFS.

```rust
pub fn walls_and_gates(rooms: &mut Vec<Vec<i32>>) {
    // TODO: Implement solution
    todo!()
}
```

---

## 5. Rotting Oranges (LeetCode 994) — Medium
You are given an $m \times n$ grid where each cell is empty (0), fresh orange (1), or rotten orange (2). Return the minimum number of minutes that must elapse until no cell has a fresh orange using multi-source BFS.

```rust
pub fn oranges_rotting(grid: Vec<Vec<i32>>) -> i32 {
    // TODO: Implement solution
    todo!()
}
```

---

## 6. Pacific Atlantic Water Flow (LeetCode 417) — Medium
Return a 2D list of grid coordinates where water can flow to both the Pacific and Atlantic oceans.

```rust
pub fn pacific_atlantic(heights: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    // TODO: Implement solution
    todo!()
}
```

---

## 7. Surrounded Regions (LeetCode 130) — Medium
Given an $m \times n$ matrix `board` containing `'X'` and `'O'`, capture all regions that are 4-directionally surrounded by `'X'`.

```rust
pub fn solve(board: &mut Vec<Vec<char>>) {
    // TODO: Implement solution
    todo!()
}
```

---

## 8. Course Schedule (LeetCode 207) — Medium
There are a total of `numCourses` you have to take, labeled from 0 to `numCourses - 1`. Some courses have prerequisites. Return `true` if you can finish all courses (Cycle Detection in Directed Graph).

```rust
pub fn can_finish(num_courses: i32, prerequisites: Vec<Vec<i32>>) -> bool {
    // TODO: Implement solution
    todo!()
}
```

---

## 9. Course Schedule II (LeetCode 210) — Medium
Return the ordering of courses you should take to finish all courses using **Topological Sort**.

```rust
pub fn find_order(num_courses: i32, prerequisites: Vec<Vec<i32>>) -> Vec<i32> {
    // TODO: Implement solution
    todo!()
}
```

---

## 10. Graph Valid Tree (LeetCode 261) — Medium
Given `n` nodes labeled from `0` to `n - 1` and a list of undirected edges, determine if these edges form a valid single connected acyclic tree.

```rust
pub fn valid_tree(n: i32, edges: Vec<Vec<i32>>) -> bool {
    // TODO: Implement solution
    todo!()
}
```

---

## 11. Number of Connected Components in an Undirected Graph (LeetCode 323) — Medium
Given `n` nodes and `edges`, return the number of connected components in the graph using Union-Find or BFS/DFS.

```rust
pub fn count_components(n: i32, edges: Vec<Vec<i32>>) -> i32 {
    // TODO: Implement solution
    todo!()
}
```

---

## 12. Redundant Connection (LeetCode 684) — Medium
Given an undirected graph that started as a tree with `n` nodes and had one additional edge added, return an edge that can be removed so that the resulting graph is a tree using Disjoint Set Union (Union-Find).

```rust
pub fn find_redundant_connection(edges: Vec<Vec<i32>>) -> Vec<i32> {
    // TODO: Implement solution
    todo!()
}
```

---

## 13. Word Ladder (LeetCode 127) — Hard
Given two words, `beginWord` and `endWord`, and a dictionary `wordList`, return the number of words in the shortest transformation sequence from `beginWord` to `endWord` using BFS.

```rust
pub fn ladder_length(begin_word: String, end_word: String, word_list: Vec<String>) -> i32 {
    // TODO: Implement solution
    todo!()
}
```
