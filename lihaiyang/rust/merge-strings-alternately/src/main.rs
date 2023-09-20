// main.rs
// Author: hyan23
// 2023.09.19
// https://leetcode.com/problems/merge-strings-alternately/?envType=study-plan-v2&envId=leetcode-75

/*
You are given two strings word1 and word2. Merge the strings by adding letters in alternating order, starting with word1. If a string is longer than the other, append the additional letters onto the end of the merged string.

Return the merged string.



Example 1:

Input: word1 = "abc", word2 = "pqr"
Output: "apbqcr"
Explanation: The merged string will be merged as so:
word1:  a   b   c
word2:    p   q   r
merged: a p b q c r
Example 2:

Input: word1 = "ab", word2 = "pqrs"
Output: "apbqrs"
Explanation: Notice that as word2 is longer, "rs" is appended to the end.
word1:  a   b
word2:    p   q   r   s
merged: a p b q   r   s
Example 3:

Input: word1 = "abcd", word2 = "pq"
Output: "apbqcd"
Explanation: Notice that as word1 is longer, "cd" is appended to the end.
word1:  a   b   c   d
word2:    p   q
merged: a p b q c   d


Constraints:

1 <= word1.length, word2.length <= 100
word1 and word2 consist of lowercase English letters.
*/

/*
Runtime
Details
1ms
Beats 74.45%of users with Rust
Memory
Details
2.16MB
Beats 33.27%of users with Rust
*/

struct Solution {}

impl Solution {
    pub fn merge_alternately(word1: String, word2: String) -> String {
        let mut result = String::from("");
        let mut i = 0;
        while i < word1.len() && i < word2.len() {
            result.push(word1.chars().nth(i).unwrap());
            result.push(word2.chars().nth(i).unwrap());
            i += 1;
        }
        if i < word1.len() {
            result.push_str(&word1[i..]);
        }
        if i < word2.len() {
            result.push_str(&word2[i..]);
        }
        result
    }
}

fn main() {
    println!(
        "{}",
        Solution::merge_alternately(String::from("abc"), String::from("def"))
    );
    println!(
        "{}",
        Solution::merge_alternately(String::from("abc"), String::from("defeee"))
    );

    println!(
        "{}",
        Solution::merge_alternately(String::from("abcddd"), String::from("def"))
    );

    println!(
        "{}",
        Solution::merge_alternately(String::from("abc"), String::from(""))
    );

    println!(
        "{}",
        Solution::merge_alternately(String::from(""), String::from(""))
    );
}
