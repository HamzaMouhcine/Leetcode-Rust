//https://leetcode.com/problems/hamming-distance/submissions/

impl Solution {
    pub fn hamming_distance(x: i32, y: i32) -> i32 {
        let mut hamming_dist = 0;
        
        for i in 0..31 {
            if ((1<<i) & x) != ((1<<i) & y) {
                hamming_dist += 1;
            }
        }
        
        hamming_dist
    }
}