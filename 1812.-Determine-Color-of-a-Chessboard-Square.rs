//https://leetcode.com/problems/determine-color-of-a-chessboard-square/
impl Solution {
    pub fn square_is_white(coordinates: String) -> bool {
        let position_parity = coordinates.chars().nth(0).unwrap() as u8 - 'a' as u8 
                            + coordinates.chars().nth(1).unwrap() as u8 - '1' as u8;
        
        position_parity%2 == 1
    }
}