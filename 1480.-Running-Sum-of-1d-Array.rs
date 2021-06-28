impl Solution {
    pub fn running_sum(nums: Vec<i32>) -> Vec<i32> {
        let mut ans:Vec<i32> = Vec::new();
        let mut last = 0;

        for i in 0..nums.len() {
            last += nums[i];
            ans.push(last);
        }
        
        ans
    }
}