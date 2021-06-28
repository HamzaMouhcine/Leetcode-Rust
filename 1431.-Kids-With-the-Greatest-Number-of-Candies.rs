impl Solution {
    pub fn kids_with_candies(candies: Vec<i32>, extra_candies: i32) -> Vec<bool> {
        let ans: Vec<bool> = Vec::new();
        
        let mut max_amount = 0;
        for i in 0..candies.len() {
            if max_amount < candies[i] {
                max_amount = candies[i];
            }
        }
        
        for i in 0..candies.len() {
            ans.push(candies[i] + extra_candies > max_amount);
        }
        
        ans
    }
}