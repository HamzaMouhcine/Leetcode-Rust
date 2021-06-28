impl Solution {
    pub fn defang_i_paddr(address: String) -> String {
        let mut ans = String::from("");
        
        for c in address.chars() {
            if c == '.' {
                ans.push_str("[.]");
            } else {
                ans.push(c);
            }
        }
        
        ans
    }
}