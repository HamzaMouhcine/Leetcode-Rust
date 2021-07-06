//https://leetcode.com/problems/maximum-product-of-two-elements-in-an-array/submissions/
impl Solution {
    pub fn max_product(nums: Vec<i32>) -> i32 {
        let mut first_max = 0;
        let mut second_max = 0;
        
        for i in 0..nums.len() {
            if first_max < nums[i] {
                if second_max < first_max {
                    second_max = first_max;
                }
                first_max = nums[i];
            } else if second_max < nums[i] {
                second_max = nums[i];
            }
        }
        first_max -= 1;
        second_max -= 1;
        
        first_max * second_max
    }
}