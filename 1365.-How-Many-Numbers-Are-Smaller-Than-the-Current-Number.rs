// https://leetcode.com/problems/how-many-numbers-are-smaller-than-the-current-number/

impl Solution {
    pub fn smaller_numbers_than_current(nums: Vec<i32>) -> Vec<i32> {
        let mut cnt = [0; 101];
        
        for i in 0..nums.len() {
             cnt[nums[i] as usize] += 1;
        }
        
        for i in 1..101 {
            cnt[i] += cnt[i-1];
        }
        
        let mut less_than_me = vec![0; nums.len()];
        for i in 0..nums.len() { 
            less_than_me[i] = match nums[i] {
                0 => 0,
                _ => cnt[(nums[i] - 1) as usize]
            }
        } 
        
        less_than_me
    }
}