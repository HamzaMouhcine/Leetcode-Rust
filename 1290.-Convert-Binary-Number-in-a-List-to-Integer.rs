//https://leetcode.com/problems/convert-binary-number-in-a-linked-list-to-integer/
impl Solution {
    pub fn get_decimal_value(mut head: Option<Box<ListNode>>) -> i32 {
        let mut stack: Vec<i32> = Vec::new();
        let mut next: Option<Box<ListNode>> = None;
        next = head.clone();
        
        while !next.is_none() {
            stack.push(next.clone().unwrap().val);
            next = next.unwrap().next;
        }
        
        let mut number = 0;
        let mut mult = 1;
        while !stack.is_empty() {
            number += mult * stack.pop().unwrap();
            mult *= 2;
        }
        
        number
    }
}