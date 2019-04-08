impl Solution {
    pub fn reverse_str(s: String, k: i32) -> String {
        let s = s.into_bytes();
        let mut new_string: Vec<u8> = Vec::new();

        for chunk in s.chunks(2 * k as usize) {
            let mut _i = 0;
            for sub_chunk in chunk.chunks(k as usize) {
                if _i == 0 {
                    new_string.extend(sub_chunk.iter().rev());
                } else {
                    new_string.extend(sub_chunk.iter());
                }
                _i += 1;
            }
        }
        unsafe { String::from_utf8_unchecked(new_string) }
    }
}

pub struct Solution;
