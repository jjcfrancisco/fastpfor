pub struct Cursor {
    pub value: i32,
}

impl Cursor {
    pub fn new() -> Cursor {
        Cursor { value: 0 }
    }

    pub fn get(&self) -> i32 {
        self.value
    }

    pub fn set(&mut self, i: i32) {
        self.value = i;
    }

    pub fn add(&mut self, i: i32) {
        self.value += i;
    }

    pub fn increment(&mut self) {
        self.value += 1;
    }
}
