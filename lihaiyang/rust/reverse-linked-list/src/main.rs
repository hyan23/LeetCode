// main.rs
// Author: hyan23
// Date: 2022.06.24
// https://leetcode.com/problems/reverse-linked-list/

/*
Given the head of a singly linked list, reverse the list, and return the reversed list.

 

Example 1:


Input: head = [1,2,3,4,5]
Output: [5,4,3,2,1]
Example 2:


Input: head = [1,2]
Output: [2,1]
Example 3:

Input: head = []
Output: []
 

Constraints:

The number of nodes in the list is the range [0, 5000].
-5000 <= Node.val <= 5000
 

Follow up: A linked list can be reversed either iteratively or recursively. Could you implement both?


*/

/*
Runtime: 2 ms, faster than 60.37% of Rust online submissions for Reverse Linked List.
Memory Usage: 2.5 MB, less than 28.66% of Rust online submissions for Reverse Linked List.
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
  pub fn reverse_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut cur = head;
    let mut prev: Option<Box<ListNode>> = None;
    while !cur.is_none() {
      let mut a = cur.unwrap();
      cur = a.next;
      a.next = prev;
      prev = Some(a);
    }
    
    return prev;
  }
}

fn main() {
}
