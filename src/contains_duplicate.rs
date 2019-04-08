use std::collections::HashSet;

impl Solution {
    pub fn contains_duplicate(nums: Vec<i32>) -> bool {
        let mut hashset = HashSet::new();
        for num in nums {
            if hashset.contains(&num) {
                return true;
            }

            hashset.insert(num);
        }
        false
    }
}

pub struct Solution;
