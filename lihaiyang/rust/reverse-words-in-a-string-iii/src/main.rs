// main.rs
// Author: hyan23
// 2022.06.17
// https://leetcode.com/problems/reverse-words-in-a-string-iii/

/*
Given a string s, reverse the order of characters in each word within a sentence while still preserving whitespace and initial word order.

 

Example 1:

Input: s = "Let's take LeetCode contest"
Output: "s'teL ekat edoCteeL tsetnoc"
Example 2:

Input: s = "God Ding"
Output: "doG gniD"
 

Constraints:

1 <= s.length <= 5 * 10^4
s contains printable ASCII characters.
s does not contain any leading or trailing spaces.
There is at least one word in s.
All the words in s are separated by a single space.
*/

/*
Runtime: 0 ms, faster than 100.00% of Rust online submissions for Reverse Words in a String III.
Memory Usage: 2.2 MB, less than 97.09% of Rust online submissions for Reverse Words in a String III.
*/

struct Solution {
}

impl Solution {
    
    pub fn reverse_word3(s :&mut Vec<u8>, mut word_begin :usize, mut word_end :usize) {
        while word_begin < word_end {
            let tmp = (s[word_end], s[word_begin]);
            s[word_begin] = tmp.0;
            s[word_end] = tmp.1;
            word_begin += 1;
            word_end -= 1;
        }
    }
    
    pub fn reverse_word(word :&mut String) -> &mut String {
        let word_len = word.len();
        if word_len < 2 {
            return word;
        }
        Solution::reverse_word3(unsafe { word.as_mut_vec() }, 0, word_len - 1);
        return word;
    }
    
    pub fn reverse_words(s: String) -> String {
        let mut copy = s;
        let arr = unsafe{ copy.as_mut_vec() };
        
        let mut word_begin : Option<usize> = Some(0);
        let mut word_boundaries : Vec<(usize, usize)> = Vec::new();
        
        for i in 0..arr.len() {
            let cur = arr[i] as char;
            
            if cur == ' ' {
                if word_begin.is_some() {
                    word_boundaries.push((word_begin.unwrap(), i - 1));
                    word_begin = None;
                }
            } else if i == arr.len() - 1 {
                if word_begin.is_some() {
                    word_boundaries.push((word_begin.unwrap(), i));
                    word_begin = None;
                }
            } else {
                match word_begin {
                    Some(_) => {},
                    None => {
                        word_begin = Some(i);
                    }
                }
            }
        }
        
        for word in word_boundaries {
            Solution::reverse_word3(unsafe {copy.as_mut_vec()}, word.0, word.1);
        }
        
        return copy;
    }
}

fn main() {
    println!("{}", Solution::reverse_word(&mut String::from("Hello")));
    println!("{}", Solution::reverse_word(&mut String::from("World!")));
    println!("{}", Solution::reverse_word(&mut String::from("A")));
    println!("{}", Solution::reverse_word(&mut String::from("")));
    println!("{}", Solution::reverse_words(String::from("Let's take LeetCode contest")));
    println!("{}", Solution::reverse_words(String::from("God Ding")));
}
