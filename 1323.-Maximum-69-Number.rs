//https://leetcode.com/problems/maximum-69-number/submissions/

impl Solution {
    pub fn maximum69_number (mut num: i32) -> i32 {
        let mut max_69 = 0;
        let mut last_change_dif = 0;
        let mut ten_power = 1;
        
        while num != 0 {
            let current_digit = num % 10;
            
            match current_digit {
                9 => max_69 += current_digit * ten_power,
                6 => {
                    max_69 += 9 * ten_power - last_change_dif;
                    last_change_dif = 3 * ten_power;
                },
                _ => ()
            }
            
            num /= 10;
            ten_power *= 10;
        }
        
        max_69
    }
}