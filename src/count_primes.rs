impl Solution {
    pub fn count_primes(n: i32) -> i32 {
        let mut primes: Vec<bool> = Vec::new();
        for _ in 0..n + 1 {
            primes.push(true);
        }
        let mut count = 0;
        if n > 0 {
            primes[0] = false;
        }
        if n > 1 {
            primes[1] = false;
        }

        for i in 2..n as usize {
            if primes[i] {
                count += 1;
                for j in 2..(n / (i as i32) + 1) {
                    primes[i * (j as usize)] = false;
                }
            }
        }
        count
    }
}

pub struct Solution;
