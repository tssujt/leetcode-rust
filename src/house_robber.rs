use std::cmp;

impl Solution {
    pub fn rob(nums: Vec<i32>) -> i32 {
        let mut now: i32 = 0;
        let mut last: i32 = 0;

        for num in nums {
            let tmp_now = now;

            now = cmp::max(num + last, now);
            last = tmp_now;
        }

        now
    }
}

pub struct Solution;
