//https://leetcode.com/problems/find-the-highest-altitude/
use std::cmp::max;

impl Solution {
    pub fn largest_altitude(gain: Vec<i32>) -> i32 {
        let mut highest_altitude = 0;
        
        let mut current_altitude = 0;
        for int in &gain {
            current_altitude += int;
            highest_altitude = max(highest_altitude, current_altitude);
        }
        
        highest_altitude
    }
}