//https://leetcode.com/problems/sum-of-all-subset-xor-totals/submissions/
impl Solution {
    pub fn subset_xor_sum(nums: Vec<i32>) -> i32 {
        let mut xor_sum = 0;

        for mask in 0..(1<<nums.len()) {
            let mut tmp_xor = 0;
            for i in 0..nums.len() {
                if (mask & (1 << i)) != 0 {
                    tmp_xor ^= nums[i];
                }
            }
            xor_sum += tmp_xor;
        }
        
        xor_sum
    }
}