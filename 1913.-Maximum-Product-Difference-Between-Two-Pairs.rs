//https://leetcode.com/problems/maximum-product-difference-between-two-pairs/submissions/
impl Solution {
    pub fn max_product_difference(nums: Vec<i32>) -> i32 {
        let max_first = next_max(&nums, 100000);
        let max_second = next_max(&nums, max_first);
        
        let min_first = next_min(&nums, 100000);
        let min_second = next_min(&nums, min_first);
        
        let mut max_product = nums[max_first] * nums[max_second];
        let mut min_product = nums[min_first] * nums[min_second];
        
        return max_product - min_product;
    }
}

fn next_max(nums: &Vec<i32>, used: usize) -> usize {
    let mut max_val = -1;
    let mut max_idx = 0 as usize;
    for i in 0..nums.len() {
        if i == used {
            continue;
        }
        
        if max_val < nums[i] {
            max_val = nums[i];
            max_idx = i;
        }
    }
    
    return max_idx;
}

fn next_min(nums: &Vec<i32>, used: usize) -> usize {
    let mut min_val = 100000;
    let mut min_idx = 0 as usize;
    
    for i in 0..nums.len() {
        if i == used {
            continue;
        }
        
        if min_val > nums[i] {
            min_val = nums[i];
            min_idx = i;
        }
    }
    
    return min_idx;
}