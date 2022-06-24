// main.rs
// Author: hyan23
// Date: 2022.06.24
// https://leetcode.com/problems/rotting-oranges/

/*
You are given an m x n grid where each cell can have one of three values:

0 representing an empty cell,
1 representing a fresh orange, or
2 representing a rotten orange.
Every minute, any fresh orange that is 4-directionally adjacent to a rotten orange becomes rotten.

Return the minimum number of minutes that must elapse until no cell has a fresh orange. If this is impossible, return -1.

 

Example 1:


Input: grid = [[2,1,1],[1,1,0],[0,1,1]]
Output: 4
Example 2:

Input: grid = [[2,1,1],[0,1,1],[1,0,1]]
Output: -1
Explanation: The orange in the bottom left corner (row 2, column 0) is never rotten, because rotting only happens 4-directionally.
Example 3:

Input: grid = [[0,2]]
Output: 0
Explanation: Since there are already no fresh oranges at minute 0, the answer is just 0.
 

Constraints:

m == grid.length
n == grid[i].length
1 <= m, n <= 10
grid[i][j] is 0, 1, or 2.
*/

/*
Runtime: 4 ms, faster than 19.57% of Rust online submissions for Rotting Oranges.
Memory Usage: 2.1 MB, less than 41.30% of Rust online submissions for Rotting Oranges.
*/

struct Solution {
}

impl Solution {
    fn rot(state: &mut Vec<Vec<i32>>, i: i32, j: i32) -> bool {
        if i < 0 || j < 0 || i >= state.len() as i32 || j >= state[0].len() as i32 {
            return false;
        }
        
        if state[i as usize][j as usize] == 1 {
            state[i as usize][j as usize] = 2;
            return true;
        }
        return false;
    }
    
    pub fn oranges_rotting(grid: Vec<Vec<i32>>) -> i32 {
        if grid.len() == 0 {
            return 0;
        }
        
        let mut rotten_count = 0;
        let mut fresh_count = 0;
        let mut state = vec![vec![0; grid[0].len()]; grid.len()];
        let mut stack: Vec<(usize, usize)> = Vec::new();
        for i in 0..grid.len() {
            for j in 0..grid[0].len() {
                state[i][j] = grid[i][j];
                if grid[i][j] == 1 {
                    fresh_count += 1;
                } else if grid[i][j] == 2 {
                    rotten_count += 1;
                    stack.push((i,j));
                }
            }
        }
        
        if fresh_count == 0 {
            return 0;
        }
        if rotten_count == 0 {
            return -1;
        }
        
        let mut min = 0;
        loop {
            let mut tmp: Vec<(usize,usize)> = Vec::new();
            for ij in &stack {
                let i = ij.0;
                let j = ij.1;
                
                for v in [(1,0),(-1,0),(0,1),(0,-1)] {
                    let vi = i as i32 + v.0;
                    let vj = j as i32 + v.1;
                    if Solution::rot(&mut state, vi, vj) {
                        tmp.push((vi as usize, vj as usize));
                        fresh_count -= 1;
                    }
                }
            }
            if tmp.len() == 0 {
                break;
            }
            min += 1;
            stack = tmp;
        }
        
        return if fresh_count <= 0 { min } else { -1 };
    }
}

fn main() {
}
