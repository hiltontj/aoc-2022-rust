use std::{
    collections::{HashMap, HashSet},
    ops::Div,
};

use crate::utils::get_lines;

pub fn answer_part_1() -> u32 {
    let mut sum = 0;
    for line in get_lines("input/day_03.txt") {
        let line = line.unwrap();
        let sack_size = line.len().div(2);
        let mut sack_1 = line;
        let sack_2 = sack_1.split_off(sack_size);
        let mut h = HashSet::with_capacity(sack_size);
        for c in sack_1.chars() {
            h.insert(c);
        }
        for c in sack_2.chars() {
            if h.contains(&c) {
                sum += get_priority(c);
                break;
            }
        }
    }
    sum
}

pub fn answer_part_2() -> u32 {
    let mut sum = 0;
    let mut buf = Vec::with_capacity(3);
    for line in get_lines("input/day_03.txt") {
        let line = line.unwrap();
        if buf.len() < 3 {
            buf.push(line);
        }
        if buf.len() == 3 {
            sum += process_group(buf.as_slice());
            buf.clear();
        }
    }
    sum
}

fn process_group(sacks: &[String]) -> u32 {
    let mut hash = HashMap::<char, [bool; 3]>::new();
    for (i, sack) in sacks.iter().enumerate() {
        for c in sack.chars() {
            let entry = hash.entry(c).or_insert(Default::default());
            entry[i] = true;
        }
    }
    let badge = hash
        .into_iter()
        .find(|(_, check)| check.iter().all(|s| *s))
        .expect("a badge with count 3");
    get_priority(badge.0)
}

fn get_priority(c: char) -> u32 {
    match c {
        'a'..='z' => u32::from(c) - 96,
        'A'..='Z' => u32::from(c) - 38,
        _ => panic!("unexpected character: {c}"),
    }
}

#[test]
fn test_priority() {
    assert_eq!(get_priority('a'), 1);
    assert_eq!(get_priority('A'), 27);
}
