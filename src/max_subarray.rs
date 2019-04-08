impl Solution {
    pub fn max_sub_array(nums: Vec<i32>) -> i32 {
        let mut max = i32::min_value();
        let mut cur = 0;
        for i in 0..nums.len() {
            cur += nums[i];
            max = i32::max(max, cur);
            if cur < 0 {
                cur = 0;
            }
        }
        max
    }
}

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_53() {
        assert_eq!(Solution::max_sub_array(vec![-2, 1, -3, 4, -1, 2, 1, -5, 4]), 6);
        assert_eq!(Solution::max_sub_array(vec![-8]), -8);
        assert_eq!(Solution::max_sub_array(vec![-8, -2]), -2);
    }
}
