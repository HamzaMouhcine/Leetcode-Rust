//https://leetcode.com/problems/sort-array-by-parity/submissions/
impl Solution {
    pub fn sort_array_by_parity(mut nums: Vec<i32>) -> Vec<i32> {
        let mut left = 0;
        let mut right = (nums.len()-1) as i32;
        
        while left < right {
            while (left as usize) < nums.len() && nums[left as usize]%2 == 0 {
                left += 1;
            }
            while right >= 0 && nums[right as usize]%2 == 1 {
                right -= 1;
            }
            
            if left < right {
                nums.swap(left as usize, right as usize);
            }
        }
        
        nums
    }
}