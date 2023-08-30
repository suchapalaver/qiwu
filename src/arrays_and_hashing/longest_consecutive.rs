use std::collections::{hash_map::Entry, HashMap};
pub struct Solution;

impl Solution {
    pub fn longest_consecutive(nums: Vec<i32>) -> i32 {
        let mut longest_consecutive = 0;
        let mut map: HashMap<i32, i32> = HashMap::with_capacity(nums.len());
        for num in nums {
            let (mut lo, mut hi) = (
                *map.get(&(num - 1)).unwrap_or(&num),
                *map.get(&(num + 1)).unwrap_or(&num),
            );

            let (has_lower, has_upper) =
                (map.contains_key(&(num - 1)), map.contains_key(&(num + 1)));

            if let Entry::Vacant(e) = map.entry(num) {
                match (has_lower, has_upper) {
                    (false, false) => {
                        e.insert(num);
                    }
                    (true, false) => {
                        e.insert(lo);
                        *map.get_mut(&lo).unwrap() = num;
                        hi = num;
                    }
                    (false, true) => {
                        e.insert(hi);
                        *map.get_mut(&hi).unwrap() = num;
                        lo = num;
                    }
                    (true, true) => {
                        e.insert(num);
                        *map.get_mut(&lo).unwrap() = hi;
                        *map.get_mut(&hi).unwrap() = lo;
                    }
                }

                longest_consecutive = longest_consecutive.max(i32::abs(hi - lo + 1));
            } else {
                longest_consecutive = longest_consecutive.max(1);
            }
        }

        longest_consecutive
    }
}

#[cfg(test)]
mod tests {
    use crate::arrays_and_hashing::longest_consecutive::Solution;

    #[test]
    fn longest_consecutive() {
        let result = Solution::longest_consecutive(vec![0, 3, 7, 2, 5, 8, 4, 6, 0, 1]);
        assert_eq!(result, 9);
        let result = Solution::longest_consecutive(vec![100, 4, 200, 1, 3, 2]);
        assert_eq!(result, 4);
        let result = Solution::longest_consecutive(vec![-2, -3, -3, 7, -3, 0, 5, 0, -8, -4, -1, 2]);
        assert_eq!(result, 5);
    }
}
