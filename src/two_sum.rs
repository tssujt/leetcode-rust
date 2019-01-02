impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut result: Vec<i32> = Vec::new();
        for (_i, num_1) in nums.iter().enumerate() {
            for (_j, num_2) in nums[_i + 1..].iter().enumerate() {
                if num_1 + num_2 == target {
                    result.push(_i as i32);
                    result.push((_j + _i + 1) as i32);
                    return result;
                }
            }
        }

        return result;
    }
}

pub struct Solution;

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test() {
        assert_eq!(Solution::two_sum(vec![2, 7, 11, 5], 9), vec![0, 1]);
    }
}
