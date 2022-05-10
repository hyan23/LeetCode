// HouseRobberII.go
// Author: hyan23
// 2022.05.10
// https://leetcode.com/problems/house-robber-ii/

/*
You are a professional robber planning to rob houses along a street. Each house has a certain amount of money stashed. All houses at this place are arranged in a circle. That means the first house is the neighbor of the last one. Meanwhile, adjacent houses have a security system connected, and it will automatically contact the police if two adjacent houses were broken into on the same night.

Given an integer array nums representing the amount of money of each house, return the maximum amount of money you can rob tonight without alerting the police.



Example 1:

Input: nums = [2,3,2]
Output: 3
Explanation: You cannot rob house 1 (money = 2) and then rob house 3 (money = 2), because they are adjacent houses.
Example 2:

Input: nums = [1,2,3,1]
Output: 4
Explanation: Rob house 1 (money = 1) and then rob house 3 (money = 3).
Total amount you can rob = 1 + 3 = 4.
Example 3:

Input: nums = [1,2,3]
Output: 3


Constraints:

1 <= nums.length <= 100
0 <= nums[i] <= 1000
*/

/*
Runtime: 2 ms, faster than 37.69% of Go online submissions for House Robber II.
Memory Usage: 1.9 MB, less than 72.70% of Go online submissions for House Robber II.
*/

package main

func rob(nums []int) int {
	if len(nums) == 0 {
		return 0
	} else if len(nums) == 1 {
		return nums[0]
	}

	tmp1 := helper(nums[1:])
	tmp2 := helper(nums[:len(nums)-1])
	if tmp1 > tmp2 {
		return tmp1
	}
	return tmp2
}

func helper(nums []int) int {
	if len(nums) == 0 {
		return 0
	}
	robbed := make([]bool, len(nums))
	dp := make([]int, len(nums)+1)
	dp[1] = nums[0]
	robbed[0] = true
	if len(nums) > 1 {
		if nums[0] > nums[1] {
			dp[2] = nums[0]
			robbed[0] = true
		} else {
			dp[2] = nums[1]
			robbed[1] = true
		}
	}
	for i := 3; i <= len(nums); i++ {
		tmp1 := dp[i-2] + nums[i-1]
		if !robbed[i-2] {
			tmp2 := dp[i-1] + nums[i-1]
			if tmp2 > tmp1 {
				tmp1 = tmp2
			}
		}
		var tmp2 = dp[i-1]

		if tmp1 > tmp2 {
			robbed[i-1] = true
			dp[i] = tmp1
		} else {
			dp[i] = tmp2
		}
	}
	return dp[len(nums)]
}

func main() {
}
