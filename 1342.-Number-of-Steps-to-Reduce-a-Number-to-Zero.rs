// https://leetcode.com/problems/number-of-steps-to-reduce-a-number-to-zero/submissions/
impl Solution {
    pub fn number_of_steps(mut num: i32) -> i32 {
        oper(num)
    }
}

fn oper(mut num: i32) -> i32 {
    match num {
        0 => 0,
        _ => match num %2 {
            0 => 1 + oper(num / 2),
            _ => 1 + oper(num - 1)
        }
    }
}