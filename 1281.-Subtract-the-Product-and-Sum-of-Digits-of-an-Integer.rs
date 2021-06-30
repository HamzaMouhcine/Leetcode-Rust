impl Solution {
    pub fn subtract_product_and_sum(n: i32) -> i32 {
        let mut digit_sum = 0;
        let mut digit_product = 1;

        let mut n = n;        
        while n != 0 {
            let digit = n%10;
            digit_sum += digit;
            digit_product *= digit;

            n /= 10;
        }
        
        digit_product - digit_sum
    }
}