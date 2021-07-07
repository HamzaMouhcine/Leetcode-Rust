//https://leetcode.com/problems/merge-strings-alternately/

impl Solution {
    pub fn merge_alternately(word1: String, word2: String) -> String {
        let word1_chars: Vec<char> = word1.chars().collect();
        let word2_chars: Vec<char> = word2.chars().collect();
        let max_len = word1_chars.len().max(word2_chars.len());

        let mut alt_string = String::new();
        for i in 0..max_len {
            if i < word1.len() {
                alt_string.push(word1_chars[i]);
            }
            if i < word2.len() {
                alt_string.push(word2_chars[i]);
            }
        }
        
        alt_string
    }
}