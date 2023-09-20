// main.rs
// Author: hyan23
// 2023.09.20
// https://leetcode.com/problems/reverse-words-in-a-string/?envType=study-plan-v2&envId=leetcode-75

/*
Given an input string s, reverse the order of the words.

A word is defined as a sequence of non-space characters. The words in s will be separated by at least one space.

Return a string of the words in reverse order concatenated by a single space.

Note that s may contain leading or trailing spaces or multiple spaces between two words. The returned string should only have a single space separating the words. Do not include any extra spaces.



Example 1:

Input: s = "the sky is blue"
Output: "blue is sky the"
Example 2:

Input: s = "  hello world  "
Output: "world hello"
Explanation: Your reversed string should not contain leading or trailing spaces.
Example 3:

Input: s = "a good   example"
Output: "example good a"
Explanation: You need to reduce multiple spaces between two words to a single space in the reversed string.


Constraints:

1 <= s.length <= 104
s contains English letters (upper-case and lower-case), digits, and spaces ' '.
There is at least one word in s.


Follow-up: If the string data type is mutable in your language, can you solve it in-place with O(1) extra space?
*/

/*
Runtime
Details
0ms
Beats 100.00%of users with Rust
Memory
Details
2.08MB
Beats 88.92%of users with Rust
*/

struct Solution {}

impl Solution {
    pub fn reverse_words(s: String) -> String {
        let mut words: Vec<&str> = s.split_ascii_whitespace().collect();
        words.reverse();
        words.join(" ")
    }
}

fn main() {}
