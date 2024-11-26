pub struct ByteBuffer {
    buffer: Vec<u8>,
    position: usize,
    limit: usize,
}

impl ByteBuffer {
    // Create a new ByteBuffer with the specified capacity
    pub fn new(capacity: usize) -> Self {
        ByteBuffer {
            buffer: vec![0; capacity],
            position: 0,
            limit: capacity,
        }
    }

    // Clear the buffer, resetting position and limit
    pub fn clear(&mut self) {
        self.position = 0;
        self.limit = self.buffer.len();
    }

    // Get the current position
    pub fn position(&self) -> usize {
        self.position
    }

    // Set a new position
    pub fn set_position(&mut self, pos: usize) {
        if pos > self.limit {
            panic!("Position exceeds limit");
        }
        self.position = pos;
    }

    // Write a single byte to the buffer
    pub fn put(&mut self, byte: u8) {
        if self.position >= self.limit {
            panic!("Buffer overflow");
        }
        self.buffer[self.position] = byte;
        self.position += 1;
    }

    // Read a single byte from the buffer
    pub fn get(&mut self) -> u8 {
        if self.position >= self.limit {
            panic!("Buffer underflow");
        }
        let byte = self.buffer[self.position];
        self.position += 1;
        byte
    }

    // Get the internal buffer as a slice
    pub fn as_slice(&self) -> &[u8] {
        &self.buffer
    }
}

