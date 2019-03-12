impl Solution {
    pub fn is_happy(n: i32) -> bool {
        let mut nums: Vec<i32> = Vec::new();
        let mut _n = n;
        while _n != 1 {
            let mut t: i32 = 0;
            while _n > 0 {
                t += (_n % 10) * (_n % 10);
                _n /= 10;
            }
            _n = t;
            if nums.contains(&_n) {
                break;
            } else {
                nums.push(_n)
            }
        }
        _n == 1
    }
}

pub struct Solution;

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test() {
        assert_eq!(
            Solution::is_happy(19),
            true
        );
    }
}
