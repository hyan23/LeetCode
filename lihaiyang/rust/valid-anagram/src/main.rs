// main.rs
// Author: hyan23
// 2022.06.12
// https://leetcode.com/problems/valid-anagram/

/*
Given two strings s and t, return true if t is an anagram of s, and false otherwise.

An Anagram is a word or phrase formed by rearranging the letters of a different word or phrase, typically using all the original letters exactly once.

 

Example 1:

Input: s = "anagram", t = "nagaram"
Output: true
Example 2:

Input: s = "rat", t = "car"
Output: false
 

Constraints:

1 <= s.length, t.length <= 5 * 104
s and t consist of lowercase English letters.
 

Follow up: What if the inputs contain Unicode characters? How would you adapt your solution to such a case?
*/

/*
Runtime: 0 ms, faster than 100.00% of Rust online submissions for Valid Anagram.
Memory Usage: 2.3 MB, less than 67.31% of Rust online submissions for Valid Anagram.
*/

struct Solution {
}

impl Solution {
    pub fn is_anagram(s: String, t: String) -> bool {
        
        let mut counter_of_s = [0u16;256];
        let mut counter_of_t = [0u16;256];
        for c in s.chars() {
            counter_of_s[c as usize] += 1;
        }
        for c in t.chars() {
            counter_of_t[c as usize] += 1;
        }
        
        let mut equal = true;
        for i in 0..256 {
            if counter_of_s[i] != counter_of_t[i] {
                equal = false;
                break;
            }
        }
        
        return s.len()==t.len() && equal;
    }
}

fn main() {
    println!("{}", Solution::is_anagram(String::from("anagram"), String::from("nagaram")));
    println!("{}", Solution::is_anagram(String::from("rat"), String::from("car")));
}
