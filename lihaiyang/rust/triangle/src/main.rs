// main.rs
// Author: hyan23
// Date: 2022.06.27
// https://leetcode.com/problems/triangle/

/*
Given a triangle array, return the minimum path sum from top to bottom.

For each step, you may move to an adjacent number of the row below. More formally, if you are on index i on the current row, you may move to either index i or index i + 1 on the next row.

 

Example 1:

Input: triangle = [[2],[3,4],[6,5,7],[4,1,8,3]]
Output: 11
Explanation: The triangle looks like:
   2
  3 4
 6 5 7
4 1 8 3
The minimum path sum from top to bottom is 2 + 3 + 5 + 1 = 11 (underlined above).
Example 2:

Input: triangle = [[-10]]
Output: -10
 

Constraints:

1 <= triangle.length <= 200
triangle[0].length == 1
triangle[i].length == triangle[i - 1].length + 1
-10^4 <= triangle[i][j] <= 10^4
 

Follow up: Could you do this using only O(n) extra space, where n is the total number of rows in the triangle?
*/

/*
Runtime: 0 ms, faster than 100.00% of Rust online submissions for Triangle.
Memory Usage: 2.4 MB, less than 14.00% of Rust online submissions for Triangle.
*/

struct Solution {
}

impl Solution {
    pub fn minimum_total(triangle: Vec<Vec<i32>>) -> i32 {
        if triangle.len() == 0 {
            return 0;
        }
        
        let mut dp = vec![triangle[0][0]];
        for i in 1..triangle.len() {
            let mut dp_new = vec![0;triangle[i].len()];
            for j in 0..triangle[i].len() {
                dp_new[j] = triangle[i][j] +
                     if j < dp.len() { dp[j] } else { 10000000 }.min(
                        if j as i32 - 1 >= 0 { dp[j-1] } else { 10000000 });
            }
            dp = dp_new;
        }
        
        match dp.iter().min() {
            None => 0,
            Some(sum) => *sum,
        }
    }
}

fn main() {
}
