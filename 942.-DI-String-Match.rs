//https://leetcode.com/problems/di-string-match/submissions/

impl Solution {
    pub fn di_string_match(s: String) -> Vec<i32> {
        let mut perm = Vec::with_capacity(s.len());
        
        let mut left: i32 = 0;
        let mut right = s.len() as i32;
        let mut last_char = s.chars().nth(0).unwrap();
        let mut seq_len = 1;
        
        for c in s.chars() {
            if last_char == c {
                seq_len += 1;
            } else {
                match last_char {
                    'D' => {
                        insert_sequence(&mut perm, seq_len, last_char, left);
                        left += seq_len;
                    },
                    'I' => {
                        insert_sequence(&mut perm, seq_len, last_char, right);
                        right -= seq_len;
                    },
                    _   => ()
                }
                seq_len = 1;
            }
            last_char = c;
        }
        
        match last_char {
                    'D' => insert_sequence(&mut perm, seq_len, last_char, left),
                    'I' => insert_sequence(&mut perm, seq_len, last_char, right),
                    _   => ()
        }
        
        perm
    }
}

pub fn insert_sequence(perm: &mut Vec<i32>,cnt: i32, c: char, idx: i32) {
    match c {
        'I' => {
            for i in (idx-cnt+1)..(idx+1) {
                perm.push(i);
            }
        },
        'D' => {
            for i in (idx..(idx+cnt)).rev() {
                perm.push(i);
            }
        },
        _ => ()
    }
}