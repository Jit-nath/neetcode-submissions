use std::collections::HashSet;

impl Solution {
    pub fn has_duplicate(nums: Vec<i32>) -> bool {
        let mut seen_list = HashSet::new();

        for num in nums {
            if !seen_list.insert(num) {
                return true;
            }
        }

        false
    }
}