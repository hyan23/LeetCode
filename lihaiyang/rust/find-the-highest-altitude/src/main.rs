// main.rs
// Author: hyan23
// 2023.09.24
// https://leetcode.com/problems/find-the-highest-altitude/?envType=study-plan-v2&envId=leetcode-75

/*
There is a biker going on a road trip. The road trip consists of n + 1 points at different altitudes. The biker starts his trip on point 0 with altitude equal 0.

You are given an integer array gain of length n where gain[i] is the net gain in altitude between points i​​​​​​ and i + 1 for all (0 <= i < n). Return the highest altitude of a point.



Example 1:

Input: gain = [-5,1,5,0,-7]
Output: 1
Explanation: The altitudes are [0,-5,-4,1,1,-6]. The highest is 1.
Example 2:

Input: gain = [-4,-3,-2,-1,4,3,2]
Output: 0
Explanation: The altitudes are [0,-4,-7,-9,-10,-6,-3,-1]. The highest is 0.
*/

/*
Runtime
Details
1ms
Beats 33.33%of users with Rust
Memory
Details
1.96MB
Beats 91.67%of users with Rust
*/

struct Solution {}
impl Solution {
    pub fn largest_altitude(gain: Vec<i32>) -> i32 {
        let mut al = 0;
        let mut max_al = 0;
        for i in gain {
            al += i;
            if al > max_al {
                max_al = al;
            }
        }
        max_al
    }
}

fn main() {}
