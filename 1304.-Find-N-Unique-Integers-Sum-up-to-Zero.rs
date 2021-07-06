// https://leetcode.com/problems/find-n-unique-integers-sum-up-to-zero/submissions/

impl Solution {
    pub fn sum_zero(n: i32) -> Vec<i32> {
        let mut unique_array = Vec::with_capacity(n as usize);
        
        for i in 1..n {
            unique_array.push(i);
        }
        unique_array.push(-(n * (n - 1)) / 2);
        
        unique_array
    }
}