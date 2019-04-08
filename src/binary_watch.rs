static WATCHES: [i32; 10] = [1, 2, 4, 8, 16, 32, 60, 2 * 60, 4 * 60, 8 * 60];

impl Solution {
    pub fn read_binary_watch(num: i32) -> Vec<String> {
        let mut res: Vec<String> = Vec::new();
        Solution::backtrace(0, 9, num, &mut res, vec![]);
        res
    }

    fn backtrace(start: i32, end: i32, n: i32, res: &mut Vec<String>, time_set: Vec<i32>) {
        if n == 0 {
            let mut hours: i32 = time_set.iter().filter(|&&h| h >= 60).sum();
            hours /= 60;
            let minutes: i32 = time_set.iter().filter(|&&h| h < 60).sum();

            if hours > 11 || minutes > 59 {
                return;
            }
            let _minutes = if minutes < 10 { format!("0{}", minutes) } else { format!("{}", minutes) };
            res.push(format!("{}:{}", hours, _minutes));
            return;
        }
        if end - start + 1 < n {
            return;
        }

        for i in start..end + 1 {
            let mut cloned = time_set.clone();
            cloned.push(WATCHES[i as usize]);
            Solution::backtrace(i + 1, end, n - 1, res, cloned);
        }
    }
}

pub struct Solution;
