// main.rs
// Author: hyan23
// Date: 2022.06.24
// https://leetcode.com/problems/merge-two-sorted-lists/

/*
You are given the heads of two sorted linked lists list1 and list2.

Merge the two lists in a one sorted list. The list should be made by splicing together the nodes of the first two lists.

Return the head of the merged linked list.

 

Example 1:


Input: list1 = [1,2,4], list2 = [1,3,4]
Output: [1,1,2,3,4,4]
Example 2:

Input: list1 = [], list2 = []
Output: []
Example 3:

Input: list1 = [], list2 = [0]
Output: [0]
 

Constraints:

The number of nodes in both lists is in the range [0, 50].
-100 <= Node.val <= 100
Both list1 and list2 are sorted in non-decreasing order.
*/

/*
Runtime: 2 ms, faster than 58.90% of Rust online submissions for Merge Two Sorted Lists.
Memory Usage: 2.2 MB, less than 27.54% of Rust online submissions for Merge Two Sorted Lists.
*/

struct Solution {
}

// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
  pub val: i32,
  pub next: Option<Box<ListNode>>
}

impl ListNode {
  #[inline]
  fn new(val: i32) -> Self {
    ListNode {
      next: None,
      val
    }
  }
}

impl Solution {
    pub fn merge_two_lists(list1: Option<Box<ListNode>>, list2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        if list1.is_none() {
          return list2;
        }
        if list2.is_none() {
          return list1;
        }
        
        let mut l1 = list1.unwrap();
        let mut l2 = list2.unwrap();
        if l1.val < l2.val {
          l1.next = Solution::merge_two_lists(l1.next, Some(l2));
          return Some(l1);
        } else {
          l2.next = Solution::merge_two_lists(Some(l1), l2.next);
          return Some(l2);
        }
    }
}

fn main() {
}
