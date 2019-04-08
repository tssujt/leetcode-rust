impl Solution {
    pub fn compress(chars: &mut Vec<char>) -> i32 {
        let mut i: i32 = 0;
        let mut count = 1;
        let n = chars.len();
        for j in 1..n + 1 {
            if j < n && chars[j] == chars[j - 1] {
                count += 1;
            } else {
                chars[i as usize] = chars[j - 1];
                i += 1;
                if count > 1 {
                    let s = count.to_string();
                    let s_vec: Vec<char> = s.chars().collect();
                    for c in s_vec {
                        chars[i as usize] = c;
                        i += 1;
                    }
                }
                count = 1;
            }
        }
        i
    }
}

pub struct Solution;
