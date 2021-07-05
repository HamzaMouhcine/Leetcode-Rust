// https://leetcode.com/problems/count-of-matches-in-tournament/submissions/
impl Solution {
    pub fn number_of_matches(mut n: i32) -> i32 {
        let mut number_of_matches = 0;
        while n != 1 {
            number_of_matches += n/2;
            n = (n+1)/2;
        }
        
        number_of_matches
    }
}