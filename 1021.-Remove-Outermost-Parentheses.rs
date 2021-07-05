//https://leetcode.com/problems/remove-outermost-parentheses/
impl Solution {
    pub fn remove_outer_parentheses(s: String) -> String {
        let mut remove_string = String::from("");
        
        let mut cnt = 0;
        for c in s.chars() {
            match c {
                '(' => {
                    cnt += 1;
                    match cnt {
                        1 => (),
                        _ => remove_string.push('(')
                    }
                },
                ')' => {
                    cnt -= 1;
                    match cnt {
                        0 => (),
                        _ => remove_string.push(')')
                    }
                },
                _ => ()
            }
        }
        
        remove_string
    }
}