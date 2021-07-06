//https://leetcode.com/problems/determine-if-string-halves-are-alike/

impl Solution {
    pub fn halves_are_alike(s: String) -> bool {
        let mut valid: Vec<bool> = vec![false; 200];
        valid['a' as usize] = true; valid['A' as usize] = true;
        valid['o' as usize] = true; valid['O' as usize] = true;
        valid['i' as usize] = true; valid['I' as usize] = true;
        valid['u' as usize] = true; valid['U' as usize] = true;
        valid['e' as usize] = true; valid['E' as usize] = true;

        let mut first_half_freq = 0;
        let mut second_half_freq = 0;

        for (idx, c) in s.chars().enumerate() {
            if !valid[c as usize] {
                continue;
            }

            match idx < s.len()/2 {
                true => first_half_freq += 1,
                false => second_half_freq += 1
            }
        }
        
        first_half_freq == second_half_freq
    }
}