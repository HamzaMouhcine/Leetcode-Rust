//https://leetcode.com/problems/decode-xored-array/
impl Solution {
    pub fn decode(encoded: Vec<i32>,mut first: i32) -> Vec<i32> {
        let mut original = vec![0; encoded.len() + 1];
        original[0] = first;
        
        for i in 0..encoded.len() {
            original[i + 1] = encoded[i] ^ original[i];
        }
        
        original
    }
}