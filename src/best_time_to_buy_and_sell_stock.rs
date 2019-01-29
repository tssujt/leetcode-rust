impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let mut max_profit: i32 = 0;
        let mut min_price = i32::max_value();

        for price in prices {
            min_price = price.min(min_price);
            max_profit = (price - min_price).max(max_profit);
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
        assert_eq!(Solution::max_profit(vec![7, 1, 5, 3, 6, 4]), 5);
        assert_eq!(Solution::max_profit(vec![7, 6, 4, 3, 1]), 0);
    }
}
