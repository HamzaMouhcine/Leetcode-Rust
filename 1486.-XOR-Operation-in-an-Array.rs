//https://leetcode.com/problems/xor-operation-in-an-array/submissions/
impl Solution {
    pub fn xor_operation(n: i32,mut start: i32) -> i32 {
        let mut nums_xor = 0;
        
        for i in 0..n {
            nums_xor ^= start + 2*i;
        }
        
        nums_xor
    }
}