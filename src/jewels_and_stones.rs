impl Solution {
    pub fn num_jewels_in_stones(j: String, s: String) -> i32 {
        let mut num: i32 = 0;
        for _j in j.chars() {
            for _s in s.chars() {
                if _s == _j {
                    num += 1;
                }
            }
        }

        num
    }
}
