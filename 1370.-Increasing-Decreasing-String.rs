//https://leetcode.com/problems/increasing-decreasing-string/submissions/
impl Solution {
    pub fn sort_string(s: String) -> String {
        let mut freq = [0; 26];
        
        for c in s.chars() {
            freq[c as usize - 'a' as usize] += 1;
        }
        
        let mut sorted_string = String::from("");
        let mut operation_number = 0;
        while sorted_string.len() < s.len() {
            match operation_number % 2 {
                0 => {
                    for i in 0usize..26 {
                        if freq[i] != 0 {
                            freq[i] -= 1;
                            sorted_string.push((i as u8+'a' as u8) as char);
                        }
                    }
                },
                1 => {
                    for i in (0usize..26).rev() {
                        if freq[i] != 0 {
                            freq[i] -= 1;
                            sorted_string.push((i as u8+'a' as u8) as char);
                        }
                    }
                },
                _ => ()
            }
            operation_number += 1;
        }
        
        sorted_string
    }
}