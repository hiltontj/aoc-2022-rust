use std::{collections::HashMap, ops::BitAnd, path::Path};

use crate::utils::get_lines;

pub fn answer_part_1() -> u64 {
    let tp = TreePatch::from_file("input/day_08.txt");
    let hidden = tp.row_wise_forward()
        & tp.row_wise_backward()
        & tp.col_wise_forward()
        & tp.col_wise_backward();
    let mut result = 0;
    // print!("{hidden}");
    for (_, h) in hidden.grid {
        if !h {
            result += 1;
        }
    }
    result
}

struct TreePatch<T> {
    grid: HashMap<(usize, usize), T>,
    size: (usize, usize),
}

impl std::fmt::Display for TreePatch<bool> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut disp: Vec<Vec<bool>> = vec![vec![Default::default(); self.n_cols()]; self.n_rows()];
        for ((i, j), v) in &self.grid {
            disp[*i][*j] = *v;
        }
        for row in disp {
            for col in row {
                if col {
                    write!(f, "1 ")?;
                } else {
                    write!(f, "0 ")?;
                }
            }
            write!(f, "\n")?;
        }
        Ok(())
    }
}

impl<T> TreePatch<T> {
    fn new() -> Self {
        Self {
            grid: HashMap::new(),
            size: (0, 0),
        }
    }

    fn insert(&mut self, (x, y): (usize, usize), val: T) -> Option<T> {
        self.size.0 = self.size.0.max(x + 1);
        self.size.1 = self.size.1.max(y + 1);
        self.grid.insert((x, y), val)
    }

    fn row(&self, row: usize) -> RowIter<'_, T> {
        RowIter {
            tree_patch: self,
            row,
            current: 0,
            current_back: self.size.0.checked_sub(1),
        }
    }

    fn col(&self, col: usize) -> ColIter<'_, T> {
        ColIter {
            tree_patch: self,
            col,
            current: 0,
            current_back: self.size.1.checked_sub(1),
        }
    }

    fn n_rows(&self) -> usize {
        self.size.0
    }

    fn n_cols(&self) -> usize {
        self.size.1
    }
}

impl TreePatch<u8> {
    fn from_file<P: AsRef<Path>>(path: P) -> Self {
        let mut grid = HashMap::new();
        for (i, line) in get_lines(path).map(Result::unwrap).enumerate() {
            for (j, c) in line.chars().enumerate() {
                match c {
                    '0'..='9' => assert!(
                        grid.insert((i, j), u8::try_from(c).unwrap() - 48).is_none(),
                        "insert at ({i}, {j}) was already occupied"
                    ),
                    unexpected => panic!("unexpected character: {unexpected}"),
                }
            }
        }
        let mut size = (0, 0);
        for (i, j) in grid.keys() {
            if *i > size.0 {
                size.0 = *i;
            }
            if *j > size.1 {
                size.1 = *j;
            }
        }
        size.0 += 1;
        size.1 += 1;
        Self { grid, size }
    }

    fn row_wise_forward(&self) -> TreePatch<bool> {
        let mut tp = TreePatch::<bool>::new();
        tp.size = self.size;
        for i in 0..self.n_rows() {
            let mut highest = 0;
            for (j, h) in self.row(i).enumerate() {
                // println!("{i}, {j}, {highest}, {h}");
                if i == 0 || i == self.n_rows() - 1 || j == 0 || j == self.n_cols() - 1 {
                    assert!(tp.insert((i, j), false).is_none());
                } else {
                    assert!(tp.insert((i, j), *h <= highest).is_none());
                }
                highest = highest.max(*h);
            }
        }
        tp
    }

