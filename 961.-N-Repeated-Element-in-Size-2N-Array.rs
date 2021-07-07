//https://leetcode.com/problems/n-repeated-element-in-size-2n-array/submissions/
use rand::prelude::*;
use std::collections::HashSet;

impl Solution {
    pub fn repeated_n_times(nums: Vec<i32>) -> i32 {
        let mut rng = thread_rng();
        let mut set: HashSet<i32> = HashSet::new();
        let mut visited: Vec<bool> = vec![false; nums.len()];
        
        loop {
            let idx = rng.gen_range(0, nums.len());
            if !visited[idx] && set.contains(&nums[idx]) {
                return nums[idx];
            } else {
                set.insert(nums[idx]);
                visited[idx] = true;
            }
        }
        
        return -1;
    }
}