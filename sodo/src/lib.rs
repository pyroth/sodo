mod solver;
mod strategy;
mod sodo;

pub use solver::{Difficulty, Solver, Stats};
pub use strategy::{all as all_strategies, Strategy};
pub use sodo::{Cell, Sudoku};

use std::{fs, process};

fn parse_size(s: &str) -> usize {
    s.parse().unwrap_or_else(|_| {
        eprintln!("Invalid size: {s}");
        process::exit(1)
    })
}

fn parse_puzzle(s: &str, size: usize) -> Sudoku {
    Sudoku::from_string(s, size).unwrap_or_else(|e| {
        eprintln!("Error parsing puzzle: {e}");
        process::exit(1)
    })
}

/// Solves a puzzle from string.
pub fn solve_puzzle(puzzle_str: &str, size_str: &str) {
    let size = parse_size(size_str);
    let puzzle = parse_puzzle(puzzle_str, size);

    println!("Original puzzle:");
    println!("{puzzle}");

    let mut solver = Solver::new();

    match solver.solve_with_stats(puzzle) {
        Ok((solution, stats)) => {
            println!("Solution found!");
            println!("{solution}");
            println!("\nStatistics:");
            println!("  Iterations: {}", stats.iterations);
            println!("  Cells filled: {}", stats.cells_filled);
            println!("  Backtracks: {}", stats.backtracks);
            println!("  Strategies:");
            for (name, count) in &stats.strategies_used {
                println!("    {name}: {count}");
            }
        }
        Err(e) => {
            eprintln!("Failed to solve: {e}");
            process::exit(1);
        }
    }
}

/// Solves a puzzle from file.
pub fn solve_from_file(path: &str, size_str: &str) {
    let content = fs::read_to_string(path)
        .map(|s| s.trim().to_string())
        .unwrap_or_else(|e| {
            eprintln!("Error reading file: {e}");
            process::exit(1)
        });
    solve_puzzle(&content, size_str);
}

/// Generates a puzzle.
pub fn generate_puzzle(size_str: &str, difficulty_str: &str) {
    let size = parse_size(size_str);
    let difficulty = match difficulty_str.to_lowercase().as_str() {
        "easy" => Difficulty::Easy,
        "medium" => Difficulty::Medium,
        "hard" => Difficulty::Hard,
        "expert" => Difficulty::Expert,
        _ => {
            eprintln!("Invalid difficulty: {difficulty_str}");
            process::exit(1)
        }
    };

    let mut solver = Solver::new();

    match solver.generate(size, difficulty) {
        Ok(puzzle) => {
            println!("Generated {difficulty_str} puzzle ({size}x{size}):");
            println!("{puzzle}");
        }
        Err(e) => {
            eprintln!("Failed to generate: {e}");
            process::exit(1);
        }
    }
}

/// Validates a puzzle.
pub fn validate_puzzle(puzzle_str: &str, size_str: &str) {
    let size = parse_size(size_str);
    let puzzle = parse_puzzle(puzzle_str, size);

    println!("Puzzle:");
    println!("{puzzle}");

    if puzzle.is_valid() {
        println!("Valid!");
        if puzzle.is_complete() {
            println!("Complete and solved!");
        } else {
            println!("Not yet complete.");
        }
    } else {
        println!("Invalid!");
        if !puzzle.valid_rows() {
            println!("  - Invalid rows");
        }
        if !puzzle.valid_cols() {
            println!("  - Invalid columns");
        }
        if !puzzle.valid_boxes() {
            println!("  - Invalid boxes");
        }
    }
}

/// Gets a hint for the puzzle.
pub fn get_hint(puzzle_str: &str, size_str: &str) {
    let size = parse_size(size_str);
    let mut puzzle = parse_puzzle(puzzle_str, size);

    println!("Current puzzle:");
    println!("{puzzle}");

    let solver = Solver::new();

    match solver.hint(&puzzle) {
        Some((r, c, val)) => {
            println!("Hint: Place {val} at ({}, {})", r + 1, c + 1);
            let _ = puzzle.set(r, c, val);
            println!("\nWith hint applied:");
            println!("{puzzle}");
        }
        None => {
            println!("No obvious hint available.");
        }
    }
}
