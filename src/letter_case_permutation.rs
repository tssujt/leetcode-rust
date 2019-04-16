impl Solution {
    pub fn letter_case_permutation(s: String) -> Vec<String> {
        let mut res: Vec<String> = Vec::new();
        let l = s.len() as i32;
        let chars = s.chars().collect();
        Solution::backtrack(&mut Vec::new(), &chars, 0, &mut res);
        res
    }

    fn backtrack(prefix: &mut Vec<char>, chars: &Vec<char>, i: usize, result: &mut Vec<String>) {
        let l = chars.len();
        if i == l {
            result.push(prefix.iter().collect());
            return;
        }
        for _i in i..l {
            let c = chars[_i];
            if !c.is_ascii_alphabetic() {
                prefix.push(c);
            } else {
                let mut prefix1 = prefix.clone();
                prefix1.push(c);
                Solution::backtrack(&mut prefix1, chars, _i + 1, result);

                let mut prefix2 = prefix.clone();
                if c.is_uppercase() {
                    prefix2.push(c.to_ascii_lowercase());
                } else {
                    prefix2.push(c.to_ascii_uppercase());
                }
                Solution::backtrack(&mut prefix2, chars, _i + 1, result);
                return;
            }
        }
        result.push(prefix.iter().collect());
    }
}

pub struct Solution;

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test() {
        assert_eq!(
            Solution::letter_case_permutation(String::from("a1b2")),
            vec!["a1b2", "a1B2", "A1b2", "A1B2"]
        );
    }
}
