impl Solution {
    pub fn find_kth_largest(nums: Vec<i32>, k: i32) -> i32 {
        let mut k_elements: Vec<i32> = Vec::new();

        for num in nums {
            if (k_elements.len() as i32) < k || k_elements[0] < num {
                let mut i: i32 = 0;
                for element in &k_elements {
                    if num >= *element {
                        i += 1;
                    } else {
                        break;
                    }
                }
                k_elements.insert(i as usize, num);
            }

            if k_elements.len() as i32 > k {
                k_elements.remove(0);
            }
        }
        k_elements[0]
    }
}


pub struct Solution;

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test() {
        assert_eq!(
            Solution::find_kth_largest(vec![3, 2, 3, 1, 2, 4, 5, 5, 6], 4),
            4
        );
    }
}
