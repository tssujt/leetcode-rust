impl Solution {
    pub fn matrix_reshape(nums: Vec<Vec<i32>>, r: i32, c: i32) -> Vec<Vec<i32>> {
        let m = nums.len();
        let n = nums[0].len();
        if m * n != r as usize * c as usize {
            return nums;
        }

        let mut v: Vec<Vec<i32>> = Vec::new();

        for i in 0..r {
            let mut row = Vec::new();
            for j in 0..c {
                let k: usize = i as usize * c as usize + j as usize;
                row.push(nums[k / n][k % n]);
            }
            v.push(row);
        }

        v
    }
}

pub struct Solution;
