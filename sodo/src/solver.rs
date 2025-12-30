use crate::strategy::{all as all_strategies, Strategy};
use crate::sodo::{Cell, Sudoku};
use rand::{rng, seq::SliceRandom, Rng};
use std::collections::HashMap;

/// Statistics collected during solving.
#[derive(Debug, Clone, Default)]
pub struct Stats {
    pub strategies_used: HashMap<String, usize>,
    pub cells_filled: usize,
    pub iterations: usize,
    pub backtracks: usize,
}

/// Puzzle difficulty level.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Difficulty {
    Easy,
    Medium,
    Hard,
    Expert,
}

/// A Sudoku solver using logical strategies and optional backtracking.
pub struct Solver {
    strategies: Vec<Box<dyn Strategy>>,
    max_iters: usize,
    backtrack: bool,
}

impl Default for Solver {
    fn default() -> Self {
        Self::new()
    }
}

impl Solver {
    /// Creates a new solver with default strategies.
    pub fn new() -> Self {
        Self {
            strategies: all_strategies(),
            max_iters: 1000,
            backtrack: true,
        }
    }

    /// Creates a solver with custom strategies.
    pub fn with_strategies(strategies: Vec<Box<dyn Strategy>>) -> Self {
        Self {
            strategies,
            max_iters: 1000,
            backtrack: true,
        }
    }

    /// Sets maximum iterations for logical solving.
    pub fn max_iterations(mut self, n: usize) -> Self {
        self.max_iters = n;
        self
    }

    /// Enables or disables backtracking.
    pub fn use_backtracking(mut self, enabled: bool) -> Self {
        self.backtrack = enabled;
        self
    }

    /// Solves the puzzle, returning the solution.
    pub fn solve(&mut self, sudoku: Sudoku) -> Result<Sudoku, String> {
        self.solve_with_stats(sudoku).map(|(s, _)| s)
    }

    /// Solves the puzzle, returning solution and statistics.
    pub fn solve_with_stats(&mut self, mut sudoku: Sudoku) -> Result<(Sudoku, Stats), String> {
        if !sudoku.is_valid() {
            return Err("Invalid initial state".into());
        }

        let mut stats = Stats::default();

        if self.apply_strategies(&mut sudoku, &mut stats) {
            return Ok((sudoku, stats));
        }

        if self.backtrack && self.backtrack_solve(&mut sudoku, &mut stats) {
            return Ok((sudoku, stats));
        }

        if sudoku.is_solved() {
            Ok((sudoku, stats))
        } else {
            Err("No solution found".into())
        }
    }

    fn apply_strategies(&self, sudoku: &mut Sudoku, stats: &mut Stats) -> bool {
        let mut progress = true;

        while progress && !sudoku.is_complete() && stats.iterations < self.max_iters {
            progress = false;
            stats.iterations += 1;

            for strategy in &self.strategies {
                let before = sudoku.empty_count();

                if strategy.apply(sudoku) {
                    stats.cells_filled += before - sudoku.empty_count();
                    *stats.strategies_used.entry(strategy.name().into()).or_default() += 1;
                    progress = true;

                    if !sudoku.is_valid() {
                        return false;
                    }
                }
            }
        }

        sudoku.is_solved()
    }

    fn backtrack_solve(&self, sudoku: &mut Sudoku, stats: &mut Stats) -> bool {
        if sudoku.is_complete() {
            return sudoku.is_valid();
        }

        let Some((r, c)) = self.find_mrv_cell(sudoku) else {
            return sudoku.is_valid();
        };

        for val in sudoku.candidates(r, c) {
            if sudoku.set(r, c, val).is_ok() {
                stats.backtracks += 1;

                if sudoku.is_valid() && self.backtrack_solve(sudoku, stats) {
                    return true;
                }

                let _ = sudoku.set(r, c, 0);
            }
        }

        false
    }

    /// Finds empty cell with minimum remaining values (MRV heuristic).
    fn find_mrv_cell(&self, sudoku: &Sudoku) -> Option<(usize, usize)> {
        let mut best = None;
        let mut min_cands = usize::MAX;

        for r in 0..sudoku.size {
            for c in 0..sudoku.size {
                if sudoku.grid[r][c].is_empty() {
                    let n = sudoku.candidates(r, c).len();
                    if n < min_cands {
                        min_cands = n;
                        best = Some((r, c));
                        if n == 0 {
                            return best;
                        }
                    }
                }
            }
        }

        best
    }

