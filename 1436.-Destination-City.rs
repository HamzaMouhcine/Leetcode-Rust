use std::collections::HashMap;

impl Solution {
    pub fn dest_city(paths: Vec<Vec<String>>) -> String {
        let mut out_freq: HashMap<String, u8> = HashMap::new();
        
        for i in 0..paths.len() {
            *out_freq.entry(paths[i][0].clone()).or_insert(0) += 1;
            out_freq.entry(paths[i][1].clone()).or_insert(0);
        }
        
        let mut destination_city = String::from("");
        for (k, v) in &out_freq {
            if *v == 0 {
                destination_city = (*k.clone()).to_string();
            }
        }
        destination_city
    }
}