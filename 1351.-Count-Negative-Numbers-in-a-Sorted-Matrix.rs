//https://leetcode.com/problems/count-negative-numbers-in-a-sorted-matrix/

impl Solution {
    pub fn count_negatives(grid: Vec<Vec<i32>>) -> i32 {
        let mut count_negative = 0;
        
        for i in 0..grid.len() {
            for j in 0..grid[i].len() {
                count_negative += if grid[i][j] < 0 {1} else {0};
            }
        }
        
        count_negative
    }
}