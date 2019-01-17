impl Solution {
    pub fn longest_common_prefix(strs: Vec<String>) -> String {
        let mut index = 0;
        if let Some(_str) = strs.iter().nth(0) {
            for (i, c) in _str.chars().enumerate() {
                for s in strs.iter().skip(1) {
                    if let Some(v) = s.chars().nth(i) {
                        if v != c {
                            return _str[0..index].to_string();
                        }
                    } else {
                        return _str[0..index].to_string();
                    }
                }
                index += 1;
            }
            _str[0..index].to_string()
        } else {
            String::new()
        }
    }
}

pub struct Solution;

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test() {
        assert_eq!(Solution::longest_common_prefix(
            vec![String::from("flower"), String::from("flow"), String::from("flight")]),
                   String::from("fl"));
        assert_eq!(Solution::longest_common_prefix(
            vec![String::from(""), String::from("flow"), String::from("flight")]),
                   String::from(""));
    }
}
