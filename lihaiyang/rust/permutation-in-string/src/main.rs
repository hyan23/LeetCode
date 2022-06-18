// main.rs
// Author: hyan23
// 2022.06.18
// https://leetcode.com/problems/permutation-in-string/

/*
Given two strings s1 and s2, return true if s2 contains a permutation of s1, or false otherwise.

In other words, return true if one of s1's permutations is the substring of s2.

 

Example 1:

Input: s1 = "ab", s2 = "eidbaooo"
Output: true
Explanation: s2 contains one permutation of s1 ("ba").
Example 2:

Input: s1 = "ab", s2 = "eidboaoo"
Output: false
 

Constraints:

1 <= s1.length, s2.length <= 10^4
s1 and s2 consist of lowercase English letters.
*/

/*
Runtime: 4 ms, faster than 42.11% of Rust online submissions for Permutation in String.
Memory Usage: 2.2 MB, less than 45.26% of Rust online submissions for Permutation in String.
*/

struct Solution {
}

impl Solution {
    pub fn check_inclusion(s1: String, s2: String) -> bool {
        if s1.len() > s2.len() {
            return false;
        }
        
        let mut letter_types = 0;
        let mut letter_count = [0;256];
        for ch in s1.chars() {
            if letter_count[ch as u8 as usize] == 0 {
                letter_types += 1;
            }
            letter_count[ch as u8 as usize] += 1;
        }
        
        let mut difference = letter_types;
        let mut window_letter_count = [0;256];
        for i in 0..s1.len() {
            let ch = s2.as_bytes()[i];
            if window_letter_count[ch as usize] == letter_count[ch as usize] && letter_count[ch as usize] > 0 {
                difference += 1;
            }
            window_letter_count[ch as usize] += 1;
            if window_letter_count[ch as usize] == letter_count[ch as usize] && letter_count[ch as usize] > 0 {
                difference -= 1;
            }
        }
        
        if difference == 0 {
            return true;
        }
        
        let mut i = s1.len();
        while i < s2.len() {
            
            let ch_removed = s2.as_bytes()[i - s1.len()];
            let ch_added = s2.as_bytes()[i];
            
            if ch_removed == ch_added {
                i += 1;
                continue;
            }
            
            if window_letter_count[ch_added as usize] == letter_count[ch_added as usize] && letter_count[ch_added as usize] > 0 {
                difference += 1;
            }
            if window_letter_count[ch_removed as usize] == letter_count[ch_removed as usize] && letter_count[ch_removed as usize] > 0 {
                difference += 1;
            }
            window_letter_count[ch_removed as usize] -= 1;
            if window_letter_count[ch_removed as usize] == letter_count[ch_removed as usize] && letter_count[ch_removed as usize] > 0 {
                difference -= 1;
            }
            window_letter_count[ch_added as usize] += 1;
            if window_letter_count[ch_added as usize] == letter_count[ch_added as usize] && letter_count[ch_added as usize] > 0 {
                difference -= 1;
            }
            if difference == 0 {
                return true;
            }
            
            i += 1;
        }
        
        return false;
    }
}

fn main() {
    println!("{}", Solution::check_inclusion(String::from(""), String::from("")));
    println!("{}", Solution::check_inclusion(String::from("ab"), String::from("eidbaooo")));
    println!("{}", Solution::check_inclusion(String::from("ab"), String::from("eidboaoo")));
    println!("{}", Solution::check_inclusion(String::from("5523"), String::from("eidb532532oaoo")));
    println!("{}", Solution::check_inclusion(String::from("5523b"), String::from("eidb532532oaoo")));
    println!("{}", Solution::check_inclusion(String::from("5523be"), String::from("eidb532532oaoo")));
}