    /// Returns a hint: (row, col, value) for the next logical move.
    pub fn hint(&self, sudoku: &Sudoku) -> Option<(usize, usize, u8)> {
        // Try naked singles first
        for r in 0..sudoku.size {
            for c in 0..sudoku.size {
                if sudoku.grid[r][c].is_empty() {
                    let cands = sudoku.candidates(r, c);
                    if cands.len() == 1 {
                        return Some((r, c, *cands.iter().next().unwrap()));
                    }
                }
            }
        }

        // Try hidden singles
        for strategy in &self.strategies {
            if strategy.name() == "Hidden Singles" {
                let mut temp = sudoku.clone();
                if strategy.apply(&mut temp) {
                    for r in 0..sudoku.size {
                        for c in 0..sudoku.size {
                            if sudoku.grid[r][c] != temp.grid[r][c]
                                && let Some(v) = temp.grid[r][c].value()
                            {
                                return Some((r, c, v));
                            }
                        }
                    }
                }
            }
        }

        None
    }

    /// Counts solutions up to a maximum.
    pub fn count_solutions(&self, mut sudoku: Sudoku, max: usize) -> usize {
        let mut count = 0;
        Self::count_recursive(&mut sudoku, &mut count, max);
        count
    }

    fn count_recursive(sudoku: &mut Sudoku, count: &mut usize, max: usize) {
        if *count >= max {
            return;
        }

        if sudoku.is_complete() {
            if sudoku.is_valid() {
                *count += 1;
            }
            return;
        }

        let Some((r, c)) = sudoku.first_empty() else {
            return;
        };

        for val in sudoku.candidates(r, c) {
            if sudoku.set(r, c, val).is_ok() && sudoku.is_valid() {
                Self::count_recursive(sudoku, count, max);
            }
            let _ = sudoku.set(r, c, 0);
        }
    }

    /// Generates a puzzle of the given size and difficulty.
    pub fn generate(&mut self, size: usize, difficulty: Difficulty) -> Result<Sudoku, String> {
        let mut sudoku = Sudoku::new(size);
        let mut rng = rng();

        // Fill diagonal boxes first (they don't affect each other)
        let bs = sudoku.box_size;
        let mut diag: Vec<usize> = (0..bs).collect();
        diag.shuffle(&mut rng);

        for &i in &diag {
            self.fill_box(&mut sudoku, i * bs)?;
        }

        // Solve to get complete grid
        let solution = self.solve(sudoku.clone())?;

        // Calculate cells to remove based on difficulty
        let total = size * size;
        let base_remove = match difficulty {
            Difficulty::Easy => total * 40 / 100,
            Difficulty::Medium => total * 50 / 100,
            Difficulty::Hard => total * 60 / 100,
            Difficulty::Expert => total * 70 / 100,
        };

        // Add ±5% variation
        let var = (base_remove as f32 * 0.05) as usize;
        let to_remove = if var > 0 {
            let min = base_remove.saturating_sub(var);
            let max = (base_remove + var).min(total - 17);
            rng.random_range(min..=max)
        } else {
            base_remove
        };

        self.remove_cells(solution, to_remove)
    }

    fn fill_box(&self, sudoku: &mut Sudoku, start: usize) -> Result<(), String> {
        let bs = sudoku.box_size;
        let mut vals: Vec<u8> = (1..=sudoku.size as u8).collect();
        vals.shuffle(&mut rng());

        let mut i = 0;
        for r in start..start + bs {
            for c in start..start + bs {
                sudoku.set(r, c, vals[i])?;
                i += 1;
            }
        }

        Ok(())
    }

    fn remove_cells(&self, mut sudoku: Sudoku, to_remove: usize) -> Result<Sudoku, String> {
        let size = sudoku.size;
        let mut rng = rng();
        let mut removed = 0;

        let mut positions: Vec<_> = (0..size)
            .flat_map(|r| (0..size).map(move |c| (r, c)))
            .collect();
        positions.shuffle(&mut rng);

        for (r, c) in positions {
            if removed >= to_remove {
                break;
            }

            if sudoku.grid[r][c] != Cell::Empty {
                sudoku.grid[r][c] = Cell::Empty;
                removed += 1;

                // Remove symmetric cell with 70% probability
                if removed < to_remove && rng.random_bool(0.7) {
                    let (sr, sc) = (size - 1 - r, size - 1 - c);
                    if (sr != r || sc != c) && sudoku.grid[sr][sc] != Cell::Empty {
                        sudoku.grid[sr][sc] = Cell::Empty;
                        removed += 1;
                    }
                }
            }
        }

        Ok(sudoku)
    }

    /// Applies one strategy step. Returns true if progress was made.
    pub fn step(&self, sudoku: &mut Sudoku) -> bool {
        self.strategies.iter().any(|s| s.apply(sudoku))
    }
}
