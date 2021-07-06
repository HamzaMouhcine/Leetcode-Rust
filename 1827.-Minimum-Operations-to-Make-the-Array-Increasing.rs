//https://leetcode.com/problems/minimum-operations-to-make-the-array-increasing/submissions/
impl Solution {
    pub fn min_operations(mut nums: Vec<i32>) -> i32 {
        let mut num_of_operations = 0;
        
        for i in 1..nums.len() {
            num_of_operations += 0.max(nums[i-1] - nums[i] + 1);            
            nums[i] += 0.max(nums[i-1] - nums[i] + 1);
        }
        
        num_of_operations
    }
}