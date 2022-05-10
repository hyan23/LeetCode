// ClimbingStairs.go
// Author: hyan23
// 2022.05.10
// https://leetcode.com/problems/climbing-stairs/

/*
You are climbing a staircase. It takes n steps to reach the top.

Each time you can either climb 1 or 2 steps. In how many distinct ways can you climb to the top?



Example 1:

Input: n = 2
Output: 2
Explanation: There are two ways to climb to the top.
1. 1 step + 1 step
2. 2 steps
Example 2:

Input: n = 3
Output: 3
Explanation: There are three ways to climb to the top.
1. 1 step + 1 step + 1 step
2. 1 step + 2 steps
3. 2 steps + 1 step


Constraints:

1 <= n <= 45
*/

/*
Runtime: 0 ms, faster than 100.00% of Go online submissions for Climbing Stairs.
Memory Usage: 1.9 MB, less than 87.48% of Go online submissions for Climbing Stairs.
*/

package main

import "fmt"

func climbStairs(n int) int {
	dp := make([]int, n+1)
	dp[1] = 1
	if n > 1 {
		dp[2] = 2
	}
	for i := 3; i <= n; i++ {
		dp[i] = dp[i-1] + dp[i-2]
	}
	return dp[n]
}

func main() {
	fmt.Println(climbStairs(1))
}
