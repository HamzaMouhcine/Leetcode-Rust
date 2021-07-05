//https://leetcode.com/problems/minimum-time-visiting-all-points/submissions/
use std::cmp::min;
use std::cmp::max;

impl Solution {
    pub fn min_time_to_visit_all_points(points: Vec<Vec<i32>>) -> i32 {
        let mut distance = 0;
        
        for i in 0..points.len()-1 {
            distance += distance_2_points(&points[i], &points[i+1]);
        }
        
        distance
    }
}

pub fn distance_2_points(point1: &Vec<i32>, point2: &Vec<i32>) -> i32 {
    let difX = (point1[0] - point2[0]).abs();
    let difY = (point1[1] - point2[1]).abs();
    
    let minDif = min(difX, difY);
    let maxDif = max(difX, difY);
    
    minDif + (maxDif-minDif).abs()
}