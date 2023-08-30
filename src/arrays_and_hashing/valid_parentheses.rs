pub struct Solution;

impl Solution {
    pub fn is_valid(s: String) -> bool {
        let mut v = vec![];
        for c in s.chars() {
            match c {
                '{' | '[' | '(' => v.push(c),
                '}' | ']' | ')' => match v.pop() {
                    Some(top) if Self::is_matching(c, top) => {
                        continue;
                    }
                    _ => return false,
                },
                _ => unreachable!(),
            }
        }
        v.is_empty()
    }

    fn is_matching(c: char, top: char) -> bool {
        (top == '{' && c == '}') | (top == '[' && c == ']') | (top == '(' && c == ')')
    }
}

#[cfg(test)]
mod tests {
    use crate::arrays_and_hashing::valid_parentheses::Solution;

    #[test]
    fn is_valid() {
        let result = Solution::is_valid("()".to_string());
        assert!(result);
        let result = Solution::is_valid("()[]{}".to_string());
        assert!(result);
        let result = Solution::is_valid("(]".to_string());
        assert!(!result);
        let result = Solution::is_valid("(".to_string());
        assert!(!result);
    }
}
