// https://leetcode.com/problems/sorting-the-sentence/
impl Solution {
    pub fn sort_sentence(s: String) -> String {
        let list: Vec<&str> = s.split(" ").collect();
        let mut ans = String::from("");


        for position in  0..list.len() {
            for j in 0..list.len() {
                let c = *(&list[j][list[j].len()-1..list[j].len()].chars().next().unwrap()) as i32 - 48;
                if c as usize == position+1 {
                    let tmp = &list[j][..list[j].len() - 1];
                    if position+1 == list.len() {
                        ans.push_str(&format!("{}", tmp) as &str);
                    } else {
                        ans.push_str(&format!("{} ", tmp) as &str);
                    }
                }
            }
        }

        ans
    }
}