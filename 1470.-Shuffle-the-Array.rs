impl Solution {
    pub fn shuffle(nums: Vec<i32>, n: i32) -> Vec<i32> {
        let mut ans = Vec::new();

        for i in 0..nums.len()/2 {
            ans.push(nums[i]);
            ans.push(nums[i + nums.len()/2]);
        }
        
        ans
    }
}