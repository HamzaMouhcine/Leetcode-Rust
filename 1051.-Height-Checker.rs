//https://leetcode.com/problems/height-checker/submissions/

impl Solution {
    pub fn height_checker(heights: Vec<i32>) -> i32 {
        let mut sorted_array = Vec::with_capacity(heights.len());
        let mut freq_array = vec![0; 101];
        
        for height in &heights {
            freq_array[*height as usize] += 1;
        }
        
        for i in 1..101 {
            while freq_array[i] != 0 {
                sorted_array.push(i as i32);
                freq_array[i] -= 1;
            }
        }
        
        let mut wrong_placed_elements = 0;
        for i in 0..heights.len() {
            if heights[i] != sorted_array[i] {
                wrong_placed_elements += 1;
            }
        }
        
        wrong_placed_elements
    }
}