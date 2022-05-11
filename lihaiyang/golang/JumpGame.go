// JumpGame.go
// Author: hyan23
// 2022.05.11
// https://leetcode.com/problems/jump-game/

/*
You are given an integer array nums. You are initially positioned at the array's first index, and each element in the array represents your maximum jump length at that position.

Return true if you can reach the last index, or false otherwise.



Example 1:

Input: nums = [2,3,1,1,4]
Output: true
Explanation: Jump 1 step from index 0 to 1, then 3 steps to the last index.
Example 2:

Input: nums = [3,2,1,0,4]
Output: false
Explanation: You will always arrive at index 3 no matter what. Its maximum jump length is 0, which makes it impossible to reach the last index.


Constraints:

1 <= nums.length <= 10^4
0 <= nums[i] <= 10^5
*/

/*
Runtime: 96 ms, faster than 46.43% of Go online submissions for Jump Game.
Memory Usage: 7.9 MB, less than 28.91% of Go online submissions for Jump Game.
*/

package main

import "fmt"

func maxOfInt(a int, b int) int {
	if a > b {
		return a
	}
	return b
}

func canJump(nums []int) bool {
	if len(nums) == 0 {
		return true
	}
	dp := make([]int, len(nums))

	dp[0] = nums[0]
	for i := 1; i < len(nums); i++ {
		if dp[i-1] == 0 {
			return false
		}
		dp[i] = maxOfInt(nums[i], dp[i-1]-1)
	}

	return true
}

func main() {
	fmt.Println(canJump([]int{2, 3, 1, 1, 4}))
	fmt.Println(canJump([]int{3, 2, 1, 0, 4}))
	fmt.Println(canJump([]int{2, 0, 0}))
}
