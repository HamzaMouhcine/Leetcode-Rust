//https://leetcode.com/problems/cells-with-odd-values//-in-a-matrix/submissions/
impl Solution {
    pub fn odd_cells(m: i32, n: i32, indices: Vec<Vec<i32>>) -> i32 {
        let mut row = vec![0; m as usize];
        let mut column = vec![0; n as usize];
        let mut odd_cells = 0;
        
        for i in 0..indices.len() {
            row[indices[i][0] as usize] += 1;
            column[indices[i][1] as usize] += 1;
        }
        
        for i in 0..m as usize {
            for j in 0..n as usize {
                if (row[i] + column[j]) %2 == 1 {
                    odd_cells += 1;
                }
            }
        }
        
        odd_cells
    }
}