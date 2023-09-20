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

*/

struct Solution {}

impl Solution {
    fn check(str1: &String, t: &String) -> bool {
        if str1.len() == 0 && t.len() == 0 {
            return true;
        }
        if str1.len() == 0 || t.len() == 0 {
            return false;
        }
        if str1.len() < t.len() {
            return false;
        }
        if str1.len() % t.len() != 0 {
            return false;
        }
        let mut i = 0;
        while i < str1.len() {
            if str1[i..i + t.len()] != t[..] {
                return false;
            }
            i += t.len();
        }
        return true;
    }

    pub fn gcd_of_strings(str1: String, str2: String) -> String {
        let mut i = 1;
        let mut result = String::from("");
        while i <= str1.len() && i <= str2.len() {
            let t = String::from(&str1[..i]);
            if Solution::check(&str1, &t) && Solution::check(&str2, &t) {
                result = t;
            }
            i += 1;
        }
        result
    }
}

fn main() {
    println!("{}", Solution::check(&String::from(""), &String::from("")));
    println!("{}", Solution::check(&String::from(""), &String::from("a")));
    println!("{}", Solution::check(&String::from("a"), &String::from("")));
    println!(
        "{}",
        &Solution::check(&String::from("aaaa"), &String::from("a"))
    );
    println!(
        "{}",
        Solution::check(&String::from("aaaa"), &String::from("aa"))
    );
    println!(
        "{}",
        Solution::check(&String::from("aaaab"), &String::from("a"))
    );
    println!(
        "{}",
        Solution::check(&String::from("a"), &String::from("a"))
    );
    println!(
        "{}",
        Solution::check(&String::from("ababab"), &String::from("ab"))
    );
}
