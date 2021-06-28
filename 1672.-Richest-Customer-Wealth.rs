impl Solution {
    pub fn maximum_wealth(accounts: Vec<Vec<i32>>) -> i32 {
        let mut ans = 0;
        
        for i in 0..accounts.len() {
            let mut tmp_sum = 0;
            for j in 0..accounts[i].len() {
                tmp_sum += accounts[i][j];
            }
            
            if tmp_sum > ans {
                ans = tmp_sum;
            }
        }
        
        ans
    }
}