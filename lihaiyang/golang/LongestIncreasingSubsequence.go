// LongestIncreasingSubsequence.go
// Author: hyan23
// 2023.05.31
// https://leetcode.com/problems/longest-increasing-subsequence/description/

/*
Given an integer array nums, return the length of the longest strictly increasing
subsequence
.



Example 1:

Input: nums = [10,9,2,5,3,7,101,18]
Output: 4
Explanation: The longest increasing subsequence is [2,3,7,101], therefore the length is 4.
Example 2:

Input: nums = [0,1,0,3,2,3]
Output: 4
Example 3:

Input: nums = [7,7,7,7,7,7,7]
Output: 1


Constraints:

1 <= nums.length <= 2500
-10^4 <= nums[i] <= 10^4


Follow up: Can you come up with an algorithm that runs in O(n log(n)) time complexity?
*/

/*
Runtime
46 ms
Beats
61.52%
Memory
3.8 MB
Beats
41.23%
*/

package main

func lengthOfLIS(nums []int) int {
	dp := make([]int, len(nums), len(nums))
	dp[0] = 1
	for i := 1; i < len(nums); i++ {
		dp[i] = 0
		for j := i - 1; j >= 0; j-- {
			if nums[j] < nums[i] {
				if dp[j] > dp[i] {
					dp[i] = dp[j]
				}
			}
		}

		dp[i] += 1
	}
	max := 0
	for i := 0; i < len(dp); i++ {
		if max < dp[i] {
			max = dp[i]
		}
	}

	return max
}

func main() {
}
