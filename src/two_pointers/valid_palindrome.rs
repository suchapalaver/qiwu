pub struct Solution;

impl Solution {
    pub fn is_palindrome(s: String) -> bool {
        let alphanumeric_chars: Vec<char> = s
            .chars()
            .filter(|c| c.is_alphanumeric())
            .map(|c| c.to_ascii_lowercase())
            .collect();

        alphanumeric_chars
            .iter()
            .eq(alphanumeric_chars.iter().rev())
    }

    pub fn is_palindrome_two_pointers(s: String) -> bool {
        let alphanumeric_lowercase_chars = s
            .chars()
            .filter(|c| c.is_alphanumeric())
            .map(|c| c.to_ascii_lowercase())
            .collect::<Vec<char>>();

        if !alphanumeric_lowercase_chars.is_empty() {
            let mut front: usize = 0;
            let mut back = alphanumeric_lowercase_chars.len() - 1;

            while back > front {
                if alphanumeric_lowercase_chars[front] != alphanumeric_lowercase_chars[back] {
                    return false;
                } else {
                    front += 1;
                    back -= 1;
                }
            }
        }

        true
    }
}

#[cfg(test)]
mod tests {
    use crate::two_pointers::valid_palindrome::Solution;

    #[test]
    fn is_palindrome() {
        let result = Solution::is_palindrome("   ".to_string());
        assert!(result);
        let result = Solution::is_palindrome("A man, a plan, a canal: Panama".to_string());
        assert!(result);
        let result = Solution::is_palindrome("race a car".to_string());
        assert!(!result);
        let result = Solution::is_palindrome("0P".to_string());
        assert!(!result);
    }

    #[test]
    fn is_palindrome_two_pointers() {
        let result = Solution::is_palindrome_two_pointers("   ".to_string());
        assert!(result);
        let result =
            Solution::is_palindrome_two_pointers("A man, a plan, a canal: Panama".to_string());
        assert!(result);
        let result = Solution::is_palindrome_two_pointers("race a car".to_string());
        assert!(!result);
        let result = Solution::is_palindrome_two_pointers("0P".to_string());
        assert!(!result);
    }
}
