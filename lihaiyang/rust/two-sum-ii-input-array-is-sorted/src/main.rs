// main.rs
// Author: hyan23
// 2022.06.17
// https://leetcode.com/problems/two-sum-ii-input-array-is-sorted/

/*
Given a 1-indexed array of integers numbers that is already sorted in non-decreasing order, find two numbers such that they add up to a specific target number. Let these two numbers be numbers[index1] and numbers[index2] where 1 <= index1 < index2 <= numbers.length.

Return the indices of the two numbers, index1 and index2, added by one as an integer array [index1, index2] of length 2.

The tests are generated such that there is exactly one solution. You may not use the same element twice.

Your solution must use only constant extra space.

 

Example 1:

Input: numbers = [2,7,11,15], target = 9
Output: [1,2]
Explanation: The sum of 2 and 7 is 9. Therefore, index1 = 1, index2 = 2. We return [1, 2].
Example 2:

Input: numbers = [2,3,4], target = 6
Output: [1,3]
Explanation: The sum of 2 and 4 is 6. Therefore index1 = 1, index2 = 3. We return [1, 3].
Example 3:

Input: numbers = [-1,0], target = -1
Output: [1,2]
Explanation: The sum of -1 and 0 is -1. Therefore index1 = 1, index2 = 2. We return [1, 2].
 

Constraints:

2 <= numbers.length <= 3 * 10^4
-1000 <= numbers[i] <= 1000
numbers is sorted in non-decreasing order.
-1000 <= target <= 1000
The tests are generated such that there is exactly one solution.
*/

/*
Runtime: 4 ms, faster than 43.39% of Rust online submissions for Two Sum II - Input Array Is Sorted.
Memory Usage: 2.2 MB, less than 45.17% of Rust online submissions for Two Sum II - Input Array Is Sorted.
*/

struct Solution {
}

impl Solution {
    
    pub fn binary_search(numbers: &Vec<i32>, target: i32) -> Result<usize,()> {
        
        if numbers.len() == 0 {
            return Err(())
        }
        
        let mut left = 0;
        let mut right: i32 = (numbers.len() - 1) as i32;
        
        loop {
            let mid = (left + right) / 2;

            if target > numbers[mid as usize] {
                left = mid + 1;
            } else if target < numbers[mid as usize] {
                right = mid - 1;
            } else {
                return Ok(mid as usize);
            }
            
            if left > right {
                return Err(())
            }
        }
    }
    
    pub fn two_sum(numbers: Vec<i32>, target: i32) -> Vec<i32> {
        for i in 0..numbers.len() {
            let cur = numbers[i];
            let expect = target - cur;
            match Solution::binary_search(&numbers, expect) {
                Ok(index) => {
                    if index != i {
                        return vec![i as i32 + 1, index as i32 + 1];
                    }
                    if index < numbers.len() - 1 && numbers[index + 1] == cur {
                        return vec![i as i32 + 1, i as i32 + 2]; 
                    }
                }
                Err(_) => {},
            }
        }
        
        return vec![];
    }
}

fn main() {
    println!("{:#?}", Solution::two_sum(vec![2,7,11,15], 9));
    println!("{:#?}", Solution::two_sum(vec![2,3,4], 6));
    println!("{:#?}", Solution::two_sum(vec![-1,0], -1));
    println!("{:#?}", Solution::two_sum(vec![2,7,11,11,11,11,12,12,15], 24));
}
