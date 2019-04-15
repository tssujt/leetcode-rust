use std::collections::HashMap;

impl Solution {
    pub fn contains_nearby_duplicate(nums: Vec<i32>, k: i32) -> bool {
        let mut map = HashMap::new();

        for (i, num) in nums.iter().enumerate() {
            if let Some(j) = map.insert(num, i) {
                if (i as i32) - (j as i32) <= k {
                    return true;
                }
            }
        }
        false
    }
}

pub struct Solution;

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test() {
        assert_eq!(
            Solution::contains_nearby_duplicate(vec![1, 2, 3, 1], 3),
            true
        );
        assert_eq!(
            Solution::contains_nearby_duplicate(vec![99, 99], 2),
            true
        );
    }
}
