use std::collections::VecDeque;

pub struct Buffer {
    vec: VecDeque<char>,
}

impl Buffer {
    pub fn new(input: &str) -> Buffer {
        let mut vec: VecDeque<char> = VecDeque::new();

        for char in input.chars() {
            vec.push_back(char);
        }

        Buffer { vec }
    }

    pub fn is_empty(&self) -> bool {
        self.vec.is_empty()
    }

    pub fn next(&mut self) -> char {
        self.vec
            .pop_front()
            .expect("buffers in the queue are empty")
    }
}
