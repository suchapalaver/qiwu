pub struct Solution;

impl Solution {
    // # Python Solution
    //
    // def maxProfit(self, prices: List[int]) -> int:
    //     left = 0
    //     right = 1

    //     max_profit = 0

    //     while right < len(prices):
    //         if prices[right] > prices[left]:
    //             profit = prices[right] - prices[left]
    //             max_profit = max(max_profit, profit)
    //         else:
    //             left = right

    //         right += 1

    //     return max_profit
    pub fn max_profit_sliding_window(prices: Vec<i32>) -> i32 {
        let mut left = 0;
        let mut right = 1;

        let mut max_profit = 0;

        while right < prices.len() {
            if prices[left] < prices[right] {
                let profit = prices[right] - prices[left];
                max_profit = max_profit.max(profit);
            } else {
                left = right;
            }
            right += 1;
        }
        max_profit
    }

    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let (mut ans, mut cur_sum): (i32, i32) = (0, 0);

        for i in 0..prices.len() - 1 {
            cur_sum += prices[i + 1] - prices[i];

            if cur_sum < 0 {
                cur_sum = 0;
            }

            ans = ans.max(cur_sum);
        }
        ans
    }

    pub fn max_profit2(prices: Vec<i32>) -> i32 {
        if prices.is_empty() {
            return 0;
        }

        let mut max_profit = 0;
        let mut min_price = prices[0];

        for price in prices {
            min_price = min_price.min(price);
            max_profit = max_profit.max(price - min_price);
        }

        max_profit
    }
}

#[cfg(test)]
mod tests {
    use crate::sliding_window::max_profit::Solution;

    #[test]
    fn max_profit_sliding_window() {
        let result = Solution::max_profit_sliding_window(vec![1]);
        assert_eq!(result, 0);
        let result = Solution::max_profit_sliding_window(vec![7, 1]);
        assert_eq!(result, 0);
        let result = Solution::max_profit_sliding_window(vec![1, 5]);
        assert_eq!(result, 4);
        let result = Solution::max_profit_sliding_window(vec![2, 1, 4]);
        assert_eq!(result, 3);
        let result = Solution::max_profit_sliding_window(vec![7, 1, 5, 3, 6, 4]);
        assert_eq!(result, 5);
        let result = Solution::max_profit_sliding_window(vec![7, 6, 4, 3, 1]);
        assert_eq!(result, 0);
        let result = Solution::max_profit_sliding_window(vec![2, 1, 2, 1, 0, 1, 2]);
        assert_eq!(result, 2);
        let result = Solution::max_profit_sliding_window(vec![1, 2, 11, 4, 7]);
        assert_eq!(result, 10);
    }
    
    #[test]
    fn is_valid() {
        let result = Solution::max_profit(vec![1]);
        assert_eq!(result, 0);
        let result = Solution::max_profit(vec![7, 1]);
        assert_eq!(result, 0);
        let result = Solution::max_profit(vec![1, 5]);
        assert_eq!(result, 4);
        let result = Solution::max_profit(vec![2, 1, 4]);
        assert_eq!(result, 3);
        let result = Solution::max_profit(vec![7, 1, 5, 3, 6, 4]);
        assert_eq!(result, 5);
        let result = Solution::max_profit(vec![7, 6, 4, 3, 1]);
        assert_eq!(result, 0);
        let result = Solution::max_profit(vec![2, 1, 2, 1, 0, 1, 2]);
        assert_eq!(result, 2);
        let result = Solution::max_profit(vec![1, 2, 11, 4, 7]);
        assert_eq!(result, 10);
    }

    #[test]
    fn max_profit2() {
        let result = Solution::max_profit2(vec![1]);
        assert_eq!(result, 0);
        let result = Solution::max_profit2(vec![7, 1]);
        assert_eq!(result, 0);
        let result = Solution::max_profit2(vec![1, 5]);
        assert_eq!(result, 4);
        let result = Solution::max_profit2(vec![2, 1, 4]);
        assert_eq!(result, 3);
        let result = Solution::max_profit2(vec![7, 1, 5, 3, 6, 4]);
        assert_eq!(result, 5);
        let result = Solution::max_profit2(vec![7, 6, 4, 3, 1]);
        assert_eq!(result, 0);
        let result = Solution::max_profit2(vec![2, 1, 2, 1, 0, 1, 2]);
        assert_eq!(result, 2);
        let result = Solution::max_profit2(vec![1, 2, 11, 4, 7]);
        assert_eq!(result, 10);
    }
}
