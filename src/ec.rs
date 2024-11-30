//! Everybody Codes common type definitions, and submodules with runner and CLI functions.

pub mod ansi;
pub mod cli;
pub mod error;
pub mod runner;

pub const START_SEASON: usize = 2024;
pub const MAX_SEASONS: usize = 2; // empty YYYY season also included as a template
pub const MAX_DAYS: usize = 20;

pub use error::PuzzleError;

/// The expected solution for a test case, containing all parts of the puzzle.
pub type PuzzleExpected<'a> = (&'a str, &'a str, &'a str);

/// The parameter type of `the solve()` functions: the puzzle input, already split to lines
pub type PuzzleInput<'a> = &'a [&'a str];

/// A candidate solution for a test case, containing one part of the puzzle.
pub type PuzzleSolution = String;

/// The return type of the `solve()` functions.
pub type PuzzleResult = Result<PuzzleSolution, PuzzleError>;

/// Each solution must have a `metadata()` function with this signature.
pub type MetaData<'a> = fn() -> PuzzleMetaData<'a>;

/// Each solution must have a `solve()` function with this signature.
pub type Solver = fn(PuzzleInput, part: usize) -> PuzzleResult;

/// An implemented puzzle: its `metadata()` and `solve()` functions, used by the `PUZZLES` constants in all season modules.
pub type Puzzle<'a> = (MetaData<'a>, Solver);

/// The array of the implemented puzzles, used by the [`PUZZLES`] constant in this (`ec`) module.
pub type Season<'a> = [Option<Puzzle<'a>>; MAX_DAYS];

/// Each solution must have a `metadata()` function, returning an instance of this struct.
pub struct PuzzleMetaData<'a> {
    pub year: usize,
    pub day: usize,
    pub title: &'a str,
    pub solution: PuzzleExpected<'a>,
    pub example_solutions: Vec<PuzzleExpected<'a>>,
}

/// Array of seasons containing the arrays of the implemented puzzles.
pub const PUZZLES: [Option<Season>; MAX_SEASONS] = [
    Some(crate::ec2024::PUZZLES),
    None, // Some(crate::ecYYYY::PUZZLES),
];
