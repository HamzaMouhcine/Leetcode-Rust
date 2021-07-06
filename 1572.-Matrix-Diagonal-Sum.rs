// https://leetcode.com/problems/matrix-diagonal-sum/
impl Solution {
    pub fn diagonal_sum(mat: Vec<Vec<i32>>) -> i32 {
        let mut diagonal_sum = 0;
        
        for i in 0..mat.len() {
            diagonal_sum += mat[i][i] + mat[mat.len()-i-1][i];
        }
        
        if (mat.len()%2 == 1) {
            diagonal_sum -= mat[mat.len()/2][mat.len()/2];
        }
        
        diagonal_sum
    }
}