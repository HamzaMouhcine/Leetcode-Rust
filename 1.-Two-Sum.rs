use std::collections::HashMap;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut map = HashMap::new();
        let mut ans = Vec::new();
        
        for i in 0..nums.len() {
            let tmp = map.entry(target - nums[i]).or_insert(-1);
            
            if *tmp != -1 {
                ans = vec![*tmp, i as i32];
            }

            map.insert(nums[i], i as i32);
        }
        
        ans
    }
}