//https://leetcode.com/problems/generate-a-string-with-characters-that-have-odd-counts/

impl Solution {
    pub fn generate_the_string(n: i32) -> String {
        let mut odd_count = String::from("");
        
        for i in 0..n-1 {
            odd_count.push('a');
        }
        match n%2 == 0 {
            true => odd_count.push('b'),
            false => odd_count.push('a')
        }
        
        odd_count
    }
}