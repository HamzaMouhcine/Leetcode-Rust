// https://leetcode.com/problems/find-numbers-with-even-number-of-digits/submissions/
impl Solution {
    pub fn find_numbers(mut nums: Vec<i32>) -> i32 {
        let mut even_digits_count = 0;
        
        let mut cnt_digits = 0;
        for int in &mut nums {
            cnt_digits = 0;
            while *int != 0 {
                cnt_digits += 1;
                *int = *int / 10;
            }

            if cnt_digits %2 == 0 {
                even_digits_count += 1;
            }
        }
        
        even_digits_count
    }
}