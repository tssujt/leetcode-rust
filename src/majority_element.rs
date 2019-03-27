pub struct Solution;

impl Solution {
    pub fn majority_element(nums: Vec<i32>) -> i32 {
        let mut res: i32 = 0;
        let mut count: i32 = 0;
        for num in nums {
            if count == 0 {
                res = num;
                count += 1;
            } else {
                if num == res {
                    count += 1;
                } else {
                    count -= 1
                }
            }
        }

        res
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test() {
        assert_eq!(Solution::majority_element(vec![2, 2, 1, 1, 1, 2, 2]), 2);
    }
}
