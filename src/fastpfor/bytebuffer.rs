use bytemuck::cast_slice;
use std::u32;

#[derive(Debug)]
pub struct ByteBuffer {
    buffer: Vec<u8>,
    position: i32,
    limit: usize,
}

#[derive(Debug)]
pub struct IntBuffer {
    pub buffer: Vec<i32>,
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
    pub fn position(&self) -> i32 {
        self.position
    }

    // Set a new position
    pub fn set_position(&mut self, pos: i32) {
        if pos > self.limit as i32 {
            panic!("Position exceeds limit");
        }
        self.position = pos;
    }

    // Write a single byte to the buffer
    pub fn put(&mut self, byte: u8) {
        if self.position >= self.limit as i32 {
            panic!("Buffer overflow");
        }
        self.buffer[self.position as usize] = byte;
        self.position += 1;
    }

    // Read a single byte from the buffer
    pub fn get(&mut self) -> u8 {
        if self.position >= self.limit as i32 {
            panic!("Buffer underflow");
        }
        let byte = self.buffer[self.position as usize];
        self.position += 1;
        byte
    }

    // Get the internal buffer as a slice
    pub fn as_slice(&self) -> &[u8] {
        &self.buffer
    }

    // Flip the buffer, setting the limit to the current position
    pub fn flip(&mut self) {
        self.limit = self.position as usize;
        self.position = 0;
    }

    // Get the buffer as a slice of integers
    // https://www.dotnetperls.com/convert-bytes-integer-rust
    pub fn as_int_buffer(&self) -> IntBuffer {
        // Ensure the buffer length is a multiple of 4
        if self.buffer.len() % 4 != 0 {
            panic!("Buffer length is not a multiple of 4");
        }

        IntBuffer {
            buffer: cast_slice(&self.buffer).to_vec(),
        }
    }
}

impl IntBuffer {
    // Create a new IntBuffer with the specified capacity
    pub fn get(&self, dst: &mut [i32], offset: usize, length: usize) {
        if offset + length > dst.len() {
            panic!("Buffer overflow");
        }
        dst[offset..offset + length].copy_from_slice(&self.buffer[..length]);
    }
}
