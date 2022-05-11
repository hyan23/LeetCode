// DeleteAndEarn.go
// Author: hyan23
// 2022.05.10
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

 */

package main

import "fmt"

func deleteAndEarn(nums []int) int {

	m := make(map[int]int)
	deleteBySelf := make(map[int]bool)
	for i := 1; i <= 104; i++ {
		m[i] = 0
		deleteBySelf[i] = false
	}
	max := 0
	for _, e := range nums {
		m[e]++
		if e > max {
			max = e
		}
	}

	dp := make([]int, 104+1)

	dp[1] = m[1]
	if m[1] > 0 {
		deleteBySelf[1] = true
	}
	for i := 2; i <= 104; i++ {
		if m[i] == 0 {
			dp[i] = dp[i-1]
			continue
		}
		if m[i-1] == 0 {
			dp[i] = dp[i-1] + m[i]*i
			deleteBySelf[i] = true
		} else {
			if deleteBySelf[i-1] {
				deleteMeEarn := m[i] * i
				deleteMeLoss := m[i-1] * (i - 1)
				undo := false
				if yes, ok := deleteBySelf[i-3]; !ok || !yes {
					if n, ok := m[i-2]; ok && n > 0 {
						deleteMeEarn += m[i-2] * (i - 2)
						undo = true
					}
				}
				if deleteMeEarn >= deleteMeLoss {
					dp[i] = dp[i-1] - deleteMeLoss + deleteMeEarn
					// m[i] = 0
					deleteBySelf[i] = true
					deleteBySelf[i-1] = false
					if undo {
						fmt.Println(fmt.Sprintf("undo %v", i-2))
						deleteBySelf[i-2] = true
					}
					// fmt.Printf("delete %v, dp=%v", i, dp[i])
				} else {
					dp[i] = dp[i-1]
				}
			} else {
				dp[i] = dp[i-1] + m[i]*i
				deleteBySelf[i] = true
			}
		}
		fmt.Println(i, dp[i], deleteBySelf[i])
	}

	for i := 1; i <= 104; i++ {
		if m[i] > 0 {
			fmt.Printf("%v:%v ", i, deleteBySelf[i])
		}
	}

	return dp[max]
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
