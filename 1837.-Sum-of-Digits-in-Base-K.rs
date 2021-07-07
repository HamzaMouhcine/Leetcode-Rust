//https://leetcode.com/problems/sum-of-digits-in-base-k/submissions/

impl Solution {
    pub fn sum_base(mut n: i32, k: i32) -> i32 {
        let mut digits_sum = 0;
        
        while n != 0 {
            let q = n / k;
            digits_sum += n - q*k;
            n = q;
        }
        
        digits_sum
    }
}