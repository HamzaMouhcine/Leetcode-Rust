//https://leetcode.com/problems/count-items-matching-a-rule/submissions/
impl Solution {
    pub fn count_matches(items: Vec<Vec<String>>, rule_key: String, rule_value: String) -> i32 {
        let rule_key = &rule_key[..];
        let rule_type = match rule_key {
            "type" => 0,
            "color" => 1,
            "name" => 2,
            _ => 3
        };
        
        let mut ans = 0;
        for i in 0..items.len() {
            if items[i][rule_type as usize] == rule_value {
                ans += 1;
            }
        }
        
        ans
    }
}