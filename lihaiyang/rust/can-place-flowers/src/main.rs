// main.rs
// Author: hyan23
// 2023.09.20
// https://leetcode.com/problems/can-place-flowers/?envType=study-plan-v2&envId=leetcode-75

/*
You have a long flowerbed in which some of the plots are planted, and some are not. However, flowers cannot be planted in adjacent plots.

Given an integer array flowerbed containing 0's and 1's, where 0 means empty and 1 means not empty, and an integer n, return true if n new flowers can be planted in the flowerbed without violating the no-adjacent-flowers rule and false otherwise.



Example 1:

Input: flowerbed = [1,0,0,0,1], n = 1
Output: true
Example 2:

Input: flowerbed = [1,0,0,0,1], n = 2
Output: false


Constraints:

1 <= flowerbed.length <= 2 * 104
flowerbed[i] is 0 or 1.
There are no two adjacent flowers in flowerbed.
0 <= n <= flowerbed.length
*/

/*
Runtime
Details
3ms
Beats 76.07%of users with Rust
Memory
Details
2.09MB
Beats 97.38%of users with Rust
*/

use std::ops::RangeFull;

struct Solution {}

impl Solution {
    pub fn can_place_flowers(flowerbed: Vec<i32>, n: i32) -> bool {
        let mut count = 0;
        let mut temp = Vec::from(flowerbed);

        for i in 0..temp.len() {
            if temp[i] == 0
                && (i == 0 || temp[i - 1] == 0)
                && (i == temp.len() - 1 || temp[i + 1] == 0)
            {
                temp[i] = 1;
                count += 1;
            }
        }
        return count >= n;
    }
}

fn main() {}
