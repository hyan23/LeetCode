// main.rs
// Author: hyan23
// Date: 2022.06.21
// https://leetcode.com/problems/max-area-of-island/

/*
You are given an m x n binary matrix grid. An island is a group of 1's (representing land) connected 4-directionally (horizontal or vertical.) You may assume all four edges of the grid are surrounded by water.

The area of an island is the number of cells with a value 1 in the island.

Return the maximum area of an island in grid. If there is no island, return 0.

 

Example 1:


Input: grid = [[0,0,1,0,0,0,0,1,0,0,0,0,0],[0,0,0,0,0,0,0,1,1,1,0,0,0],[0,1,1,0,1,0,0,0,0,0,0,0,0],[0,1,0,0,1,1,0,0,1,0,1,0,0],[0,1,0,0,1,1,0,0,1,1,1,0,0],[0,0,0,0,0,0,0,0,0,0,1,0,0],[0,0,0,0,0,0,0,1,1,1,0,0,0],[0,0,0,0,0,0,0,1,1,0,0,0,0]]
Output: 6
Explanation: The answer is not 11, because the island must be connected 4-directionally.
Example 2:

Input: grid = [[0,0,0,0,0,0,0,0]]
Output: 0
 

Constraints:

m == grid.length
n == grid[i].length
1 <= m, n <= 50
grid[i][j] is either 0 or 1.
*/

/*
Runtime: 8 ms, faster than 33.33% of Rust online submissions for Max Area of Island.
Memory Usage: 2.5 MB, less than 16.67% of Rust online submissions for Max Area of Island.
*/

struct Solution {
}

impl Solution {
    fn visit(i: i32, j: i32, grid: &Vec<Vec<i32>>, visited: &mut Vec<Vec<bool>>) -> i32 {
        if i < 0 || j < 0 || i >= grid.len() as i32 || j >= grid[0].len() as i32 {
            return 0;
        }
        if grid[i as usize][j as usize] == 0 || visited[i as usize][j as usize] {
            return 0;
        }
        
        visited[i as usize][j as usize] = true;
        
        return 1 + Solution::visit(i-1, j, grid, visited) + 
            Solution::visit(i+1, j, grid, visited) + 
            Solution::visit(i, j-1, grid, visited) + 
            Solution::visit(i, j+1, grid, visited);
    }
    
    pub fn max_area_of_island(grid: Vec<Vec<i32>>) -> i32 {
        if grid.len() == 0 {
            return 0;
        }
        let mut visited : Vec<Vec<bool>> = vec![vec![false; grid[0].len()]; grid.len()];
        
        let mut max = 0;
        for i in 0..grid.len() {
            for j in 0..grid[0].len() {
                let area = Solution::visit(i as i32, j as i32, &grid, &mut visited);
                if area > max {
                    max = area;
                }
            }
        }
        
        return max;
    }
}

fn main() {
}
