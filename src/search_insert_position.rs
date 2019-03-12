impl Solution {
    pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
        let mut index: i32 = 0;
        for i in 0..nums.len() {
            if nums[i] >= target {
                return index;
            } else {
                index += 1;
            }
        }
        return index;
    }
}

pub struct Solution;

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test() {
        assert_eq!(
            Solution::search_insert(vec![1, 3, 5, 6], 0),
            0
        );
    }
}
