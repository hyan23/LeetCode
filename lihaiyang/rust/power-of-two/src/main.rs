// main.rs
// Author: hyan23
// Date: 2022.06.28
// https://leetcode.com/problems/power-of-two/

/*
Given an integer n, return true if it is a power of two. Otherwise, return false.

An integer n is a power of two, if there exists an integer x such that n == 2^x.

 

Example 1:

Input: n = 1
Output: true
Explanation: 20 = 1
Example 2:

Input: n = 16
Output: true
Explanation: 24 = 16
Example 3:

Input: n = 3
Output: false
 

Constraints:

-2^31 <= n <= 2^31 - 1
 

Follow up: Could you solve it without loops/recursion?
*/

/*
Runtime: 0 ms, faster than 100.00% of Rust online submissions for Power of Two.
Memory Usage: 2.2 MB, less than 11.59% of Rust online submissions for Power of Two.
*/

struct Solution {
}

impl Solution {
    pub fn is_power_of_two(n: i32) -> bool {
        if n <= 0 {
            return false;
        }
        let mut m = n;
        let mut bit_count = 0;
        while m != 0 {
            if m & 0x1 == 0x1 {
                bit_count += 1;
                if bit_count > 1 {
                    return false;
                }
            }
            m = m >> 1;
        }
        return bit_count == 1;
    }
}

fn main() {
}
