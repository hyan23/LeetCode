// main.rs
// Author: hyan23
// 2023.09.20
// https://leetcode.com/problems/reverse-vowels-of-a-string/?envType=study-plan-v2&envId=leetcode-75

/*
Given a string s, reverse only all the vowels in the string and return it.

The vowels are 'a', 'e', 'i', 'o', and 'u', and they can appear in both lower and upper cases, more than once.



Example 1:

Input: s = "hello"
Output: "holle"
Example 2:

Input: s = "leetcode"
Output: "leotcede"


Constraints:

1 <= s.length <= 3 * 105
s consist of printable ASCII characters.
*/

/*

*/

struct Solution {}
impl Solution {
    fn is_vowel(ch: char) -> bool {
        ['a', 'e', 'i', 'o', 'u'].contains(&ch.to_ascii_lowercase())
    }

    pub fn reverse_vowels(s: String) -> String {
        let mut s1 = String::from(s);
        unsafe {
            let result = s1.as_bytes_mut();
            let mut i = 0;
            let mut j = result.len() - 1;
            while i < j {
                while !Self::is_vowel(result[i] as char) && i < j {
                    i += 1;
                }
                while !Self::is_vowel(result[j] as char) && j > i {
                    j -= 1;
                }
                if i < j {
                    let t = result[i];
                    result[i] = result[j];
                    result[j] = t;
                    i += 1;
                    j -= 1;
                }
            }
        }
        s1
    }
    pub fn reverse_vowels1(s: String) -> String {
        let mut indices: Vec<usize> = Vec::new();
        for i in 0..s.len() {
            if Self::is_vowel(s.chars().nth(i).unwrap()) {
                indices.push(i);
            }
        }
        let mut s1 = String::from(s);
        unsafe {
            let len = indices.len();
            let result = s1.as_bytes_mut();
            for i in 0..(len / 2) {
                let t = result[indices[i]];
                result[indices[i]] = result[indices[len - i - 1]];
                result[indices[len - i - 1]] = t;
            }
        }
        s1
    }
}

fn main() {}
