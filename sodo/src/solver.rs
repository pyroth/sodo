use crate::strategy::{get_all_strategies, SolvingStrategy};
use crate::sudoku::{Cell, Sudoku};
use rand::{rng, seq::SliceRandom, Rng};
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct SolverStats {
    pub strategies_used: HashMap<String, usize>,
    pub cells_filled: usize,
    pub iterations: usize,
    pub backtrack_steps: usize,
}

impl Default for SolverStats {
    fn default() -> Self {
        Self::new()
    }
}

impl SolverStats {
    pub fn new() -> Self {
        Self {
            strategies_used: HashMap::new(),
            cells_filled: 0,
            iterations: 0,
            backtrack_steps: 0,
        }
    }
}

pub struct SudokuSolver {
    strategies: Vec<Box<dyn SolvingStrategy>>,
    max_iterations: usize,
    use_backtracking: bool,
}

impl SudokuSolver {
    pub fn new() -> Self {
        Self {
            strategies: get_all_strategies(),
            max_iterations: 1000,
            use_backtracking: true,
        }
    }

    pub fn new_with_strategies(strategies: Vec<Box<dyn SolvingStrategy>>) -> Self {
        Self {
            strategies,
            max_iterations: 1000,
            use_backtracking: true,
        }
    }

    pub fn set_max_iterations(&mut self, max_iterations: usize) {
        self.max_iterations = max_iterations;
    }

    pub fn set_use_backtracking(&mut self, use_backtracking: bool) {
        self.use_backtracking = use_backtracking;
    }

    pub fn solve(&mut self, sudoku: Sudoku) -> Result<Sudoku, String> {
        self.solve_with_stats(sudoku).map(|(s, _)| s)
    }

    pub fn solve_with_stats(&mut self, mut sudoku: Sudoku) -> Result<(Sudoku, SolverStats), String> {
        if !sudoku.is_valid() {
            return Err("Invalid initial state".to_string());
        }

        let mut stats = SolverStats::new();

        if self.solve_with_strategies(&mut sudoku, &mut stats) {
            return Ok((sudoku, stats));
        }

        if self.use_backtracking && self.solve_with_backtracking(&mut sudoku, &mut stats) {
            return Ok((sudoku, stats));
        }

        if sudoku.is_complete() && sudoku.is_valid() {
            Ok((sudoku, stats))
        } else {
            Err("No solution found".to_string())
        }
    }

    fn solve_with_strategies(&self, sudoku: &mut Sudoku, stats: &mut SolverStats) -> bool {
        let mut progress = true;

        while progress && !sudoku.is_complete() && stats.iterations < self.max_iterations {
            progress = false;
            stats.iterations += 1;

            for strategy in &self.strategies {
                let initial_empty = sudoku.empty_count();

                if strategy.apply(sudoku) {
                    stats.cells_filled += initial_empty - sudoku.empty_count();
                    *stats.strategies_used.entry(strategy.name().to_string()).or_insert(0) += 1;
                    progress = true;

                    if !sudoku.is_valid() {
                        return false;
                    }
                }
            }
        }

        sudoku.is_complete() && sudoku.is_valid()
    }

    fn solve_with_backtracking(&self, sudoku: &mut Sudoku, stats: &mut SolverStats) -> bool {
        if sudoku.is_complete() {
            return sudoku.is_valid();
        }

        let Some((row, col)) = self.find_best_empty_cell(sudoku) else {
            return sudoku.is_valid();
        };

        for value in sudoku.get_candidates(row, col) {
            if sudoku.set(row, col, value).is_ok() {
                stats.backtrack_steps += 1;

                if sudoku.is_valid() && self.solve_with_backtracking(sudoku, stats) {
                    return true;
                }

                let _ = sudoku.set(row, col, 0);
            }
        }

        false
    }

    fn find_best_empty_cell(&self, sudoku: &Sudoku) -> Option<(usize, usize)> {
        let mut best_cell = None;
        let mut min_candidates = usize::MAX;

        for row in 0..sudoku.size {
            for col in 0..sudoku.size {
                if sudoku.grid[row][col].is_empty() {
                    let candidates = sudoku.get_candidates(row, col);
                    if candidates.len() < min_candidates {
                        min_candidates = candidates.len();
                        best_cell = Some((row, col));

                        // If we find a cell with no candidates, return immediately
                        if min_candidates == 0 {
                            return best_cell;
                        }
                    }
                }
            }
        }

        best_cell
    }


    pub fn get_hint(&mut self, sudoku: &mut Sudoku) -> Option<(usize, usize, u8)> {
        // Try to find a cell that can be filled using logical strategies
        for row in 0..sudoku.size {
            for col in 0..sudoku.size {
                if sudoku.grid[row][col].is_empty() {
                    let candidates = sudoku.get_candidates(row, col);
                    if candidates.len() == 1 {
                        let value = *candidates.iter().next().unwrap();
                        return Some((row, col, value));
                    }
                }
            }
        }

        // If no naked singles, try hidden singles
        for strategy in &self.strategies {
            if strategy.name() == "Hidden Singles" {
                let mut temp_sudoku = sudoku.clone();
                if strategy.apply(&mut temp_sudoku) {
                    // Find the difference
                    for row in 0..sudoku.size {
                        for col in 0..sudoku.size {
                            if sudoku.grid[row][col] != temp_sudoku.grid[row][col]
                                && let Some(value) = temp_sudoku.grid[row][col].value()
                            {
                                return Some((row, col, value));
                            }
                        }
                    }
                }
            }
        }

        None
    }

