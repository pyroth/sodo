use crate::sudoku::Sudoku;

pub trait SolvingStrategy {
    fn apply(&self, sudoku: &mut Sudoku) -> bool;
    fn name(&self) -> &'static str;
}

/// Naked Singles: If a cell has only one possible candidate, fill it
pub struct NakedSingles;

impl SolvingStrategy for NakedSingles {
    fn apply(&self, sudoku: &mut Sudoku) -> bool {
        let mut progress = false;
        
        for row in 0..sudoku.size {
            for col in 0..sudoku.size {
                if sudoku.grid[row][col].is_empty() {
                    let candidates = sudoku.get_candidates(row, col);
                    if candidates.len() == 1 {
                        let value = *candidates.iter().next().unwrap();
                        sudoku.set(row, col, value).unwrap();
                        progress = true;
                    }
                }
            }
        }
        
        progress
    }

    fn name(&self) -> &'static str {
        "Naked Singles"
    }
}

/// Hidden Singles: If a value can only go in one cell in a unit (row, column, or box)
pub struct HiddenSingles;

impl SolvingStrategy for HiddenSingles {
    fn apply(&self, sudoku: &mut Sudoku) -> bool {
        let mut progress = false;
        
        // Check rows
        for row in 0..sudoku.size {
            progress |= self.apply_to_row(sudoku, row);
        }
        
        // Check columns
        for col in 0..sudoku.size {
            progress |= self.apply_to_col(sudoku, col);
        }
        
        // Check boxes
        for box_row in 0..sudoku.box_size {
            for box_col in 0..sudoku.box_size {
                progress |= self.apply_to_box(sudoku, box_row, box_col);
            }
        }
        
        progress
    }

    fn name(&self) -> &'static str {
        "Hidden Singles"
    }
}

impl HiddenSingles {
    fn apply_to_row(&self, sudoku: &mut Sudoku, row: usize) -> bool {
        let mut progress = false;
        
        for value in 1..=sudoku.size as u8 {
            let mut possible_cells = Vec::new();
            
            for col in 0..sudoku.size {
                if sudoku.grid[row][col].is_empty() {
                    let candidates = sudoku.get_candidates(row, col);
                    if candidates.contains(&value) {
                        possible_cells.push(col);
                    }
                }
            }
            
            if possible_cells.len() == 1 {
                let col = possible_cells[0];
                sudoku.set(row, col, value).unwrap();
                progress = true;
            }
        }
        
        progress
    }
    
    fn apply_to_col(&self, sudoku: &mut Sudoku, col: usize) -> bool {
        let mut progress = false;
        
        for value in 1..=sudoku.size as u8 {
            let mut possible_cells = Vec::new();
            
            for row in 0..sudoku.size {
                if sudoku.grid[row][col].is_empty() {
                    let candidates = sudoku.get_candidates(row, col);
                    if candidates.contains(&value) {
                        possible_cells.push(row);
                    }
                }
            }
            
            if possible_cells.len() == 1 {
                let row = possible_cells[0];
                sudoku.set(row, col, value).unwrap();
                progress = true;
            }
        }
        
        progress
    }
    
    fn apply_to_box(&self, sudoku: &mut Sudoku, box_row: usize, box_col: usize) -> bool {
        let mut progress = false;
        
        for value in 1..=sudoku.size as u8 {
            let mut possible_cells = Vec::new();
            
            for row in box_row * sudoku.box_size..(box_row + 1) * sudoku.box_size {
                for col in box_col * sudoku.box_size..(box_col + 1) * sudoku.box_size {
                    if sudoku.grid[row][col].is_empty() {
                        let candidates = sudoku.get_candidates(row, col);
                        if candidates.contains(&value) {
                            possible_cells.push((row, col));
                        }
                    }
                }
            }
            
            if possible_cells.len() == 1 {
                let (row, col) = possible_cells[0];
                sudoku.set(row, col, value).unwrap();
                progress = true;
            }
        }
        
        progress
    }
}

pub fn get_all_strategies() -> Vec<Box<dyn SolvingStrategy>> {
    vec![
        Box::new(NakedSingles),
        Box::new(HiddenSingles),
    ]
}