    fn row_wise_backward(&self) -> TreePatch<bool> {
        let mut tp = TreePatch::<bool>::new();
        tp.size = self.size;
        for i in 0..self.n_rows() {
            let mut highest = 0;
            for (j, h) in self.row(i).enumerate().rev() {
                // println!("{i}, {j}, {highest}, {h}");
                if i == 0 || i == self.n_rows() - 1 || j == 0 || j == self.n_cols() - 1 {
                    assert!(tp.insert((i, j), false).is_none());
                } else {
                    assert!(tp.insert((i, j), *h <= highest).is_none());
                }
                highest = highest.max(*h);
            }
        }
        tp
    }

    fn col_wise_forward(&self) -> TreePatch<bool> {
        let mut tp = TreePatch::<bool>::new();
        tp.size = self.size;
        for j in 0..self.n_cols() {
            let mut highest = 0;
            for (i, h) in self.col(j).enumerate() {
                // println!("{i}, {j}, {highest}, {h}");
                if i == 0 || i == self.n_rows() - 1 || j == 0 || j == self.n_cols() - 1 {
                    assert!(tp.insert((i, j), false).is_none());
                } else {
                    assert!(tp.insert((i, j), *h <= highest).is_none());
                }
                highest = highest.max(*h);
            }
        }
        tp
    }

    fn col_wise_backward(&self) -> TreePatch<bool> {
        let mut tp = TreePatch::<bool>::new();
        tp.size = self.size;
        for j in 0..self.n_cols() {
            let mut highest = 0;
            for (i, h) in self.col(j).enumerate().rev() {
                // println!("{i}, {j}, {highest}, {h}");
                if i == 0 || i == self.n_rows() - 1 || j == 0 || j == self.n_cols() - 1 {
                    assert!(tp.insert((i, j), false).is_none());
                } else {
                    assert!(tp.insert((i, j), *h <= highest).is_none());
                }
                highest = highest.max(*h);
            }
        }
        tp
    }
}

impl BitAnd for TreePatch<bool> {
    type Output = Self;

    fn bitand(mut self, rhs: Self) -> Self::Output {
        for ((i, j), this) in self.grid.iter_mut() {
            let other = rhs.grid.get(&(*i, *j)).unwrap_or(&false);
            *this = *this & other;
        }
        self
    }
}

struct RowIter<'a, T> {
    tree_patch: &'a TreePatch<T>,
    row: usize,
    current: usize,
    current_back: Option<usize>,
}

impl<'a, T> Iterator for RowIter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        match self.tree_patch.grid.get(&(self.row, self.current)) {
            Some(v) => {
                self.current += 1;
                Some(v)
            }
            None => None,
        }
    }
}

impl<'a, T> DoubleEndedIterator for RowIter<'a, T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        match self
            .current_back
            .and_then(|cb| self.tree_patch.grid.get(&(self.row, cb)))
        {
            Some(v) => {
                self.current_back = self.current_back.and_then(|cb| cb.checked_sub(1));
                Some(v)
            }
            None => None,
        }
    }
}

impl<'a, T> ExactSizeIterator for RowIter<'a, T> {
    fn len(&self) -> usize {
        match self.current_back {
            Some(cb) => cb + 1 - self.current,
            None => 0,
        }
    }
}

struct ColIter<'a, T> {
    tree_patch: &'a TreePatch<T>,
    col: usize,
    current: usize,
    current_back: Option<usize>,
}

impl<'a, T> Iterator for ColIter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        match self.tree_patch.grid.get(&(self.current, self.col)) {
            Some(v) => {
                self.current += 1;
                Some(v)
            }
            None => None,
        }
    }
}

impl<'a, T> DoubleEndedIterator for ColIter<'a, T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        match self
            .current_back
            .and_then(|cb| self.tree_patch.grid.get(&(cb, self.col)))
        {
            Some(v) => {
                self.current_back = self.current_back.and_then(|cb| cb.checked_sub(1));
                Some(v)
            }
            None => None,
        }
    }
}

impl<'a, T> ExactSizeIterator for ColIter<'a, T> {
    fn len(&self) -> usize {
        match self.current_back {
            Some(cb) => cb + 1 - self.current,
            None => 0,
        }
    }
}