    pub fn validate_solution(&self, sudoku: &Sudoku) -> bool {
        sudoku.is_complete() && sudoku.is_valid()
    }

    pub fn count_solutions(&mut self, mut sudoku: Sudoku, max_solutions: usize) -> usize {
        let mut count = 0;
        Self::count_solutions_recursive(&mut sudoku, &mut count, max_solutions);
        count
    }

    fn count_solutions_recursive(
        sudoku: &mut Sudoku,
        count: &mut usize,
        max_solutions: usize,
    ) {
        if *count >= max_solutions {
            return;
        }

        if sudoku.is_complete() {
            if sudoku.is_valid() {
                *count += 1;
            }
            return;
        }

        let Some((row, col)) = sudoku.find_empty_cell() else {
            return;
        };

        for value in sudoku.get_candidates(row, col) {
            if sudoku.set(row, col, value).is_ok() && sudoku.is_valid() {
                Self::count_solutions_recursive(sudoku, count, max_solutions);
            }
            let _ = sudoku.set(row, col, 0);
        }
    }

    pub fn generate_puzzle(
        &mut self,
        size: usize,
        difficulty: Difficulty,
    ) -> Result<Sudoku, String> {
        let mut sudoku = Sudoku::new(size);
        let mut rng = rng();

        // Fill the diagonal boxes first (they don't interfere with each other)
        // Randomize the order of filling diagonal boxes for more variety
        let box_size = sudoku.box_size;
        let mut diagonal_indices: Vec<usize> = (0..box_size).collect();
        diagonal_indices.shuffle(&mut rng);

        for &i in &diagonal_indices {
            self.fill_box(&mut sudoku, i * box_size, i * box_size)?;
        }

        // Solve the complete puzzle
        let full_solution = self.solve(sudoku.clone())?;

        // Remove cells based on difficulty with some randomization
        let base_cells_to_remove = match difficulty {
            Difficulty::Easy => size * size * 40 / 100, // Remove 40%
            Difficulty::Medium => size * size * 50 / 100, // Remove 50%
            Difficulty::Hard => size * size * 60 / 100, // Remove 60%
            Difficulty::Expert => size * size * 70 / 100, // Remove 70%
        };

        // Add some randomization to the number of cells removed (±5%)
        let variation = (base_cells_to_remove as f32 * 0.05) as usize;
        let cells_to_remove = if variation > 0 {
            let min_remove = base_cells_to_remove.saturating_sub(variation);
            let max_remove = (base_cells_to_remove + variation).min(size * size - 17);
            rng.random_range(min_remove..=max_remove)
        } else {
            base_cells_to_remove
        };

        self.remove_cells_symmetrically(full_solution, cells_to_remove)
    }

    fn fill_box(
        &self,
        sudoku: &mut Sudoku,
        start_row: usize,
        start_col: usize,
    ) -> Result<(), String> {
        let mut values: Vec<u8> = (1..=sudoku.size as u8).collect();

        // Shuffle values randomly
        values.shuffle(&mut rng());

        let mut idx = 0;
        for row in start_row..start_row + sudoku.box_size {
            for col in start_col..start_col + sudoku.box_size {
                sudoku.set(row, col, values[idx])?;
                idx += 1;
            }
        }

        Ok(())
    }

    fn remove_cells_symmetrically(
        &self,
        mut sudoku: Sudoku,
        cells_to_remove: usize,
    ) -> Result<Sudoku, String> {
        let mut removed = 0;
        let size = sudoku.size;
        let mut rng = rng();

        // Create a list of all cell positions
        let mut positions: Vec<(usize, usize)> = (0..size)
            .flat_map(|i| (0..size).map(move |j| (i, j)))
            .collect();

        // Shuffle the positions randomly
        positions.shuffle(&mut rng);

        // Remove cells randomly while maintaining some symmetry
        for &(row, col) in &positions {
            if removed >= cells_to_remove {
                break;
            }

            // Remove current cell
            if sudoku.grid[row][col] != Cell::Empty {
                sudoku.grid[row][col] = Cell::Empty;
                removed += 1;

                // Optionally remove symmetric cell (not always for more variety)
                if removed < cells_to_remove && rng.random_bool(0.7) {
                    let sym_row = size - 1 - row;
                    let sym_col = size - 1 - col;
                    if (sym_row != row || sym_col != col)
                        && sudoku.grid[sym_row][sym_col] != Cell::Empty {
                            sudoku.grid[sym_row][sym_col] = Cell::Empty;
                            removed += 1;
                        }
                }
            }
        }

        Ok(sudoku)
    }

    pub fn solve_step(&self, sudoku: &mut Sudoku) -> bool {
        for strategy in &self.strategies {
            if strategy.apply(sudoku) {
                return true;
            }
        }
        false
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Difficulty {
    Easy,
    Medium,
    Hard,
    Expert,
}

impl Default for SudokuSolver {
    fn default() -> Self {
        Self::new()
    }
}
