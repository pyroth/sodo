use clap::{Parser, Subcommand, ValueEnum};
use sodo::{Difficulty, Solver, Sudoku};
use std::{fs, path::PathBuf, process};

#[derive(Parser)]
#[command(name = "sodo", version, about = "Sudoku solver and generator")]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// Solve a puzzle
    #[command(visible_alias = "s")]
    Solve {
        /// Puzzle string (81 chars for 9x9)
        puzzle: Option<String>,
        /// Read puzzle from file
        #[arg(short, long, conflicts_with = "puzzle")]
        file: Option<PathBuf>,
        /// Grid size
        #[arg(short, long, default_value = "9")]
        size: usize,
    },
    /// Generate a new puzzle
    #[command(visible_alias = "g")]
    Generate {
        /// Grid size
        #[arg(short, long, default_value = "9")]
        size: usize,
        /// Difficulty level
        #[arg(short, long, default_value = "medium")]
        difficulty: Level,
    },
    /// Validate a puzzle
    #[command(visible_alias = "v")]
    Validate {
        /// Puzzle string
        puzzle: String,
        /// Grid size
        #[arg(short, long, default_value = "9")]
        size: usize,
    },
    /// Get a hint for the next move
    #[command(visible_alias = "h")]
    Hint {
        /// Puzzle string
        puzzle: String,
        /// Grid size
        #[arg(short, long, default_value = "9")]
        size: usize,
    },
}

#[derive(Clone, ValueEnum)]
enum Level {
    Easy,
    Medium,
    Hard,
    Expert,
}

impl From<Level> for Difficulty {
    fn from(level: Level) -> Self {
        match level {
            Level::Easy => Difficulty::Easy,
            Level::Medium => Difficulty::Medium,
            Level::Hard => Difficulty::Hard,
            Level::Expert => Difficulty::Expert,
        }
    }
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Command::Solve { puzzle, file, size } => solve(puzzle, file, size),
        Command::Generate { size, difficulty } => generate(size, difficulty.into()),
        Command::Validate { puzzle, size } => validate(&puzzle, size),
        Command::Hint { puzzle, size } => hint(&puzzle, size),
    }
}

fn solve(puzzle: Option<String>, file: Option<PathBuf>, size: usize) {
    let input = match (puzzle, file) {
        (Some(p), _) => p,
        (_, Some(f)) => fs::read_to_string(&f).unwrap_or_else(|e| {
            eprintln!("Error reading {}: {e}", f.display());
            process::exit(1);
        }),
        _ => {
            eprintln!("Provide puzzle string or --file");
            process::exit(1);
        }
    };

    let sudoku = parse(input.trim(), size);
    println!("Puzzle:\n{sudoku}");

    let mut solver = Solver::new();
    match solver.solve_with_stats(sudoku) {
        Ok((solution, stats)) => {
            println!("Solution:\n{solution}");
            println!(
                "Stats: {} iters, {} cells, {} backtracks",
                stats.iterations, stats.cells_filled, stats.backtracks
            );
        }
        Err(e) => {
            eprintln!("Failed: {e}");
            process::exit(1);
        }
    }
}

fn generate(size: usize, difficulty: Difficulty) {
    let mut solver = Solver::new();
    match solver.generate(size, difficulty) {
        Ok(puzzle) => {
            println!("{puzzle}");
            println!("{}", puzzle.to_string_compact());
        }
        Err(e) => {
            eprintln!("Failed: {e}");
            process::exit(1);
        }
    }
}

fn validate(puzzle: &str, size: usize) {
    let sudoku = parse(puzzle, size);
    println!("{sudoku}");

    if sudoku.is_valid() {
        let status = if sudoku.is_complete() {
            " and complete!"
        } else {
            ""
        };
        println!("Valid{status}");
    } else {
        println!("Invalid!");
        process::exit(1);
    }
}

fn hint(puzzle: &str, size: usize) {
    let sudoku = parse(puzzle, size);
    let solver = Solver::new();

    match solver.hint(&sudoku) {
        Some((r, c, v)) => println!("Place {v} at row {}, col {}", r + 1, c + 1),
        None => println!("No hint available"),
    }
}

fn parse(s: &str, size: usize) -> Sudoku {
    Sudoku::from_string(s, size).unwrap_or_else(|e| {
        eprintln!("Invalid puzzle: {e}");
        process::exit(1)
    })
}
