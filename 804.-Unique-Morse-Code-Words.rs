//https://leetcode.com/problems/unique-morse-code-words/
use std::collections::HashMap;

impl Solution {
    pub fn unique_morse_representations(words: Vec<String>) -> i32 {
        let lettre_to_morse = [".-","-...","-.-.","-..",".","..-.","--.","....","..",".---","-.-",".-..","--","-.","---",".--.","--.-",".-.","...","-","..-","...-",".--","-..-","-.--","--.."];
        let mut word_to_morse:HashMap<String, bool> = HashMap::new();



        let mut unique_morse = 0;
        for word in &words {
            let mut new_decoded_word = String::new();
            for c in word.chars() {
                let c_index = c as usize - 'a' as usize;
                new_decoded_word.push_str(lettre_to_morse[c_index]);
            }

            let word_exist = word_to_morse.entry(new_decoded_word).or_insert(false);
            if *word_exist == false {
                unique_morse += 1;
                *word_exist = true;
            }
        }
    
        unique_morse
    }
}