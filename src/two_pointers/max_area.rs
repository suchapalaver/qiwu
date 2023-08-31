pub struct Solution;

impl Solution {
    pub fn max_area(height: Vec<i32>) -> i32 {
        let mut volume = 0;

        if !height.is_empty() {
            let mut left: usize = 0;
            let mut right = height.len() - 1;

            while right > left {
                let current_level = height[left].min(height[right]);
                let current_vol = current_level * (right - left) as i32;

                if current_vol > volume {
                    volume = current_vol;
                }

                if height[left] < (height[right]) {
                    left += 1;
                } else {
                    right -= 1;
                }
            }
        }

        volume
    }
}

#[cfg(test)]
mod tests {
    use crate::two_pointers::max_area::Solution;

    #[test]
    fn is_palindrome() {
        let result = Solution::max_area(vec![1, 8, 6, 2, 5, 4, 8, 3, 7]);
        assert_eq!(result, 49);
        let result = Solution::max_area(vec![1, 1]);
        assert_eq!(result, 1);
        let result = Solution::max_area(vec![]);
        assert_eq!(result, 0);
        let result = Solution::max_area(vec![5]);
        assert_eq!(result, 0);
        let result = Solution::max_area(vec![2, 3, 4, 5, 18, 17, 6]);
        assert_eq!(result, 17);
    }
}
