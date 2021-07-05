//https://leetcode.com/problems/maximum-nesting-depth-of-the-parentheses/submissions/
use std::cmp::max;

impl Solution {
    pub fn max_depth(s: String) -> i32 {
        let mut nesting_depth = 0;

        let mut tmp = 0;
        for c in s.chars() {
            match c {
                '(' => tmp += 1,
                ')' => tmp += -1,
                _ => ()
            }
            nesting_depth = max(nesting_depth, tmp);
        }

        nesting_depth
    }
}