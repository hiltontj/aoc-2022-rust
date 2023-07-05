use std::{
    cell::RefCell,
    collections::{BTreeMap, VecDeque},
    ops::{Deref, DerefMut},
    str::FromStr,
};

use crate::utils::get_lines;

/*

    Sample Input:

            [H]         [S]         [D]
        [S] [C]         [C]     [Q] [L]
        [C] [R] [Z]     [R]     [H] [Z]
        [G] [N] [H] [S] [B]     [R] [F]
    [D] [T] [Q] [F] [Q] [Z]     [Z] [N]
    [Z] [W] [F] [N] [F] [W] [J] [V] [G]
    [T] [R] [B] [C] [L] [P] [F] [L] [H]
    [H] [Q] [P] [L] [G] [V] [Z] [D] [B]
     1   2   3   4   5   6   7   8   9

*/

pub fn answer_part_1() -> String {
    let mut cargo_bay = CargoBay::new_9000();
    for line in get_lines("input/day_05.txt").map(Result::unwrap) {
        if !cargo_bay.process_input_line(&line) {
            continue;
        }
    }
    for line in get_lines("input/day_05.txt").map(Result::unwrap) {
        if let Ok(instruction) = line.parse::<Instruction>() {
            cargo_bay.move_crate(instruction);
        }
    }
    cargo_bay.list_top_crates()
}

pub fn answer_part_2() -> String {
    let mut cargo_bay = CargoBay::new_9001();
    for line in get_lines("input/day_05.txt").map(Result::unwrap) {
        if !cargo_bay.process_input_line(&line) {
            continue;
        }
    }
    for line in get_lines("input/day_05.txt").map(Result::unwrap) {
        if let Ok(instruction) = line.parse::<Instruction>() {
            cargo_bay.move_crate(instruction);
        }
    }
    cargo_bay.list_top_crates()
}

#[derive(Debug)]
struct Crate(char);

impl Crate {
    fn from_input(c: char) -> Option<Self> {
        match c {
            'A'..='Z' => Some(Self(c)),
            _ => None,
        }
    }
}

#[derive(Default, Debug)]
struct CrateStack(VecDeque<Crate>);

impl Deref for CrateStack {
    type Target = VecDeque<Crate>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for CrateStack {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

struct CargoBay {
    stack_list: BTreeMap<usize, RefCell<CrateStack>>,
    crane: Crane,
}

impl CargoBay {
    fn new_9000() -> Self {
        Self {
            stack_list: Default::default(),
            crane: Crane::NineThousand,
        }
    }

    fn new_9001() -> Self {
        Self {
            stack_list: Default::default(),
            crane: Crane::NineThousandOne(Default::default()),
        }
    }

    fn move_crate(&self, instruction: Instruction) {
        let mut from = self
            .stack_list
            .get(&instruction.from)
            .expect("valid from instruction")
            .borrow_mut();
        let mut to = self
            .stack_list
            .get(&instruction.to)
            .expect("valid to instruction")
            .borrow_mut();
        match self.crane {
            Crane::NineThousand => {
                for _ in 0..instruction.amount {
                    to.push_front(
                        from.pop_front()
                            .expect("could not pop a crate off from stack"),
                    );
                }
            }
            Crane::NineThousandOne(ref buffer) => {
                let mut buffer = buffer.borrow_mut();
                for _ in 0..instruction.amount {
                    buffer.push(
                        from.pop_front()
                            .expect("could not pop a crate off from stack"),
                    );
                }
                while let Some(crt) = buffer.pop() {
                    to.push_front(crt);
                }
            }
        }
    }

    fn insert_crate(&mut self, slot: usize, crt: Crate) {
        let stack = self.stack_list.entry(slot).or_insert(Default::default());
        stack.borrow_mut().push_back(crt);
    }

    fn process_input_line(&mut self, line: &str) -> bool {
        let mut crates_inserted = false;
        let slots = line
            .chars()
            .collect::<Vec<char>>()
            .chunks(4)
            .map(|chars| Crate::from_input(chars[1]))
            .collect::<Vec<Option<Crate>>>();
        for (i, slot) in slots.into_iter().enumerate() {
            if let Some(crt) = slot {
                self.insert_crate(i + 1, crt);
                crates_inserted = true;
            }
        }
        crates_inserted
    }

    fn list_top_crates(&self) -> String {
        self.stack_list
            .iter()
            .map(|(_, stack)| stack.borrow_mut().pop_front().expect("stack not empty").0)
            .collect::<String>()
    }
}

struct Instruction {
    amount: usize,
    from: usize,
    to: usize,
}

impl FromStr for Instruction {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut words = s.split_whitespace();
        if !matches!(words.next(), Some("move")) {
            return Err(String::from("expected move instruction"));
        }
        let amount = words
            .next()
            .ok_or_else(|| format!("move amount"))?
            .parse::<usize>()
            .map_err(|_| format!("move amount is valid usize"))?;
        if !matches!(words.next(), Some("from")) {
            return Err(String::from("expected from instruction"));
        }
        let from = words
            .next()
            .ok_or_else(|| format!("from stack"))?
            .parse::<usize>()
            .map_err(|_| format!("from stack is valid usize"))?;
        if !matches!(words.next(), Some("to")) {
            return Err(String::from("expected to instruction"));
        }
        let to = words
            .next()
            .ok_or_else(|| format!("to stack"))?
            .parse::<usize>()
            .map_err(|_| format!("to stack is valid usize"))?;
        Ok(Self { amount, from, to })
    }
}

enum Crane {
    NineThousand,
    NineThousandOne(RefCell<Vec<Crate>>),
}
