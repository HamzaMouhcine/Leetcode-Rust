//https://leetcode.com/problems/count-the-number-of-consistent-strings/submissions/
impl Solution {
    pub fn count_consistent_strings(allowed: String, words: Vec<String>) -> i32 {
        let mut good: Vec<bool> = vec![false; 26];
        
        for c in allowed.chars() {
            let c_value = c as i32 - 'a' as i32;
            good[c_value as usize] = true;
        }
        
        let mut good_words = 0;
        for i in 0..words.len() {
            good_words += 1;
            for c in words[i].chars() {
                let c_value = c as i32 - 'a' as i32;
                if !good[c_value as usize] {
                    good_words -= 1;
                    break;
                }
            }
        }
        
        good_words
    }
}