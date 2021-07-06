//https://leetcode.com/problems/flipping-an-image/submissions/
impl Solution {
    pub fn flip_and_invert_image(image: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let n = image.len();
        let mut inverted_image: Vec<Vec<i32>> = vec![vec![0; n]; n];

        for i in 0..n {
            for j in 0..n {
                inverted_image[i][j] = image[i][n-j-1]^1;
            }
        }

        inverted_image
    }
}