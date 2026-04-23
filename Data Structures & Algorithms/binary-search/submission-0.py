from typing import List

class Solution:
    def search(self, nums: List[int], target: int) -> int:
        return self.bs(nums, target, 0, len(nums) - 1)

    def bs(self, nums, target, left, right):
        if left > right:
            return -1

        mid = (left + right) // 2

        if nums[mid] == target:
            return mid
        elif target < nums[mid]:
            return self.bs(nums, target, left, mid - 1)
        else:
            return self.bs(nums, target, mid + 1, right)