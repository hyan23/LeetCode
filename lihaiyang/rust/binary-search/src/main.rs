// main.rs
// Author: hyan23
// 2022.06.13
// https://leetcode.com/problems/binary-search/

/*
Given an array of integers nums which is sorted in ascending order, and an integer target, write a function to search target in nums. If target exists, then return its index. Otherwise, return -1.

You must write an algorithm with O(log n) runtime complexity.

 

Example 1:

Input: nums = [-1,0,3,5,9,12], target = 9
Output: 4
Explanation: 9 exists in nums and its index is 4
Example 2:

Input: nums = [-1,0,3,5,9,12], target = 2
Output: -1
Explanation: 2 does not exist in nums so return -1
 

Constraints:

1 <= nums.length <= 10^4
-10^4 < nums[i], target < 10^4
All the integers in nums are unique.
nums is sorted in ascending order.
*/

/*
Runtime: 5 ms, faster than 64.61% of Rust online submissions for Binary Search.
Memory Usage: 2.2 MB, less than 80.04% of Rust online submissions for Binary Search.
*/

struct Solution {
}

impl Solution {
    
    fn max<T>(a: T, b: T) -> T 
    where T : Ord
    {
        if a >= b {
            a
        } else {
            b
        }
    }
    
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        
        if nums.len() == 0 {
            return -1;
        }
        
        let mut left = 0i32;
        let mut right = (nums.len() - 1) as i32;
        
        loop {
            let mid = (left + right) / 2;
            if left == right {
                break if nums[left as usize] == target {
                    left
                } else {
                    -1
                }
            }
            if target > nums[mid as usize] {
                left = mid + 1;
            } else if target < nums[mid as usize] {
                right = Solution::max(left, mid - 1);
            } else {
                break mid
            }
        }
    }
}

fn main() {
    println!("{}", Solution::search(vec![-1,0,3,5,9,12], 9));
    println!("{}", Solution::search(vec![-1,0,3,5,9,12], 2));
    println!("{}", Solution::search(vec![2,5], 0));
}
