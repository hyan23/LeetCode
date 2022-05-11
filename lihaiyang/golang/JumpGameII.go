// JumpGameII.go
// Author: hyan23
// 2022.05.11
// https://leetcode.com/problems/jump-game-ii/

/*
Given an array of non-negative integers nums, you are initially positioned at the first index of the array.

Each element in the array represents your maximum jump length at that position.

Your goal is to reach the last index in the minimum number of jumps.

You can assume that you can always reach the last index.



Example 1:

Input: nums = [2,3,1,1,4]
Output: 2
Explanation: The minimum number of jumps to reach the last index is 2. Jump 1 step from index 0 to 1, then 3 steps to the last index.
Example 2:

Input: nums = [2,3,0,1,4]
Output: 2


Constraints:

1 <= nums.length <= 10^4
0 <= nums[i] <= 1000
*/

/*
Runtime: 53 ms, faster than 30.46% of Go online submissions for Jump Game II.
Memory Usage: 6.4 MB, less than 25.42% of Go online submissions for Jump Game II.
*/

package main

import "fmt"

func jump(nums []int) int {
	if len(nums) == 0 {
		return 0
	}

	dp := make([]int, len(nums))
	dp[len(nums)-1] = 0

	for i := len(nums) - 2; i >= 0; i-- {
		farest := nums[i] + i
		if farest >= len(nums)-1 {
			dp[i] = 1
		} else {
			dp[i] = 100000
			for j := i + 1; j <= farest; j++ {
				if dp[j] < dp[i] {
					dp[i] = dp[j]
				}
			}
			dp[i] += 1
		}
	}

	return dp[0]
}

func main() {
	fmt.Println(jump([]int{2, 3, 1, 1, 4}))
	fmt.Println(jump([]int{2, 3, 0, 1, 4}))
	fmt.Println(jump([]int{2, 3, 1, 4, 2, 1, 4, 3, 6, 2, 4, 3, 4, 8, 1, 4, 1, 4}))
}
