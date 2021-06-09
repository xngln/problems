package main

import "fmt"

func twoSum(nums []int, target int) []int {
	complements := make(map[int]int)
	ans := []int{}

	for index, val := range nums {
		complement := target - val
		if cIndex, ok := complements[complement]; ok {
			ans = []int{index, cIndex}
		}
		complements[val] = index
	}

	return ans
}

func main() {
	nums := []int{2, 7, 11, 15}
	target := 9

	ans := twoSum(nums, target)

	fmt.Printf("%v\n", ans)
}
