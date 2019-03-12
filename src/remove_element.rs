impl Solution {
    pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
        let mut i = 0;
        for index in 0..nums.len() {
            if nums[index] != val {
                nums[i] = nums[index];
                i += 1;
            }
        }
        i as i32
    }
}

pub struct Solution;

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test() {
        let mut vec = vec![0, 1, 2, 2, 3, 0, 4, 2];
        assert_eq!(
            Solution::remove_element(&mut vec, 2),
            5
        );
    }
}
