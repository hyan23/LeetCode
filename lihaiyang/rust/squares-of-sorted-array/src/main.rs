// main.rs
// Author: hyan23
// 2022.06.17
// https://leetcode.com/problems/squares-of-a-sorted-array/

/*
Given an integer array nums sorted in non-decreasing order, return an array of the squares of each number sorted in non-decreasing order.

 

Example 1:

Input: nums = [-4,-1,0,3,10]
Output: [0,1,9,16,100]
Explanation: After squaring, the array becomes [16,1,0,9,100].
After sorting, it becomes [0,1,9,16,100].
Example 2:

Input: nums = [-7,-3,2,3,11]
Output: [4,9,9,49,121]
 

Constraints:

1 <= nums.length <= 10^4
-10^4 <= nums[i] <= 10^4
nums is sorted in non-decreasing order.
 

Follow up: Squaring each element and sorting the new array is very trivial, could you find an O(n) solution using a different approach?
*/

/*
Runtime: 14 ms, faster than 46.48% of Rust online submissions for Squares of a Sorted Array.
Memory Usage: 2.4 MB, less than 27.11% of Rust online submissions for Squares of a Sorted Array.
*/

struct Solution {

}

impl Solution {
    pub fn sorted_squares(nums: Vec<i32>) -> Vec<i32> {

        let mut a : Vec<i32> = Vec::new();
        let mut b : Vec<i32> = Vec::new();
        for num in nums {
            if num < 0 {
                a.push(num*num);
            } else {
                b.push(num*num);
            }
        }
        
        a.reverse();
        
        let mut result : Vec<i32> = Vec::new();
        let mut i = 0usize;
        let mut j = 0usize;
        
        while i<a.len() && j<b.len() {
            if a[i] <= b[j] {
                result.push(a[i]);
                i += 1;
            } else {
                result.push(b[j]);
                j += 1;
            }
        }
        while i<a.len() {
            result.push(a[i]);
            i += 1;
        }
        while j<b.len() {
            result.push(b[j]);
            j += 1;
        }
        return result;
    }
}

fn main() {
    println!("{:#?}", Solution::sorted_squares(vec![-4,-1,0,3,10]));
    println!("{:#?}", Solution::sorted_squares(vec![-7,-3,2,3,11]));
}
