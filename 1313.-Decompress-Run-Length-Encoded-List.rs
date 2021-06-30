// https://leetcode.com/problems/decompress-run-length-encoded-list/submissions/
impl Solution {
    pub fn decompress_rl_elist(nums: Vec<i32>) -> Vec<i32> {
        let mut ans: Vec<i32> = Vec::new();
        
        for i in 0..nums.len()/2 {
            for j in 0..nums[i*2] {
                ans.push(nums[i*2 + 1]);
            }
        }
        
        ans
    }
}