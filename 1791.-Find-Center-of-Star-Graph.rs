//https://leetcode.com/problems/find-center-of-star-graph/submissions/
impl Solution {
    pub fn find_center(edges: Vec<Vec<i32>>) -> i32 {
        let mut ans = 0;
        let mut cnt: Vec<i32> = vec![0; 100001];
        
        for i in 0..edges.len() {
            cnt[edges[i][0] as usize] += 1;
            cnt[edges[i][1] as usize] += 1;
            
            if cnt[edges[i][0] as usize] == 2 {
                ans = edges[i][0];
            } else if cnt[edges[i][1] as usize] == 2 {
                ans = edges[i][1];
            }
        }
        
        ans
    }
}