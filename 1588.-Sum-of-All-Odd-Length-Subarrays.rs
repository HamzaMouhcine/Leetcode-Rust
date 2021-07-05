//https://leetcode.com/problems/sum-of-all-odd-length-subarrays/submissions/
impl Solution {
    pub fn sum_odd_length_subarrays(arr: Vec<i32>) -> i32 {
        let mut odd_sum = (0, 0);
        let mut even_sum = (0, 0);
        even_sum.1 += 1;

        let mut odd_size_sum = 0;
        let mut current_sum = 0;

        for i in 0..arr.len() {
            current_sum += arr[i];
            match (i+1)%2 {
                0 => {
                    odd_size_sum += (current_sum * odd_sum.1) - odd_sum.0;

                    even_sum.0 += current_sum;
                    even_sum.1 += 1;
                },
                1 => {
                    odd_size_sum += (current_sum * even_sum.1) - even_sum.0;

                    odd_sum.0 += current_sum;
                    odd_sum.1 += 1;
                },
                _ => ()
            }
        }
        
        odd_size_sum
    }
}