//https://leetcode.com/problems/self-dividing-numbers/submissions/

impl Solution {
    pub fn self_dividing_numbers(left: i32, right: i32) -> Vec<i32> {
        let mut self_dividing_numbers = Vec::new();
        
        for i in left..right+1 {
            let mut tmp = i;
            let mut good = true;
            
            while tmp != 0 {
                let digit = tmp%10;
                if digit == 0 || i%digit != 0 {
                    good = false;
                }
                tmp /= 10;
            }
            if good {
                self_dividing_numbers.push(i);
            }
        }
        
        self_dividing_numbers
    }
}