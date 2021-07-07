//https://leetcode.com/problems/number-of-recent-calls/submissions/
use std::collections::VecDeque;

struct RecentCounter {
    recent_calls: VecDeque<i32>
}

impl RecentCounter {

    fn new() -> Self {
        RecentCounter {
            recent_calls: VecDeque::new()
        }
    }
    
    fn ping(&mut self, t: i32) -> i32 {
        self.recent_calls.push_back(t);
        
        while     self.recent_calls.len() != 0
              && *self.recent_calls.front().unwrap() < t - 3000 {
            self.recent_calls.pop_front();
        }
        
        self.recent_calls.len() as i32
    }
}