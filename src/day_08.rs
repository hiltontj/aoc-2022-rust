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

pub fn answer_part_2() -> u64 {
    let tp = TreePatch::from_file("input/day_08.txt");
    tp.score()
}

struct TreePatch<T> {
    grid: HashMap<(usize, usize), T>,
    size: (usize, usize),
}

// This is for debugging purposes
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

    fn insert(&mut self, (x, y): (usize, usize), val: T) {
        self.size.0 = self.size.0.max(x + 1);
        self.size.1 = self.size.1.max(y + 1);
        assert!(
            self.grid.insert((x, y), val).is_none(),
            "attempted insert to occupied slot ({x}, {y})"
        );
    }

    fn row(&self, row: usize) -> RowIter<'_, T> {
        RowIter {
            tree_patch: self,
            row,
            col_front: 0,
            col_back: self.size.0.checked_sub(1),
        }
    }

    fn row_bounded(&self, row: usize, start: Option<usize>, end: Option<usize>) -> RowIter<'_, T> {
        RowIter {
            tree_patch: self,
            row,
            col_front: start.unwrap_or(0),
            col_back: end.or_else(|| Some(self.n_cols() - 1)),
        }
    }

    fn col(&self, col: usize) -> ColIter<'_, T> {
        ColIter {
            tree_patch: self,
            col,
            row_front: 0,
            row_back: self.size.1.checked_sub(1),
        }
    }

    fn col_bounded(&self, col: usize, start: Option<usize>, end: Option<usize>) -> ColIter<'_, T> {
        ColIter {
            tree_patch: self,
            col,
            row_front: start.unwrap_or(0),
            row_back: end.or_else(|| Some(self.n_rows() - 1)),
        }
    }

    fn n_rows(&self) -> usize {
        self.size.0
    }

    fn n_cols(&self) -> usize {
        self.size.1
    }
}

fn char_to_u8(c: char) -> u8 {
    match c {
        '0' => 0,
        '1' => 1,
        '2' => 2,
        '3' => 3,
        '4' => 4,
        '5' => 5,
        '6' => 6,
        '7' => 7,
        '8' => 8,
        '9' => 9,
        _ => panic!("invalid char {c}"),
    }
}

impl TreePatch<u8> {
    fn from_file<P: AsRef<Path>>(path: P) -> Self {
        let mut tp = TreePatch::new();
        for (i, line) in get_lines(path).map(Result::unwrap).enumerate() {
            for (j, c) in line.chars().enumerate() {
                tp.insert((i, j), char_to_u8(c));
            }
        }
        tp
    }

    fn row_wise_forward(&self) -> TreePatch<bool> {
        let mut tp = TreePatch::<bool>::new();
        tp.size = self.size;
        for i in 0..self.n_rows() {
            let mut highest = 0;
            for (j, h) in self.row(i).enumerate() {
                highest = tp.set_hidden(i, j, *h, highest);
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
                highest = tp.set_hidden(i, j, *h, highest);
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
                highest = tp.set_hidden(i, j, *h, highest);
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
                highest = tp.set_hidden(i, j, *h, highest);
            }
        }
        tp
    }

    fn score(&self) -> u64 {
        let mut best = 0;
        for ((i, j), this) in &self.grid {
            if *i == 0 || *j == 0 || *i == self.n_rows() - 1 || *j == self.n_cols() - 1 {
                continue;
            }
            let mut up = 0;
            for other in self.col_bounded(*j, None, Some(i - 1)).rev() {
                up += 1;
                if other >= this {
                    break;
                }
            }
            let mut left = 0;
            for other in self.row_bounded(*i, None, Some(j - 1)).rev() {
                left += 1;
                if other >= this {
                    break;
                }
            }
            let mut down = 0;
            for other in self.col_bounded(*j, Some(i + 1), None) {
                down += 1;
                if other >= this {
                    break;
                }
            }
            let mut right = 0;
            for other in self.row_bounded(*i, Some(j + 1), None) {
                right += 1;
                if other >= this {
                    break;
                }
            }
            best = best.max(up * down * left * right);
        }
        best
    }
}

impl TreePatch<bool> {
    /// Set the hidden state for a tree-top in the [`TreePatch`] slot at `row` and `col` with `height`.
    ///
    /// Returns the new highest tree height.
    fn set_hidden(&mut self, row: usize, col: usize, height: u8, highest: u8) -> u8 {
        if row == 0 || row == self.n_rows() - 1 || col == 0 || col == self.n_cols() - 1 {
            // The tree-top is not hidden if it is on the edge of the tree patch:
            self.insert((row, col), false);
        } else {
            // Otherwise, we base it's hidden-ness on the highest visited tree
            self.insert((row, col), height <= highest);
        }
        highest.max(height)
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
    col_front: usize,
    col_back: Option<usize>,
}

impl<'a, T> Iterator for RowIter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        let col_front = self.col_front;
        self.col_front += 1;
        self.tree_patch.grid.get(&(self.row, col_front))
    }
}

impl<'a, T> DoubleEndedIterator for RowIter<'a, T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        let Some(col_back) = self.col_back else {
            return None;
        };
        self.col_back = col_back.checked_sub(1);
        self.tree_patch.grid.get(&(self.row, col_back))
    }
}

impl<'a, T> ExactSizeIterator for RowIter<'a, T> {
    fn len(&self) -> usize {
        match self.col_back {
            Some(cb) => cb + 1 - self.col_front,
            None => 0,
        }
    }
}

struct ColIter<'a, T> {
    tree_patch: &'a TreePatch<T>,
    col: usize,
    row_front: usize,
    row_back: Option<usize>,
}

impl<'a, T> Iterator for ColIter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        let row_front = self.row_front;
        self.row_front += 1;
        self.tree_patch.grid.get(&(row_front, self.col))
    }
}

impl<'a, T> DoubleEndedIterator for ColIter<'a, T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        let Some(row_back) = self.row_back else {
            return None;
        };
        self.row_back = row_back.checked_sub(1);
        self.tree_patch.grid.get(&(row_back, self.col))
    }
}

impl<'a, T> ExactSizeIterator for ColIter<'a, T> {
    fn len(&self) -> usize {
        match self.row_back {
            Some(rb) => rb + 1 - self.row_front,
            None => 0,
        }
    }
}
