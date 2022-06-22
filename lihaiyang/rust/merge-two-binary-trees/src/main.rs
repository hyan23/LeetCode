// main.rs
// Author: hyan23
// Date: 2022.06.21
// https://leetcode.com/problems/merge-two-binary-trees/

/*
You are given two binary trees root1 and root2.

Imagine that when you put one of them to cover the other, some nodes of the two trees are overlapped while the others are not. You need to merge the two trees into a new binary tree. The merge rule is that if two nodes overlap, then sum node values up as the new value of the merged node. Otherwise, the NOT null node will be used as the node of the new tree.

Return the merged tree.

Note: The merging process must start from the root nodes of both trees.

 

Example 1:


Input: root1 = [1,3,2,5], root2 = [2,1,3,null,4,null,7]
Output: [3,4,5,5,4,null,7]
Example 2:

Input: root1 = [1], root2 = [1,2]
Output: [2,2]
 

Constraints:

The number of nodes in both trees is in the range [0, 2000].
-10^4 <= Node.val <= 10^4
*/

/*
Runtime: 10 ms, faster than 26.32% of Rust online submissions for Merge Two Binary Trees.
Memory Usage: 2.3 MB, less than 31.58% of Rust online submissions for Merge Two Binary Trees.
*/

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
      right: None
    }
  }
}

struct Solution {
}

impl Solution {
    pub fn merge_trees(root1: Option<Rc<RefCell<TreeNode>>>, root2: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
      if root1.is_none() {
        return root2;
      }
      if root2.is_none() {
        return root1;
      }
      
      let r1 = root1.unwrap();
      let r2 = root2.unwrap();
      let mut n1 = r1.borrow_mut();
      let mut n2 = r2.borrow_mut();
      let mut new_node = TreeNode::new(n1.val + n2.val);
      new_node.left = Solution::merge_trees(
        match &n1.left{None=>None,Some(l)=>Some(l.clone())}, 
        match &n2.left{None=>None,Some(l)=>Some(l.clone())});
      new_node.right = Solution::merge_trees(
        match &n1.right{None=>None,Some(r)=>Some(r.clone())}, 
        match &n2.right{None=>None,Some(r)=>Some(r.clone())});
      return Some(Rc::new(RefCell::new(new_node)));
    }
}

fn main() {
}
