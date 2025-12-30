use sodo::{Difficulty, Solver, Sudoku};
use wasm_bindgen::prelude::*;

/// Solves a Sudoku puzzle and returns the solution.
///
/// @param puzzle - Puzzle string (81 chars for 9x9). Use `.` or `0` for empty cells.
/// @param size - Grid size. Defaults to `9`.
/// @returns Solution as compact string (81 chars for 9x9).
/// @throws {string} If puzzle is invalid or unsolvable.
/// @example
/// ```ts
/// const solution = solve("53..7....6..195....98....6.8...6...34..8.3..17...2...6.6....28....419..5....8..79");
/// ```
#[wasm_bindgen]
pub fn solve(puzzle: &str, size: Option<usize>) -> Result<String, String> {
    let size = size.unwrap_or(9);
    let sudoku = Sudoku::from_string(puzzle, size)?;
    let mut solver = Solver::new();
    solver.solve(sudoku).map(|s| s.to_string_compact())
}

/// Generates a new Sudoku puzzle with the specified difficulty.
///
/// @param difficulty - Difficulty level: `"easy"`, `"medium"`, `"hard"`, or `"expert"`. Defaults to `"medium"`.
/// @param size - Grid size. Defaults to `9`.
/// @returns Generated puzzle as compact string.
/// @throws {string} If difficulty is invalid or generation fails.
/// @example
/// ```ts
/// const puzzle = generate("hard");
/// ```
#[wasm_bindgen]
pub fn generate(difficulty: Option<String>, size: Option<usize>) -> Result<String, String> {
    let size = size.unwrap_or(9);
    let diff = match difficulty.as_deref().unwrap_or("medium") {
        "easy" => Difficulty::Easy,
        "medium" => Difficulty::Medium,
        "hard" => Difficulty::Hard,
        "expert" => Difficulty::Expert,
        other => return Err(format!("Invalid difficulty: {other}")),
    };
    let mut solver = Solver::new();
    solver.generate(size, diff).map(|s| s.to_string_compact())
}

/// Validates a Sudoku puzzle for constraint violations.
///
/// @param puzzle - Puzzle string to validate.
/// @param size - Grid size. Defaults to `9`.
/// @returns `true` if puzzle has no constraint violations (rows, columns, boxes).
/// @throws {string} If puzzle string is malformed.
/// @example
/// ```ts
/// if (validate(puzzle)) {
///   console.log("Puzzle is valid");
/// }
/// ```
#[wasm_bindgen]
pub fn validate(puzzle: &str, size: Option<usize>) -> Result<bool, String> {
    let size = size.unwrap_or(9);
    let sudoku = Sudoku::from_string(puzzle, size)?;
    Ok(sudoku.is_valid())
}

/// Checks if a puzzle has a valid solution.
///
/// @param puzzle - Puzzle string to check.
/// @param size - Grid size. Defaults to `9`.
/// @returns `true` if puzzle is solvable, `false` otherwise.
/// @throws {string} If puzzle string is malformed.
/// @example
/// ```ts
/// if (is_solvable(puzzle)) {
///   const solution = solve(puzzle);
/// }
/// ```
#[wasm_bindgen]
pub fn is_solvable(puzzle: &str, size: Option<usize>) -> Result<bool, String> {
    let size = size.unwrap_or(9);
    let sudoku = Sudoku::from_string(puzzle, size)?;
    let mut solver = Solver::new();
    Ok(solver.solve(sudoku).is_ok())
}

/// Gets a hint for the next logical move.
///
/// @param puzzle - Current puzzle state.
/// @param size - Grid size. Defaults to `9`.
/// @returns Hint object `{ row: number, col: number, value: number }` (1-indexed), or `null` if no hint available.
/// @throws {string} If puzzle string is malformed.
/// @example
/// ```ts
/// const h = hint(puzzle);
/// if (h) {
///   console.log(`Place ${h.value} at row ${h.row}, col ${h.col}`);
/// }
/// ```
#[wasm_bindgen]
pub fn hint(puzzle: &str, size: Option<usize>) -> Result<JsValue, String> {
    let size = size.unwrap_or(9);
    let sudoku = Sudoku::from_string(puzzle, size)?;
    let solver = Solver::new();

    match solver.hint(&sudoku) {
        Some((r, c, v)) => {
            let obj = js_sys::Object::new();
            js_sys::Reflect::set(&obj, &"row".into(), &(r as u32 + 1).into()).unwrap();
            js_sys::Reflect::set(&obj, &"col".into(), &(c as u32 + 1).into()).unwrap();
            js_sys::Reflect::set(&obj, &"value".into(), &(v as u32).into()).unwrap();
            Ok(obj.into())
        }
        None => Ok(JsValue::NULL),
    }
}

/// Formats a puzzle string as a human-readable grid.
///
/// @param puzzle - Puzzle string to format.
/// @param size - Grid size. Defaults to `9`.
/// @returns Multi-line formatted grid with box separators.
/// @throws {string} If puzzle string is malformed.
/// @example
/// ```ts
/// console.log(format(puzzle));
/// // Output:
/// // 5 3 . |. 7 . |. . .
/// // 6 . . |1 9 5 |. . .
/// // ...
/// ```
#[wasm_bindgen]
pub fn format(puzzle: &str, size: Option<usize>) -> Result<String, String> {
    let size = size.unwrap_or(9);
    let sudoku = Sudoku::from_string(puzzle, size)?;
    Ok(sudoku.to_string())
}
