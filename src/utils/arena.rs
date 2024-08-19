pub struct Arena {
    x: i32,
}


impl Arena {
    pub fn new() -> Arena {
        //support default size is 10
        Arena { x: 10i32 }
    }
    pub fn allocate(&self, bytes: usize) -> f64 {
        let res: f64 = (self.x as f64) * (bytes as f64);
        res
    }
    pub fn set(&mut self, val: i32) {
        self.x = val;
    }
    pub fn get(&self) -> i32 {
        self.x
    }
}