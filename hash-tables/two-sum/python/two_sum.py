# https://leetcode.com/problems/two-sum/

from typing import List

class Solution:
    def twoSum(self, nums: List[int], target: int) -> List[int]:
        complements = {}
        ans = []

        for index, val in enumerate(nums):
            complement = target - val

            if complement in complements:
                ans = [index, complements[complement]]
                break

            complements[val] = index

        return ans


def main():
    nums = [2, 7, 11, 15]
    target = 9

    print(Solution().twoSum(nums, target))


if __name__ == "__main__":
    main()
