//https://leetcode.com/problems/robot-return-to-origin/submissions/

impl Solution {
    pub fn judge_circle(moves: String) -> bool {
        let mut x_position = 0;
        let mut y_position = 0;
        
        for c in moves.chars() {
            match c {
                'U' => y_position += 1,
                'D' => y_position -= 1,
                'R' => x_position += 1,
                'L' => x_position -= 1,
                _   => ()
            }
        }
        
        x_position == y_position && x_position == 0
    }
}