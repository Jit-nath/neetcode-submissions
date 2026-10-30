class Solution:
    def twoSum(self, nums: List[int], target: int) -> List[int]:

        lookup = {}

        for i in range(len(nums)):
            difference = target - nums[i]

            if difference in lookup:
                return [lookup[difference], i]

            lookup[nums[i]] = i

        return [-1, -1]