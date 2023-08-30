use std::collections::HashMap;

pub struct Solution;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut map = HashMap::with_capacity(nums.len());
        for (i, num) in nums.iter().enumerate() {
            match map.get(&(target - num)) {
                Some(res) => return vec![*res, i as i32],
                _ => {
                    map.insert(num, i as i32);
                }
            }
        }
        vec![]
    }
}

#[cfg(test)]
mod tests {
    use crate::arrays_and_hashing::two_sum::Solution;

    #[test]
    fn two_sum() {
        let result = Solution::two_sum(vec![2, 7, 11, 15], 9);
        assert_eq!(&result, &[0, 1]);
    }
}
