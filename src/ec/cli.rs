//! The CLI runner.

use super::ansi::{ANSI_RED_INV, ANSI_RESET, ANSI_YELLOW};
use super::runner::run_puzzles;
use super::{MAX_DAYS, MAX_SEASONS, START_SEASON};
use std::env;
use std::process::ExitCode;

pub const MSG_TITLE: &str = "Everybody Codes - solutions in Rust, (c) 2025 by TBali";
pub const RUN_PARALLEL: bool = true;

const ARG_HELP: &str = "--help";
const ARG_VERSION: &str = "--version";

/// The main CLI runner.
pub fn run() -> ExitCode {
    println!("{}\n", MSG_TITLE);
    let args = env::args().collect::<Vec<_>>();
    match parse_args(&args) {
        Err(msg) => {
            if msg == ARG_VERSION {
                return ExitCode::SUCCESS;
            }
            print_help();
            if msg == ARG_HELP {
                return ExitCode::SUCCESS;
            }
            println!("{ANSI_RED_INV}[ERROR]{ANSI_RESET} {}\n", msg);
            ExitCode::from(2)
        }
        Ok((year, day)) => {
            let result = run_puzzles(year, day, RUN_PARALLEL);
            if result {
                ExitCode::SUCCESS
            } else {
                ExitCode::from(1)
            }
        }
    }
}

fn print_help() {
    println!(
        "\
        You can run the solution(s) for a specific puzzle, for a full season, or for all seasons.\n\
        Usage:  ./ec {ANSI_YELLOW}[year] [day]{ANSI_RESET}
        "
    );
}

/// Tries to parse CLI arguments to year and day, no output.
fn parse_args(args: &[String]) -> Result<(Option<usize>, Option<usize>), &'static str> {
    match args.len() {
        1 => Ok((None, None)),
        2 => {
            if args[1] == ARG_HELP {
                return Err(ARG_HELP);
            }
            if args[1] == ARG_VERSION {
                return Err(ARG_VERSION);
            }
            let year = args[1]
                .parse::<usize>()
                .map_err(|_| "Invalid argument: year must be integer")?;
            if !(START_SEASON..START_SEASON + MAX_SEASONS).contains(&year) {
                return Err("Invalid argument: year out of range (valid: 2024 only)");
            }
            Ok((Some(year), None))
        }
        3 => {
            let year = args[1]
                .parse::<usize>()
                .map_err(|_| "Invalid argument: year must be integer")?;
            let day = args[2]
                .parse::<usize>()
                .map_err(|_| "Invalid argument: day must be integer")?;
            if !(START_SEASON..START_SEASON + MAX_SEASONS).contains(&year) {
                return Err("Invalid argument: year out of range (valid: 2024 only)");
            }
            if !(1..=MAX_DAYS).contains(&day) {
                return Err("Invalid argument: day out of range (valid: 1-20)");
            }
            Ok((Some(year), Some(day)))
        }
        _ => Err("Too many arguments"),
    }
}

// ------------------------------------------------------------
#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn parse_args_invalid_arguments() {
        let args = [
            String::from("ec"),
            String::from("2024"),
            String::from("2"),
            String::from("3"),
        ];
        assert_eq!(parse_args(&args), Err("Too many arguments"));
        let args = [String::from("ec"), String::from("year")];
        assert_eq!(
            parse_args(&args),
            Err("Invalid argument: year must be integer")
        );
        let args = [String::from("ec"), String::from("2023")];
        assert_eq!(
            parse_args(&args),
            Err("Invalid argument: year out of range (valid: 2024 only)")
        );
        let args = [String::from("ec"), String::from("2030")];
        assert_eq!(
            parse_args(&args),
            Err("Invalid argument: year out of range (valid: 2024 only)")
        );
        let args = [String::from("ec"), String::from("year"), String::from("2")];
        assert_eq!(
            parse_args(&args),
            Err("Invalid argument: year must be integer")
        );
        let args = [String::from("ec"), String::from("2023"), String::from("2")];
        assert_eq!(
            parse_args(&args),
            Err("Invalid argument: year out of range (valid: 2024 only)")
        );
        let args = [String::from("ec"), String::from("2030"), String::from("2")];
        assert_eq!(
            parse_args(&args),
            Err("Invalid argument: year out of range (valid: 2024 only)")
        );
        let args = [
            String::from("ec"),
            String::from("2024"),
            String::from("day"),
        ];
        assert_eq!(
            parse_args(&args),
            Err("Invalid argument: day must be integer")
        );
        let args = [String::from("ec"), String::from("2024"), String::from("0")];
        assert_eq!(
            parse_args(&args),
            Err("Invalid argument: day out of range (valid: 1-20)")
        );
        let args = [String::from("ec"), String::from("2024"), String::from("21")];
        assert_eq!(
            parse_args(&args),
            Err("Invalid argument: day out of range (valid: 1-20)")
        );
    }

    #[test]
    fn parse_args_valid_arguments() {
        let args = [String::from("ec"), String::from("--version")];
        assert_eq!(parse_args(&args), Err("--version"));
        let args = [String::from("ec"), String::from("--help")];
        assert_eq!(parse_args(&args), Err("--help"));
        let args = [String::from("ec")];
        assert_eq!(parse_args(&args), Ok((None, None)));
        let args = [String::from("ec"), String::from("2024")];
        assert_eq!(parse_args(&args), Ok((Some(2024), None)));
        let args = [String::from("ec"), String::from("2024"), String::from("1")];
        assert_eq!(parse_args(&args), Ok((Some(2024), Some(1))));
    }
}
