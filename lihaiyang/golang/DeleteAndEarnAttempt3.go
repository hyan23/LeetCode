// DeleteAndEarn.go
// Author: hyan23
// 2022.05.11
// https://leetcode.com/problems/delete-and-earn/

/*
You are given an integer array nums. You want to maximize the number of points you get by performing the following operation any number of times:

Pick any nums[i] and delete it to earn nums[i] points. Afterwards, you must delete every element equal to nums[i] - 1 and every element equal to nums[i] + 1.
Return the maximum number of points you can earn by applying the above operation some number of times.



Example 1:

Input: nums = [3,4,2]
Output: 6
Explanation: You can perform the following operations:
- Delete 4 to earn 4 points. Consequently, 3 is also deleted. nums = [2].
- Delete 2 to earn 2 points. nums = [].
You earn a total of 6 points.
Example 2:

Input: nums = [2,2,3,3,3,4]
Output: 9
Explanation: You can perform the following operations:
- Delete a 3 to earn 3 points. All 2's and 4's are also deleted. nums = [3,3].
- Delete a 3 again to earn 3 points. nums = [3].
- Delete a 3 once more to earn 3 points. nums = [].
You earn a total of 9 points.


Constraints:

1 <= nums.length <= 2 * 10^4
1 <= nums[i] <= 10^4
*/

/*
Runtime: 56 ms, faster than 5.36% of Go online submissions for Delete and Earn.
Memory Usage: 8.5 MB, less than 5.36% of Go online submissions for Delete and Earn.
*/

package main

import "fmt"

func maxOfInt(a int, b int) int {
	if a > b {
		return a
	}
	return b
}

func deleteAndEarn(nums []int) int {
	if len(nums) < 1 {
		return 0
	}

	// 先统计下输入每个数的个数，并找出其中最大值
	m := make(map[int]int)
	for i := 0; i <= 10000; i++ {
		m[i] = 0
	}
	max := 0
	for _, e := range nums {
		m[e]++
		if e > max {
			max = e
		}
	}

	// dp, 对输入集合A，划分Ai={x|x<=i,x∈A}; 1<=i<=10000
	// dp[i][0]表示对于Ai, 不删除i能得积分数
	// dp[i][1]表示对于Ai, 删除i能得积分数
	dp := make([][2]int, 10000+1)

	// 顺序求解Ai的答案
	for i := 1; i <= max; i++ {
		if m[i] == 0 {
			dp[i][0] = maxOfInt(dp[i-1][0], dp[i-1][1])
			dp[i][1] = dp[i][0]
			continue
		}

		dp[i][1] = m[i]*i + dp[i-1][0]
		dp[i][0] = maxOfInt(dp[i-1][0], dp[i-1][1])
	}

	return maxOfInt(dp[max][0], dp[max][1])
}

func main() {
	fmt.Println(deleteAndEarn([]int{3, 4, 2}))
	fmt.Println()
	fmt.Println(deleteAndEarn([]int{8, 10, 4, 9, 1, 3, 5, 9, 4, 10}))
	fmt.Println()
	fmt.Println(deleteAndEarn([]int{8, 3, 4, 7, 6, 6, 9, 2, 5, 8, 2, 4, 9, 5, 9, 1, 5, 7, 1, 4}))
	fmt.Println()
	fmt.Println(deleteAndEarn([]int{8, 7, 3, 8, 1, 4, 10, 10, 10, 2}))
}
