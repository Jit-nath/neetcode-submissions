impl Solution {
    pub fn max_area(heights: Vec<i32>) -> i32 {
        let mut left = 0;
        let mut right = heights.len() - 1;
        let mut max_water = 0;

        while left < right {
            let width = (right - left) as i32;
            let height = heights[left].min(heights[right]);

            let area = width * height;
            max_water = max_water.max(area);

            if heights[left] < heights[right] {
                left += 1;
            } else {
                right -= 1;
            }
        }

        max_water
    }
}