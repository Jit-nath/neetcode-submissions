impl Solution {
    pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
        let len = nums.len();
        let mut result = vec![1; len];

        // Calculate prefix products
        let mut prefix = 1;

        for i in 0..len {
            result[i] = prefix;
            prefix *= nums[i];
        }

        // Calculate suffix products
        let mut suffix = 1;

        for i in (0..len).rev() {
            result[i] *= suffix;
            suffix *= nums[i];
        }

        result
    }
}