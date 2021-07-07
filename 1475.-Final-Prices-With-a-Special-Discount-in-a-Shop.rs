//https://leetcode.com/problems/final-prices-with-a-special-discount-in-a-shop/submissions/

impl Solution {
    pub fn final_prices(mut prices: Vec<i32>) -> Vec<i32> {
        let mut stack: Vec<i32> = Vec::new();
        stack.push(0);
        
        for i in (0..prices.len()).rev() {
            while stack[stack.len() - 1] > prices[i] {
                stack.pop();
            }
            
            stack.push(prices[i]);
            prices[i] -= stack[stack.len() - 2];
        }
        
        prices
    }
}