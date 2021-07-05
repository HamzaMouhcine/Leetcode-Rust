//https://leetcode.com/problems/truncate-sentence/submissions/
impl Solution {
    pub fn truncate_sentence(s: String, k: i32) -> String {
        let list = s.split(" ");
        
        let mut truncate_str = String::from("");
        
        let mut idx = 0;
        for word in list {
            match idx < k-1 {
                true => truncate_str.push_str(&(format!("{} ",word)[..])),
                false => {
                    truncate_str.push_str(word);
                    break;
                   
                }
            }
            idx += 1;
        }
        
        truncate_str
    }
}