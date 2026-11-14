impl Solution {
    pub fn is_palindrome(s: String) -> bool {
        let chars: Vec<char> = s
            .to_lowercase()
            .chars()
            .filter(|c| c.is_alphanumeric())
            .collect();

        let mut left = 0;
        let mut right = chars.len();

        while left < right {
            right -= 1;

            if chars[left] != chars[right] {
                return false;
            }

            left += 1;
        }

        true
    }
}