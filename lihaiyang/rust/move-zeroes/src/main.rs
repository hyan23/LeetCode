// main.rs
// Author: hyan23
// 2022.06.17
// leetcode.com/problems/move-zeroes/

/*
Given an integer array nums, move all 0's to the end of it while maintaining the relative order of the non-zero elements.

Note that you must do this in-place without making a copy of the array.

 

Example 1:

Input: nums = [0,1,0,3,12]
Output: [1,3,12,0,0]
Example 2:

Input: nums = [0]
Output: [0]
 

Constraints:

1 <= nums.length <= 104
-231 <= nums[i] <= 231 - 1
 

Follow up: Could you minimize the total number of operations done?
*/

/*
Runtime: 16 ms, faster than 25.53% of Rust online submissions for Move Zeroes.
Memory Usage: 2.3 MB, less than 30.21% of Rust online submissions for Move Zeroes.
*/

struct Solution {

}

impl Solution {
    pub fn move_zeroes(nums: &mut Vec<i32>) {
        let mut zero_count = 0;
        let mut i = 0usize;
        while i < nums.len() {
            if nums[i] == 0 {
                zero_count += 1;
            } else if zero_count > 0 {
                nums[i-zero_count] = nums[i];
                nums[i] = 0;
            }
            i += 1;
        }
    }
}

fn main() {
    let mut a = vec![0,1,0,3,12];
    let mut b = vec![0];
    Solution::move_zeroes(&mut a);
    Solution::move_zeroes(&mut b);
    println!("{:#?}", a);
    println!("{:#?}", b);
}
