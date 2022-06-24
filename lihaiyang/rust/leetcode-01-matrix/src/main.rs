// main.rs
// Author: hyan23
// Date: 2022.06.22
// https://leetcode.com/problems/01-matrix/

/*
Given an m x n binary matrix mat, return the distance of the nearest 0 for each cell.

The distance between two adjacent cells is 1.

 

Example 1:


Input: mat = [[0,0,0],[0,1,0],[0,0,0]]
Output: [[0,0,0],[0,1,0],[0,0,0]]
Example 2:


Input: mat = [[0,0,0],[0,1,0],[1,1,1]]
Output: [[0,0,0],[0,1,0],[1,2,1]]
 

Constraints:

m == mat.length
n == mat[i].length
1 <= m, n <= 10^4
1 <= m * n <= 10^4
mat[i][j] is either 0 or 1.
There is at least one 0 in mat.
*/

/*
Runtime: 23 ms, faster than 96.36% of Rust online submissions for 01 Matrix.
Memory Usage: 3.2 MB, less than 38.18% of Rust online submissions for 01 Matrix.
*/

struct Solution {
}

impl Solution {
    fn update_cell(mat: &mut Vec<Vec<i32>>, i: i32, j: i32, distance: i32) -> bool {
        if i < 0 || j < 0 || i >= mat.len() as i32 || j >= mat[0].len() as i32 {
            return false;
        }
        if mat[i as usize][j as usize] != -1 {
            return false;
        }
        
        mat[i as usize][j as usize] = distance;
        return true;
    }
    
    pub fn update_matrix(mat: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        if mat.len() == 0 {
            return vec![vec![]];
        }
        
        let mut sta : Vec<(usize, usize)> = Vec::new();
        let mut result = vec![vec![-1;mat[0].len()];mat.len()];
        for i in 0..mat.len() {
            for j in 0..mat[0].len() {
                
                if mat[i][j] == 0 {
                    sta.push((i, j));
                    result[i][j] = 0;
                }
            }
        }
        
        let mut distance = 1;
        loop {
            let mut tmp:Vec<(usize,usize)>= Vec::new();
            for ij in &sta {
                let i = ij.0;
                let j = ij.1;
                if Solution::update_cell(&mut result, i as i32-1, j as i32, distance) {
                    tmp.push((i-1,j));
                }
                if Solution::update_cell(&mut result, i as i32+1, j as i32, distance) {
                    tmp.push((i+1,j));
                }
                if Solution::update_cell(&mut result, i as i32, j as i32-1, distance) {
                    tmp.push((i,j-1));
                }
                if Solution::update_cell(&mut result, i as i32, j as i32 + 1, distance) {
                    tmp.push((i,j+1));
                }
            }
            distance += 1;
            if tmp.len() == 0 {
                break;
            }
            sta = tmp;
        }
        return result;
    }
}

fn main() {
}
