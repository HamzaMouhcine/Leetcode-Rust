//https://leetcode.com/problems/design-an-ordered-stream/submissions/
struct OrderedStream {
    list: Vec<String>,
    used: Vec<bool>
}


impl OrderedStream {
    
    fn new(n: i32) -> Self {
        OrderedStream {
            list : vec![String::from(""); n as usize],
            used: vec![false; n as usize]
        }
    }
    
    fn insert(&mut self,mut id_key: i32, value: String) -> Vec<String> {
        id_key -= 1;
        self.list[id_key as usize] = value;
        
        let mut new_chunk: Vec<String> = vec![];
        
        if id_key == 0 || self.used[(id_key-1) as usize] == true {
            for i in id_key as usize..self.list.len() {
                if self.list[i] == "" {
                    break;
                }
                
                self.used[i] = true;
                new_chunk.push(self.list[i].clone());
            }
        }
        
        new_chunk
    }
}