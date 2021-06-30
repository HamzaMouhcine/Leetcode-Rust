//https://leetcode.com/problems/shuffle-string/
use std::str;

impl Solution {
    pub fn restore_string(s: String, indices: Vec<i32>) -> String {
        let mut shuffled_string = vec!['a'; indices.len()]; // change to s.len() later
        
        for (i, c) in s.chars().enumerate() {
            shuffled_string[indices[i] as usize] = c;
        }
        
        shuffled_string.iter().collect()
    }
}