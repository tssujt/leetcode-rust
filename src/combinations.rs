impl Solution {
    pub fn combine(n: i32, k: i32) -> Vec<Vec<i32>> {
        let mut res: Vec<Vec<i32>> = Vec::new();
        Solution::backtrack(1, n, k, &mut res, vec![]);
        res
    }

    fn backtrack(start: i32, end: i32, k: i32, res: &mut Vec<Vec<i32>>, path: Vec<i32>) {
        if k == 0 {
            res.push(path);
            return;
        }
        if end - start + 1 < k {
            return;
        }
        for i in start..end + 1 {
            let mut cloned_path = path.clone();
            cloned_path.push(i);
            Solution::backtrack(i + 1, end, k - 1, res, cloned_path);
        }
    }
}

pub struct Solution;
