//https://leetcode.com/problems/decrypt-string-from-alphabet-to-integer-mapping/
impl Solution {
    pub fn freq_alphabets(s: String) -> String {
        let s_bytes = s.as_bytes();
        let mut decrypted_string = String::from("");
        
        let mut i = 0;
        while i < s.len() {
            match i < s.len()-2 {
                true => {
                    if s_bytes[i+2] == '#' as u8 {
                        decrypted_string.push(((s_bytes[i] - '0' as u8 ) * 10u8 + s_bytes[i+1] - '1' as u8 + 'a' as u8)  as char);
                        i += 2;
                    } else {
                        decrypted_string.push((s_bytes[i] - '1' as u8 + 'a' as u8) as char);
                    }
                },
                false => {
                    decrypted_string.push((s_bytes[i] - '1' as u8 + 'a' as u8) as char);
                }
            }
            
            i += 1;
        }
        
        decrypted_string
    }
}