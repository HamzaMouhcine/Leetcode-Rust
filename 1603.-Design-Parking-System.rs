struct ParkingSystem {
    big: i32,
    medium: i32,
    small: i32
}

impl ParkingSystem {

    fn new(big: i32, medium: i32, small: i32) -> Self {
        ParkingSystem {
            big : big,
            medium : medium,
            small : small
        }
    }
    
    fn add_car(&mut self, car_type: i32) -> bool {
        match car_type {
            1 => { // big
                match self.big != 0 {
                    true => { 
                        self.big -= 1;
                        true
                    },
                    false => false
                }
            },
            2 => {
                match self.medium != 0 {
                    true => {
                        self.medium -= 1;
                        true
                    },
                    false => false
                }
            },
            3 => {
                match self.small != 0 {
                    true => {
                        self.small -= 1;
                        true
                    },
                    false => false
                }
            },
            _ => true
        }
    }
}