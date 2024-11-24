// main.rs
// Author: hyan23
// 2023.09.21
// https://leetcode.com/problems/is-subsequence/?envType=study-plan-v2&envId=leetcode-75

/*
Given two strings s and t, return true if s is a subsequence of t, or false otherwise.

A subsequence of a string is a new string that is formed from the original string by deleting some (can be none) of the characters without disturbing the relative positions of the remaining characters. (i.e., "ace" is a subsequence of "abcde" while "aec" is not).



Example 1:

Input: s = "abc", t = "ahbgdc"
Output: true
Example 2:

Input: s = "axc", t = "ahbgdc"
Output: false


Constraints:

0 <= s.length <= 100
0 <= t.length <= 104
s and t consist only of lowercase English letters.


Follow up: Suppose there are lots of incoming s, say s1, s2, ..., sk where k >= 109, and you want to check one by one to see if t has its subsequence. In this scenario, how would you change your code?
*/

/*
Runtime
Details
56ms
Beats 5.36%of users with Rust
Memory
Details
2.16MB
Beats 36.78%of users with Rust
*/

struct Solution {}
impl Solution {
    pub fn is_subsequence(s: String, t: String) -> bool {
        let mut i = 0;
        for ch in s.chars() {
            let mut found = false;
            while i < t.len() {
                if ch == t.chars().nth(i).unwrap() {
                    i += 1;
                    found = true;
                    break;
                }
                i += 1;
            }
            if !found {
                return false;
            }
        }
        return true;
    }
}

fn main() {}
