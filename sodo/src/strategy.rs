use crate::Sudoku;

/// A solving strategy that can make progress on a puzzle.
pub trait Strategy: Send + Sync {
    /// Attempts to apply the strategy. Returns true if progress was made.
    fn apply(&self, sudoku: &mut Sudoku) -> bool;

    /// Returns the strategy name.
    fn name(&self) -> &'static str;
}

/// Returns all available strategies in priority order.
pub fn all() -> Vec<Box<dyn Strategy>> {
    vec![Box::new(NakedSingles), Box::new(HiddenSingles)]
}

/// Fills cells that have only one candidate.
pub struct NakedSingles;

impl Strategy for NakedSingles {
    fn name(&self) -> &'static str {
        "Naked Singles"
    }

    fn apply(&self, sudoku: &mut Sudoku) -> bool {
        let mut progress = false;

        for r in 0..sudoku.size {
            for c in 0..sudoku.size {
                if sudoku.grid[r][c].is_empty() {
                    let cands = sudoku.candidates(r, c);
                    if cands.len() == 1 {
                        let val = *cands.iter().next().unwrap();
                        let _ = sudoku.set(r, c, val);
                        progress = true;
                    }
                }
            }
        }

        progress
    }
}

/// Fills cells where a value can only go in one place within a unit.
pub struct HiddenSingles;

impl Strategy for HiddenSingles {
    fn name(&self) -> &'static str {
        "Hidden Singles"
    }

    fn apply(&self, sudoku: &mut Sudoku) -> bool {
        let mut progress = false;

        for i in 0..sudoku.size {
            progress |= apply_row(sudoku, i);
            progress |= apply_col(sudoku, i);
        }

        let bs = sudoku.box_size;
        for br in 0..bs {
            for bc in 0..bs {
                progress |= apply_box(sudoku, br, bc);
            }
        }

        progress
    }
}

fn apply_row(sudoku: &mut Sudoku, row: usize) -> bool {
    let mut progress = false;

    for val in 1..=sudoku.size as u8 {
        let cols: Vec<_> = (0..sudoku.size)
            .filter(|&c| sudoku.grid[row][c].is_empty() && sudoku.candidates(row, c).contains(&val))
            .collect();

        if cols.len() == 1 {
            let _ = sudoku.set(row, cols[0], val);
            progress = true;
        }
    }

    progress
}

fn apply_col(sudoku: &mut Sudoku, col: usize) -> bool {
    let mut progress = false;

    for val in 1..=sudoku.size as u8 {
        let rows: Vec<_> = (0..sudoku.size)
            .filter(|&r| sudoku.grid[r][col].is_empty() && sudoku.candidates(r, col).contains(&val))
            .collect();

        if rows.len() == 1 {
            let _ = sudoku.set(rows[0], col, val);
            progress = true;
        }
    }

    progress
}

fn apply_box(sudoku: &mut Sudoku, br: usize, bc: usize) -> bool {
    let mut progress = false;
    let bs = sudoku.box_size;
    let (sr, sc) = (br * bs, bc * bs);

    for val in 1..=sudoku.size as u8 {
        let cells: Vec<_> = (sr..sr + bs)
            .flat_map(|r| (sc..sc + bs).map(move |c| (r, c)))
            .filter(|&(r, c)| sudoku.grid[r][c].is_empty() && sudoku.candidates(r, c).contains(&val))
            .collect();

        if cells.len() == 1 {
            let (r, c) = cells[0];
            let _ = sudoku.set(r, c, val);
            progress = true;
        }
    }

    progress
}
