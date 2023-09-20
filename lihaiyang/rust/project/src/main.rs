// main.rs
// Author: hyan23
// 2023.09.20
// https://leetcode.com/problems/increasing-triplet-subsequence/?envType=study-plan-v2&envId=leetcode-75

/*
Given an integer array nums, return true if there exists a triple of indices (i, j, k) such that i < j < k and nums[i] < nums[j] < nums[k]. If no such indices exists, return false.



Example 1:

Input: nums = [1,2,3,4,5]
Output: true
Explanation: Any triplet where i < j < k is valid.
Example 2:

Input: nums = [5,4,3,2,1]
Output: false
Explanation: No triplet exists.
Example 3:

Input: nums = [2,1,5,0,4,6]
Output: true
Explanation: The triplet (3, 4, 5) is valid because nums[3] == 0 < nums[4] == 4 < nums[5] == 6.


Constraints:

1 <= nums.length <= 5 * 105
-231 <= nums[i] <= 231 - 1


Follow up: Could you implement a solution that runs in O(n) time complexity and O(1) space complexity?
*/

/*
Runtime
Details
14ms
Beats 36.71%of users with Rust
Memory
Details
5.02MB
Beats 24.68%of users with Rust
*/

use core::num;

struct Solution {}
impl Solution {
    pub fn increasing_triplet(nums: Vec<i32>) -> bool {
        let mut smallest: Option<i32> = None;
        let mut second_smallest: Option<i32> = None;
        let mut candicate: Option<i32> = None;
        for i in 0..nums.len() {
            let t = nums[i];
            if smallest.is_some() && second_smallest.is_some() && t > second_smallest.unwrap() {
                return true;
            }

            if candicate.is_some()
                && t > candicate.unwrap()
                && (second_smallest.is_none() || t <= second_smallest.unwrap())
            {
                smallest = candicate;
                second_smallest = Some(t);
                candicate = None;
            }
            if smallest.is_none() || t < smallest.unwrap() {
                candicate = Some(t);
            }
            if smallest.is_some()
                && second_smallest.is_some()
                && t > smallest.unwrap()
                && t < second_smallest.unwrap()
            {
                second_smallest = Some(t);
            }
        }
        false
    }
}

fn main() {}
