// main.rs
// Author: hyan23
// 2023.09.21
// https://leetcode.com/problems/maximum-average-subarray-i/?envType=study-plan-v2&envId=leetcode-75

/*
You are given an integer array nums consisting of n elements, and an integer k.

Find a contiguous subarray whose length is equal to k that has the maximum average value and return this value. Any answer with a calculation error less than 10-5 will be accepted.



Example 1:

Input: nums = [1,12,-5,-6,50,3], k = 4
Output: 12.75000
Explanation: Maximum average is (12 - 5 - 6 + 50) / 4 = 51 / 4 = 12.75
Example 2:

Input: nums = [5], k = 1
Output: 5.00000


Constraints:

n == nums.length
1 <= k <= n <= 105
-104 <= nums[i] <= 104
*/

/*
Runtime
Details
16ms
Beats 89.70%of users with Rust
Memory
Details
3.08MB
Beats 29.09%of users with Rust
*/

struct Solution {}
impl Solution {
    pub fn find_max_average(nums: Vec<i32>, k: i32) -> f64 {
        let mut sum = 0;
        let mut avg = 0.0;
        let mut max_avg = 0.0;
        for i in 0..k as usize {
            sum += nums[i];
        }
        avg = sum as f64 / k as f64;
        max_avg = avg;

        for i in k as usize..nums.len() {
            sum -= nums[i - k as usize];
            sum += nums[i];
            avg = sum as f64 / k as f64;
            if avg > max_avg {
                max_avg = avg;
            }
        }

        return max_avg;
    }
}

fn main() {}
