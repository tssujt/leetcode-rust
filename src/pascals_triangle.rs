impl Solution {
    pub fn generate(num_rows: i32) -> Vec<Vec<i32>> {
        let mut v: Vec<Vec<i32>> = Vec::new();

        if num_rows < 1 {
            return v;
        }
        let mut current = vec![1i32];
        for _ in 0..num_rows {
            let mut next = vec![1; current.len() + 1];
            for i in 1..current.len() {
                next[i] = current[i - 1] + current[i];
            }

            v.push(current);
            current = next;
        }

        v
    }
}

pub struct Solution;
