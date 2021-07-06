//https://leetcode.com/problems/number-of-students-doing-homework-at-a-given-time/submissions/

impl Solution {
    pub fn busy_student(start_time: Vec<i32>, end_time: Vec<i32>, query_time: i32) -> i32 {
        let mut students_count = 0;
        
        for i in 0..start_time.len() {
            if start_time[i] <= query_time && query_time <= end_time[i] {
                students_count += 1;
            } 
        }
        
        students_count
    }
}