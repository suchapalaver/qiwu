pub struct Solution;

impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        let l = nums.len();
        if l == 0 {
            return -1;
        }

        let mut left = 0;
        let mut right = l - 1;

        while left <= right {
            let mid = left + (right - left) / 2;

            if nums[mid] == target {
                return mid as i32;
            }

            if nums[left] <= nums[mid] && target >= nums[left] && target < nums[mid] {
                right = mid - 1;
            } else if nums[mid] <= nums[right] && target > nums[mid] && target <= nums[right]
                || nums[left] <= nums[mid]
            {
                left = mid + 1;
            } else {
                right = mid - 1;
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
