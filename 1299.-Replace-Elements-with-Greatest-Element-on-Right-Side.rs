//https://leetcode.com/problems/replace-elements-with-greatest-element-on-right-side/submissions/

impl Solution {
    pub fn replace_elements(mut arr: Vec<i32>) -> Vec<i32> {
        let mut max_right = -1;
        
        for i in (0..arr.len()).rev() {
            let tmp = arr[i];
            arr[i] = max_right;
            max_right = max_right.max(tmp);
        }
        
        return arr
    }
}