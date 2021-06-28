impl Solution {
    pub fn num_identical_pairs(nums: Vec<i32>) -> i32 {
        let mut cnt = vec![0; 101];

        for i in 0..nums.len() {
            cnt[nums[i] as usize] += 1;
        }

        let mut ans = 0;
        for i in 1..101 {
            ans += (cnt[i] * (cnt[i] - 1)) / 2;
        }
    
        ans
    }
}