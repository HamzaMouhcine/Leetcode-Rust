//https://leetcode.com/problems/number-of-rectangles-that-can-form-the-largest-square/
use std::cmp::min;

impl Solution {
    pub fn count_good_rectangles(rectangles: Vec<Vec<i32>>) -> i32 {
        let mut max_square_count = 0;
        let mut max_square_size = 0;
        
        let mut tmp_square = 0;
        for i in 0..rectangles.len() {
            tmp_square = min(rectangles[i][0], rectangles[i][1]);
            
            
            if tmp_square > max_square_size {
                max_square_size = tmp_square;
                max_square_count = 1;
            } else if tmp_square == max_square_size {
                max_square_count += 1;
            }
        }
        
        max_square_count
    }
}