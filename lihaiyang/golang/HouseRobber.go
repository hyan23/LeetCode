// HouseRobber.go
// Author: hyan23
// 2022.05.10
// https://leetcode.com/problems/house-robber/

/*
You are a professional robber planning to rob houses along a street. Each house has a certain amount of money stashed, the only constraint stopping you from robbing each of them is that adjacent houses have security systems connected and it will automatically contact the police if two adjacent houses were broken into on the same night.

Given an integer array nums representing the amount of money of each house, return the maximum amount of money you can rob tonight without alerting the police.



Example 1:

Input: nums = [1,2,3,1]
Output: 4
Explanation: Rob house 1 (money = 1) and then rob house 3 (money = 3).
Total amount you can rob = 1 + 3 = 4.
Example 2:

Input: nums = [2,7,9,3,1]
Output: 12
Explanation: Rob house 1 (money = 2), rob house 3 (money = 9) and rob house 5 (money = 1).
Total amount you can rob = 2 + 9 + 1 = 12.


Constraints:

1 <= nums.length <= 100
0 <= nums[i] <= 400
*/

/*
Runtime: 0 ms, faster than 100.00% of Go online submissions for House Robber.
Memory Usage: 2.1 MB, less than 33.22% of Go online submissions for House Robber.
*/

package main

func rob(nums []int) int {
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
