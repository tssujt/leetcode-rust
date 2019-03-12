impl Solution {
    pub fn is_palindrome(s: String) -> bool {
        let mut s_vec = s.chars().filter_map(|a| if a.is_ascii_alphanumeric() {
            Some(a.to_ascii_lowercase())
        } else { None }).collect::<Vec<_>>();
        let cloned_s_vec = s_vec.clone();
        s_vec.reverse();
        cloned_s_vec == s_vec
    }
}

pub struct Solution;

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test() {
        assert_eq!(
            Solution::is_palindrome(String::from("A man, a plan, a canal: Panama")),
            true
        );
    }
}
