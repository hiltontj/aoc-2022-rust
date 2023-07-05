use std::{
    collections::{HashSet, VecDeque},
    io::Read,
};

use crate::utils::get_buf_reader;

pub fn answer_part_1() -> usize {
    let buf_reader = get_buf_reader("input/day_06.txt");
    let mut sig_buffer = SignalBuffer::new();
    let mut offset = 1;
    for c in buf_reader
        .bytes()
        .map(Result::unwrap)
        .map(|byte| char::from(byte))
    {
        sig_buffer.push(c);
        if sig_buffer.is_marker() {
            return offset;
        } else {
            offset += 1;
        }
    }

    offset
}

struct SignalBuffer(VecDeque<char>);

impl SignalBuffer {
    fn new() -> Self {
        Self(VecDeque::with_capacity(4))
    }

    fn is_full(&self) -> bool {
        self.0.len() == 4
    }

    fn push(&mut self, c: char) {
        if self.is_full() {
            self.0.pop_back();
            self.0.push_front(c);
        } else {
            self.0.push_front(c);
        }
    }

    fn is_marker(&self) -> bool {
        if self.0.len() < 4 {
            return false;
        }
        let mut hash = HashSet::new();
        for c in self.0.iter() {
            if !hash.insert(c) {
                return false;
            }
        }
        true
    }
}
