use std::{collections::HashSet, str::FromStr};

use crate::utils::get_lines;

pub fn answer_part_1() -> usize {
    let mut rope = Rope::new(2);
    for line in get_lines("input/day_09.txt").map(Result::unwrap) {
        let instruction = Instruction::from_str(&line);
        rope.move_to(instruction);
    }
    rope.count_visited(1)
}

pub fn answer_part_2() -> usize {
    let mut rope = Rope::new(10);
    for line in get_lines("input/day_09.txt").map(Result::unwrap) {
        let instruction = Instruction::from_str(&line);
        rope.move_to(instruction);
    }
    rope.count_visited(9)
}

#[derive(Debug, Default)]
struct Rope(Vec<Knot>);

impl Rope {
    fn new(size: usize) -> Self {
        Self(vec![Knot::default(); size])
    }

    fn move_to(&mut self, instruction: Instruction) {
        for _ in 0..instruction.steps {
            for i in 0..self.0.len() {
                if i == 0 {
                    self.0[i].step(instruction.direction);
                } else {
                    let Knot { x, y, .. } = self.0[i - 1];
                    self.0[i].follow(x, y);
                }
            }
        }
    }

    fn count_visited(&self, knot: usize) -> usize {
        self.0[knot].count_visited()
    }
}

#[derive(Debug, Default, Clone)]
struct Knot {
    x: isize,
    y: isize,
    visited: HashSet<(isize, isize)>,
}

impl Knot {
    fn count_visited(&self) -> usize {
        self.visited.len()
    }

    fn step(&mut self, direction: Direction) {
        match direction {
            Direction::Up => self.y += 1,
            Direction::Down => self.y -= 1,
            Direction::Left => self.x -= 1,
            Direction::Right => self.x += 1,
        }
        self.visited.insert((self.x, self.y));
    }

    fn follow(&mut self, x: isize, y: isize) {
        if self.x.abs_diff(x) > 1 {
            if self.x > x {
                self.x -= 1;
            } else {
                self.x += 1;
            }
            if self.y.abs_diff(y) > 0 {
                if self.y > y {
                    self.y -= 1;
                } else {
                    self.y += 1;
                }
            }
        }
        if self.y.abs_diff(y) > 1 {
            if self.y > y {
                self.y -= 1;
            } else {
                self.y += 1;
            }
            if self.x.abs_diff(x) > 0 {
                if self.x > x {
                    self.x -= 1;
                } else {
                    self.x += 1;
                }
            }
        }
        self.visited.insert((self.x, self.y));
    }
}

#[derive(Debug)]
struct Instruction {
    direction: Direction,
    steps: usize,
}

impl Instruction {
    fn from_str(s: &str) -> Self {
        let mut split = s.split_whitespace();
        let direction = match split.next().expect("direction instruction") {
            "U" => Direction::Up,
            "D" => Direction::Down,
            "L" => Direction::Left,
            "R" => Direction::Right,
            d => panic!("unexpected direction instruction: {d}"),
        };
        let steps = usize::from_str(split.next().expect("steps instruction"))
            .expect("steps is not a number");
        Self { direction, steps }
    }
}

#[derive(Debug, Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}
