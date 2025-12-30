use std::collections::HashSet;
use std::fmt;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// A single cell in a Sudoku grid.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum Cell {
    Empty,
    Given(u8),
    Filled(u8),
}

impl Cell {
    /// Returns the numeric value, if any.
    #[inline]
    pub fn value(self) -> Option<u8> {
        match self {
            Self::Empty => None,
            Self::Given(v) | Self::Filled(v) => Some(v),
        }
    }

    #[inline]
    pub fn is_empty(self) -> bool {
        matches!(self, Self::Empty)
    }

    #[inline]
    pub fn is_given(self) -> bool {
        matches!(self, Self::Given(_))
    }
}

/// A Sudoku puzzle grid.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Sudoku {
    pub grid: Vec<Vec<Cell>>,
    pub size: usize,
    pub box_size: usize,
}

impl Sudoku {
    /// Creates an empty Sudoku of the given size.
    pub fn new(size: usize) -> Self {
        let box_size = (size as f64).sqrt() as usize;
        assert!(box_size * box_size == size, "Size must be a perfect square");

        Self {
            grid: vec![vec![Cell::Empty; size]; size],
            size,
            box_size,
        }
    }

    /// Parses a Sudoku from a string representation.
    pub fn from_string(s: &str, size: usize) -> Result<Self, String> {
        let mut sudoku = Self::new(size);
        let chars: Vec<char> = s.chars().collect();
        let expected = size * size;

        if chars.len() != expected {
            return Err(format!("Expected {expected} chars, got {}", chars.len()));
        }

        for (i, &ch) in chars.iter().enumerate() {
            let (r, c) = (i / size, i % size);
            sudoku.grid[r][c] = match ch {
                '0' | '.' | ' ' => Cell::Empty,
                _ => Cell::Given(
                    parse_char(ch, size)
                        .ok_or_else(|| format!("Invalid char '{ch}' at ({r},{c})"))?,
                ),
            };
        }

        Ok(sudoku)
    }

    /// Returns the cell at (row, col), if in bounds.
    #[inline]
    pub fn get(&self, row: usize, col: usize) -> Option<Cell> {
        self.grid.get(row).and_then(|r| r.get(col).copied())
    }

    /// Sets the cell value. Use 0 to clear.
    pub fn set(&mut self, row: usize, col: usize, val: u8) -> Result<(), String> {
        if row >= self.size || col >= self.size {
            return Err("Position out of bounds".into());
        }
        if val > self.size as u8 {
            return Err(format!("Value {val} exceeds max {}", self.size));
        }

        self.grid[row][col] = if val == 0 { Cell::Empty } else { Cell::Filled(val) };
        Ok(())
    }

    /// Checks if the puzzle satisfies all Sudoku constraints.
    pub fn is_valid(&self) -> bool {
        self.valid_rows() && self.valid_cols() && self.valid_boxes()
    }

    /// Validates all rows.
    pub fn valid_rows(&self) -> bool {
        (0..self.size).all(|r| self.valid_unit(self.row_cells(r)))
    }

    /// Validates all columns.
    pub fn valid_cols(&self) -> bool {
        (0..self.size).all(|c| self.valid_unit(self.col_cells(c)))
    }

    /// Validates all boxes.
    pub fn valid_boxes(&self) -> bool {
        let bs = self.box_size;
        (0..bs).all(|br| (0..bs).all(|bc| self.valid_unit(self.box_cells(br, bc))))
    }

    fn valid_unit(&self, cells: impl Iterator<Item = Cell>) -> bool {
        let mut seen = HashSet::new();
        cells.filter_map(|c| c.value()).all(|v| seen.insert(v))
    }

