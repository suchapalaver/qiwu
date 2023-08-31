pub struct Solution;

impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        let mut left = 0;
        let mut right = nums.len();

        while left < right {
            let mid = left + (right - left) / 2;

            match target.cmp(&nums[mid]) {
                std::cmp::Ordering::Less => right = mid,
                std::cmp::Ordering::Equal => return mid as _,
                std::cmp::Ordering::Greater => left = mid + 1,
            }
        }
        -1
    }
}

#[cfg(test)]
mod tests {
    use crate::binary_search::search::Solution;

    #[test]
    fn is_palindrome() {
        let result = Solution::search(vec![-1, 0, 3, 5, 9, 12], 9);
        assert_eq!(result, 4);
        let result = Solution::search(vec![-1, 0, 3, 5, 9, 12], 2);
        assert_eq!(result, -1);
        let result = Solution::search(vec![5], -5);
        assert_eq!(result, -1);
        let result = Solution::search(vec![5], 5);
        assert_eq!(result, 0);
        let result = Solution::search(vec![2, 5], 0);
        assert_eq!(result, -1);
    }
}
