//https://leetcode.com/problems/replace-all-digits-with-characters/
impl Solution {
    pub fn replace_digits(s: String) -> String {
        let mut replaced_string = String::from("");
        
        let mut prev = '0';
        for (mut i, mut c) in s.chars().enumerate() {
            if i%2 == 1 {
                c = (prev as u8 + (c as u8 - '0' as u8)) as char;
                replaced_string.push(c);
            } else {
                replaced_string.push(c);
                prev = c;
            }
        }
        
        replaced_string
    }
}