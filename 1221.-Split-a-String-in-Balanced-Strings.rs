//https://leetcode.com/problems/split-a-string-in-balanced-strings/
impl Solution {
    pub fn balanced_string_split(s: String) -> i32 {
        let mut ans = 0;
        let mut cnt = 0;
        
        for c in s.chars() {
            cnt += match c {
                'R' => 1,
                'L' => -1,
                _ => 0
            };
            if cnt == 0 {
                ans += 1;
            }
        }
        
        ans
    }
}