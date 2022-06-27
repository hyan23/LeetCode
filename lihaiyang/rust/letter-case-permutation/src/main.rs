// main.rs
// Author: hyan23
// Date: 2022.06.27
// https://leetcode.com/problems/letter-case-permutation/

/*
Given a string s, you can transform every letter individually to be lowercase or uppercase to create another string.

Return a list of all possible strings we could create. Return the output in any order.

 

Example 1:

Input: s = "a1b2"
Output: ["a1b2","a1B2","A1b2","A1B2"]
Example 2:

Input: s = "3z4"
Output: ["3z4","3Z4"]
 

Constraints:

1 <= s.length <= 12
s consists of lowercase English letters, uppercase English letters, and digits.
*/

/*
Runtime: 3 ms, faster than 89.29% of Rust online submissions for Letter Case Permutation.
Memory Usage: 2.3 MB, less than 78.57% of Rust online submissions for Letter Case Permutation.
*/

struct Solution {
}

impl Solution {
    pub fn letter_case_permutation(s: String) -> Vec<String> {
        
        let s1 = vec![s.to_uppercase(),s.to_lowercase()];
        
        let mut letter_pos = vec![];
        for i in 0..s.len() {
            let ch = s.as_bytes()[i];
            if ch.is_ascii_alphabetic() {
                letter_pos.push(i);
            }
        }
        
        let mut results = vec![];
        for i in 0..2u64.pow(letter_pos.len() as u32) {
            let mut s2 = s.clone();
            for j in 0..letter_pos.len() {
                let pos = letter_pos[j];
                unsafe{ s2.as_bytes_mut()[pos] =  s1[(i>>j&1) as usize].as_bytes()[pos]; }
            }
            results.push(s2);
        }
        
        return results;
    }
}

fn main() {
}
