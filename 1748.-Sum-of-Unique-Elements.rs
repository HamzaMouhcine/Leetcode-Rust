//https://leetcode.com/problems/sum-of-unique-elements/submissions/

impl Solution {
    pub fn sum_of_unique(nums: Vec<i32>) -> i32 {
        let mut freq = vec![0; 101];
        let mut unique_sum = 0;
        
        for num in &nums {
            freq[*num as usize] += 1;
            match freq[*num as usize] {
                1 => unique_sum += *num,
                2 => unique_sum -= *num,
                _ => ()
            }
        }
        
        unique_sum
    }
}