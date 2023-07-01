use std::collections::BinaryHeap;

use crate::utils::get_lines;

pub fn answer_part_1() -> usize {
    let mut most = 0;
    let mut current = 0;
    for line in get_lines("input/day_01.txt") {
        match line {
            Ok(l) => {
                if l.is_empty() {
                    if current > most {
                        most = current
                    }
                    current = 0;
                } else {
                    current += l.parse::<usize>().expect("unparseable line!")
                }
            }
            Err(e) => panic!("failed to read line: {e}"),
        }
    }
    most
}

pub fn answer_part_2() -> usize {
    let mut heap = BinaryHeap::new();
    let mut current = 0;
    for line in get_lines("input/day_01.txt") {
        let line = line.unwrap();
        if line.is_empty() {
            heap.push(current);
            current = 0;
        } else {
            current += line.parse::<usize>().expect("parseable line");
        }
    }
    heap.pop().unwrap() + heap.pop().unwrap() + heap.pop().unwrap()
}
