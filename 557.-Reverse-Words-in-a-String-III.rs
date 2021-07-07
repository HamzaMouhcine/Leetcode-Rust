//https://leetcode.com/problems/reverse-words-in-a-string-iii/

impl Solution {
    pub fn reverse_words(s: String) -> String {
        let mut reverse_words = String::new();

        let words = s.split(" ").collect::<Vec<&str>>();

        for (idx, word) in words.iter().enumerate() {
            if idx < words.len()-1 {
                reverse_words.push_str(&format!("{} ",word.chars().rev().collect::<String>()));
            } else {
                reverse_words.push_str(&format!("{}",word.chars().rev().collect::<String>()));
            }
        }
        
        reverse_words
    }
}