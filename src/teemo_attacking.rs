impl Solution {
    pub fn find_poisoned_duration(time_series: Vec<i32>, duration: i32) -> i32 {
        let l = time_series.len();
        if l == 0 {
            return 0;
        }

        let mut total_duration = 0i32;
        for i in 0..l - 1 {
            if time_series[i + 1] - time_series[i] + 1 > duration {
                total_duration += duration;
            } else {
                total_duration += time_series[i + 1] - time_series[i]
            }
        }

        total_duration + duration
    }
}

pub struct Solution;

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test() {
        assert_eq!(Solution::find_poisoned_duration(vec![1, 4], 2), 4);
    }
}
