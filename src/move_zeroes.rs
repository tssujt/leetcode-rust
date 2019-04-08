impl Solution {
    pub fn move_zeroes(nums: &mut Vec<i32>) {
        let l = nums.len();

        let mut i: usize = 0;
        for _i in 0..l {
            if nums[_i] != 0 {
                nums[i] = nums[_i];
                i += 1;
            }
        }
        for _i in 0..l - i {
            nums[l - _i - 1] = 0;
        }
    }
}

pub struct Solution;
