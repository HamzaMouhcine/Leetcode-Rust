//https://leetcode.com/problems/check-if-the-sentence-is-pangram/submissions/
impl Solution {
    pub fn check_if_pangram(sentence: String) -> bool {
        let mut visited: Vec<bool> = vec![false; 26];
        
        let mut number_of_visited_letters = 0;
        for c in sentence.chars() {
            let c_value = c as i32 - 'a' as i32;
            if !visited[c_value as usize] {
                number_of_visited_letters += 1;
                visited[c_value as usize] = true;
            }
        }
        
        number_of_visited_letters == 26
    }
}