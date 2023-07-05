use std::{
    collections::{HashSet, VecDeque},
    io::Read,
};

use crate::utils::get_buf_reader;

pub fn answer_part_1() -> usize {
    let buf_reader = get_buf_reader("input/day_06.txt");
    let mut sig_buffer = SignalBuffer::<4>::new();
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

pub fn answer_part_2() -> usize {
    let buf_reader = get_buf_reader("input/day_06.txt");
    let mut sig_buffer = SignalBuffer::<14>::new();
    let mut offset = 1;
    for c in buf_reader
        .bytes()
        .map(Result::unwrap)
        .map(|b| char::from(b))
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

struct SignalBuffer<const SIZE: usize>(VecDeque<char>);

impl<const SIZE: usize> SignalBuffer<SIZE> {
    fn new() -> Self {
        Self(VecDeque::with_capacity(SIZE))
    }

    fn is_full(&self) -> bool {
        self.0.len() == SIZE
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
        if self.0.len() < SIZE {
            return false;
        }
        // TODO - could we not allocate a new HashSet every time we call this function?
        // It may be better to house the hash set in the SignalBuffer, and evict keys
        // as elements are popped from the dequeue. However, in that case, a hash set
        // may not be sufficient, as it would need to account for there being multiples
        // of a given character.
        let mut hash = HashSet::new();
        for c in self.0.iter() {
            if !hash.insert(c) {
                return false;
            }
        }
        true
    }
}
