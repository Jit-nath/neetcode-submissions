impl Solution {
    pub fn three_sum(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        nums.sort_unstable();

        let n = nums.len();
        let mut result: Vec<Vec<i32>> = Vec::new();

        for i in 0..n {
            // Skip duplicate first elements
            if i > 0 && nums[i] == nums[i - 1] {
                continue;
            }

            let mut left = i + 1;
            let mut right = n - 1;

            while left < right {
                let sum = nums[i] + nums[left] + nums[right];

                if sum == 0 {
                    result.push(vec![nums[i], nums[left], nums[right]]);

                    left += 1;
                    right -= 1;

                    // Skip duplicate second elements
                    while left < right && nums[left] == nums[left - 1] {
                        left += 1;
                    }

                    // Skip duplicate third elements
                    while left < right && nums[right] == nums[right + 1] {
                        right -= 1;
                    }
                } else if sum < 0 {
                    left += 1;
                } else {
                    right -= 1;
                }
            }
        }

        result
    }
}