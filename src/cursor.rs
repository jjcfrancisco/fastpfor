use std::io::Cursor;

pub trait IncrementCursor {
    fn increment(&mut self);
}

impl IncrementCursor for Cursor<i32> {
    fn increment(&mut self) {
        self.set_position(self.position() + 1); // Position needs to be a u64
    }
}
