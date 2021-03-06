impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let mut max_profit: i32 = 0;
        let len = prices.len();
        let mut i: usize = 0;

        while i < len - 1 {
            if prices[i] < prices[i + 1] {
                max_profit += prices[i + 1] - prices[i];
            }

            i += 1;
        }

        return max_profit;
    }
}

pub struct Solution;

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test() {
        assert_eq!(Solution::max_profit(vec![7, 1, 5, 3, 6, 4]), 7);
        assert_eq!(Solution::max_profit(vec![1, 2, 3, 4, 5]), 4);
        assert_eq!(Solution::max_profit(vec![7, 6, 4, 3, 1]), 0);
    }
}
