// https://leetcode.com/problems/maximum-units-on-a-truck/submissions/

#[derive(Debug, Eq, Ord, PartialEq, PartialOrd)]
struct BoxType {
    number_of_units: i32,
    number_of_boxes: i32
}

impl Solution {
    
    pub fn maximum_units(box_types: Vec<Vec<i32>>, truck_size: i32) -> i32 {
        let mut sorted_types: Vec<BoxType> = Vec::with_capacity(box_types.len());
        
        for vec in &box_types {
            sorted_types.push(
                BoxType {
                    number_of_units: vec[1],
                    number_of_boxes: vec[0]
                }
            );
        }
        sorted_types.sort();
        
        let mut number_of_boxes = 0;
        let mut maximum_number_of_units = 0;
        for i in (0..sorted_types.len()).rev() {
            let add = (truck_size - number_of_boxes).min(sorted_types[i].number_of_boxes);
            maximum_number_of_units += add * sorted_types[i].number_of_units;
            
            number_of_boxes += add;
        }
        
        maximum_number_of_units
    }
}