    /// Checks if a value can be placed at (row, col).
    pub fn can_place(&self, row: usize, col: usize, val: u8) -> bool {
        if row >= self.size || col >= self.size || val == 0 || val > self.size as u8 {
            return false;
        }

        let target = Some(val);

        // Row
        if self.grid[row].iter().any(|c| c.value() == target) {
            return false;
        }

        // Column
        if (0..self.size).any(|r| self.grid[r][col].value() == target) {
            return false;
        }

        // Box
        let (br, bc) = self.box_origin(row, col);
        for r in br..br + self.box_size {
            for c in bc..bc + self.box_size {
                if self.grid[r][c].value() == target {
                    return false;
                }
            }
        }

        true
    }

    /// Returns true if all cells are filled.
    pub fn is_complete(&self) -> bool {
        self.grid.iter().flatten().all(|c| !c.is_empty())
    }

    /// Returns true if complete and valid.
    pub fn is_solved(&self) -> bool {
        self.is_complete() && self.is_valid()
    }

    /// Counts empty cells.
    pub fn empty_count(&self) -> usize {
        self.grid.iter().flatten().filter(|c| c.is_empty()).count()
    }

    /// Finds the first empty cell.
    pub fn first_empty(&self) -> Option<(usize, usize)> {
        for r in 0..self.size {
            for c in 0..self.size {
                if self.grid[r][c].is_empty() {
                    return Some((r, c));
                }
            }
        }
        None
    }

    /// Returns possible values for an empty cell.
    pub fn candidates(&self, row: usize, col: usize) -> HashSet<u8> {
        if !self.grid[row][col].is_empty() {
            return HashSet::new();
        }

        let mut cands: HashSet<u8> = (1..=self.size as u8).collect();

        // Row
        for c in &self.grid[row] {
            if let Some(v) = c.value() {
                cands.remove(&v);
            }
        }

        // Column
        for r in 0..self.size {
            if let Some(v) = self.grid[r][col].value() {
                cands.remove(&v);
            }
        }

        // Box
        let (br, bc) = self.box_origin(row, col);
        for r in br..br + self.box_size {
            for c in bc..bc + self.box_size {
                if let Some(v) = self.grid[r][c].value() {
                    cands.remove(&v);
                }
            }
        }

        cands
    }

    fn row_cells(&self, r: usize) -> impl Iterator<Item = Cell> + '_ {
        self.grid[r].iter().copied()
    }

    fn col_cells(&self, c: usize) -> impl Iterator<Item = Cell> + '_ {
        (0..self.size).map(move |r| self.grid[r][c])
    }

    fn box_cells(&self, br: usize, bc: usize) -> impl Iterator<Item = Cell> + '_ {
        let bs = self.box_size;
        let (sr, sc) = (br * bs, bc * bs);
        (sr..sr + bs).flat_map(move |r| (sc..sc + bs).map(move |c| self.grid[r][c]))
    }

    #[inline]
    fn box_origin(&self, r: usize, c: usize) -> (usize, usize) {
        let bs = self.box_size;
        (r / bs * bs, c / bs * bs)
    }
}

impl fmt::Display for Sudoku {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let bs = self.box_size;
        for r in 0..self.size {
            if r > 0 && r % bs == 0 {
                writeln!(f, "{}", "-".repeat(self.size * 2 + bs - 1))?;
            }
            for c in 0..self.size {
                if c > 0 && c % bs == 0 {
                    write!(f, "|")?;
                }
                match self.grid[r][c].value() {
                    None => write!(f, ". ")?,
                    Some(v) if v <= 9 => write!(f, "{v} ")?,
                    Some(v) => write!(f, "{} ", (b'A' + v - 10) as char)?,
                }
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

/// Parses a character to a cell value.
fn parse_char(ch: char, size: usize) -> Option<u8> {
    if let Some(d) = ch.to_digit(10) {
        let v = d as u8;
        (1..=size as u8).contains(&v).then_some(v)
    } else if ch.is_ascii_uppercase() {
        let v = ch as u8 - b'A' + 10;
        (10..=size as u8).contains(&v).then_some(v)
    } else {
        None
    }
}
