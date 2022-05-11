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
	for _, e := range nums {
		m[e]++
	}
	total := 0
	for len(m) > 0 {
		toBeDeleted := []int{}
		for e := range m {
			earn := m[e] * e
			loss := 0
			if n, ok := m[e-1]; ok {
				loss += n * (e - 1)
			}
			if n, ok := m[e+1]; ok {
				loss += n * (e + 1)
			}
			if earn >= loss {
				total += earn
				toBeDeleted = append(toBeDeleted, e)
			}
		}

		if len(toBeDeleted) == 0 {
			for k, v := range m {
				fmt.Printf("%v:%v ", k, v)
			}
			fmt.Println()
		}

		for _, e := range toBeDeleted {
			delete(m, e)
			delete(m, e-1)
			delete(m, e+1)
		}
	}
	return total
}

func main() {
	fmt.Println(deleteAndEarn([]int{8, 10, 4, 9, 1, 3, 5, 9, 4, 10}))
	fmt.Println(deleteAndEarn([]int{8, 3, 4, 7, 6, 6, 9, 2, 5, 8, 2, 4, 9, 5, 9, 1, 5, 7, 1, 4}))
}
