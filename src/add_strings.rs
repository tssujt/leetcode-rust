impl Solution {
    pub fn add_strings(num1: String, num2: String) -> String {
        let mut idx1: usize = num1.len();
        let mut idx2: usize = num2.len();
        let mut over = 0;
        let mut result = String::new();

        while idx1 > 0 || idx2 > 0 || over > 0 {
            let mut sum = 0;
            if idx1 > 0 {
                idx1 -= 1;
                sum += num1.chars().nth(idx1).unwrap().to_digit(10).unwrap();
            }
            if idx2 > 0 {
                idx2 -= 1;
                sum += num2.chars().nth(idx2).unwrap().to_digit(10).unwrap();
            }
            sum += over;
            if sum > 9 {
                over = 1;
                sum %= 10;
            } else {
                over = 0;
            }
            result = format!("{}{}", result, sum.to_string());
        }

        result.chars().rev().collect::<String>()
    }
}

pub struct Solution;
