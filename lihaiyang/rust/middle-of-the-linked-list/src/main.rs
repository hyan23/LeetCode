// main.rs
// Author: hyan23
// 2022.06.18
// https://leetcode.com/problems/middle-of-the-linked-list/

/*
Given the head of a singly linked list, return the middle node of the linked list.

If there are two middle nodes, return the second middle node.

 

Example 1:


Input: head = [1,2,3,4,5]
Output: [3,4,5]
Explanation: The middle node of the list is node 3.
Example 2:


Input: head = [1,2,3,4,5,6]
Output: [4,5,6]
Explanation: Since the list has two middle nodes with values 3 and 4, we return the second one.
 

Constraints:

The number of nodes in the list is in the range [1, 100].
1 <= Node.val <= 100
*/

/*
Runtime: 0 ms, faster than 100.00% of Rust online submissions for Middle of the Linked List.
Memory Usage: 2.2 MB, less than 32.67% of Rust online submissions for Middle of the Linked List.
*/

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

struct Solution {
}

impl Solution {
    pub fn middle_node(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut cur = head;
        let mut mid = cur.clone();

        let mut list_len = 0;
        while !cur.is_none() {
          cur = cur.unwrap().next;
          list_len += 1;
          if list_len % 2 == 0 {
            mid = mid.unwrap().next;
          }
        }

        return mid;
    }
}

fn main() {
}
