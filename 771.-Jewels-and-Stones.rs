impl Solution {
    pub fn num_jewels_in_stones(jewels: String, stones: String) -> i32 {
        let mut valid_chars = [false; 123];
        
        for c in jewels.chars() {
            valid_chars[c as usize] = true;
        }
        
        let mut ans = 0;
        for c in stones.chars() {
            ans += match valid_chars[(c-'A') as usize] {
                true => 1,
                false => 0
            }
        }
        
        ans
    }
}