# NeetCode 150: Category 07 — Trees

Tree problems explore recursion, divide-and-conquer, depth-first search (DFS), breadth-first search (BFS), and BST invariants using `Option<Rc<RefCell<TreeNode>>>`.

```rust
// Standard LeetCode Binary Tree Node Definition in Rust
use std::rc::Rc;
use std::cell::RefCell;

#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}
```

---

## 1. Invert Binary Tree (LeetCode 226) — Easy
Given the root of a binary tree, invert the tree, and return its root.

```rust
pub fn invert_tree(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
    // TODO: Implement solution
    todo!()
}
```

---

## 2. Maximum Depth of Binary Tree (LeetCode 104) — Easy
Given the root of a binary tree, return its maximum depth.

```rust
pub fn max_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    // TODO: Implement solution
    todo!()
}
```

---

## 3. Diameter of Binary Tree (LeetCode 543) — Easy
Given the root of a binary tree, return the length of the diameter of the tree (longest path between any two nodes).

```rust
pub fn diameter_of_binary_tree(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    // TODO: Implement solution
    todo!()
}
```

---

## 4. Balanced Binary Tree (LeetCode 110) — Easy
Given a binary tree, determine if it is height-balanced (depth of the two subtrees of every node never differs by more than one).

```rust
pub fn is_balanced(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
    // TODO: Implement solution
    todo!()
}
```

---

## 5. Same Tree (LeetCode 100) — Easy
Given the roots of two binary trees `p` and `q`, write a function to check if they are the same or not.

```rust
pub fn is_same_tree(
    p: Option<Rc<RefCell<TreeNode>>>,
    q: Option<Rc<RefCell<TreeNode>>>,
) -> bool {
    // TODO: Implement solution
    todo!()
}
```

---

## 6. Subtree of Another Tree (LeetCode 572) — Easy
Given the roots of two binary trees `root` and `subRoot`, return `true` if there is a subtree of `root` with the same structure and node values of `subRoot`.

```rust
pub fn is_subtree(
    root: Option<Rc<RefCell<TreeNode>>>,
    sub_root: Option<Rc<RefCell<TreeNode>>>,
) -> bool {
    // TODO: Implement solution
    todo!()
}
```

---

## 7. Lowest Common Ancestor of a Binary Search Tree (LeetCode 235) — Medium
Given a binary search tree (BST), find the lowest common ancestor (LCA) node of two given nodes in the BST.

```rust
pub fn lowest_common_ancestor(
    root: Option<Rc<RefCell<TreeNode>>>,
    p: Option<Rc<RefCell<TreeNode>>>,
    q: Option<Rc<RefCell<TreeNode>>>,
) -> Option<Rc<RefCell<TreeNode>>> {
    // TODO: Implement solution
    todo!()
}
```

---

## 8. Binary Tree Level Order Traversal (LeetCode 102) — Medium
Given the root of a binary tree, return the level order traversal of its nodes' values (using a FIFO queue).

```rust
pub fn level_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
    // TODO: Implement solution
    todo!()
}
```

---

## 9. Binary Tree Right Side View (LeetCode 199) — Medium
Given the root of a binary tree, imagine yourself standing on the right side of it, return the values of the nodes you can see ordered from top to bottom.

```rust
pub fn right_side_view(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
    // TODO: Implement solution
    todo!()
}
```

---

## 10. Count Good Nodes in Binary Tree (LeetCode 1448) — Medium
Given a binary tree `root`, a node $X$ in the tree is named good if in the path from the root to $X$ there are no nodes with a value greater than $X$. Return the number of good nodes.

```rust
pub fn good_nodes(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    // TODO: Implement solution
    todo!()
}
```

---

## 11. Validate Binary Search Tree (LeetCode 98) — Medium
Given the root of a binary tree, determine if it is a valid binary search tree (BST).

```rust
pub fn is_valid_bst(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
    // TODO: Implement solution
    todo!()
}
```

---

## 12. Kth Smallest Element in a BST (LeetCode 230) — Medium
Given the root of a binary search tree, and an integer `k`, return the $k$-th smallest value (1-indexed) of all the values of the nodes in the tree.

```rust
pub fn kth_smallest(root: Option<Rc<RefCell<TreeNode>>>, k: i32) -> i32 {
    // TODO: Implement solution
    todo!()
}
```

---

## 13. Construct Binary Tree from Preorder and Inorder Traversal (LeetCode 105) — Medium
Given two integer arrays `preorder` and `inorder`, construct and return the binary tree.

```rust
pub fn build_tree(preorder: Vec<i32>, inorder: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
    // TODO: Implement solution
    todo!()
}
```

---

## 14. Binary Tree Maximum Path Sum (LeetCode 124) — Hard
A path in a binary tree is a sequence of nodes where each pair of adjacent nodes in the sequence has an edge connecting them. Return the maximum path sum of any non-empty path.

```rust
pub fn max_path_sum(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    // TODO: Implement solution
    todo!()
}
```

---

## 15. Serialize and Deserialize Binary Tree (LeetCode 297) — Hard
Design an algorithm to serialize and deserialize a binary tree.

```rust
pub struct Codec;

impl Codec {
    pub fn serialize(&self, root: Option<Rc<RefCell<TreeNode>>>) -> String {
        todo!()
    }

    pub fn deserialize(&self, data: String) -> Option<Rc<RefCell<TreeNode>>> {
        todo!()
    }
}
```
