//! Empty solution template
//!
//! [ec](https://everybody.codes/event/YYYY/quests/DD)

use crate::ec::{PuzzleInput, PuzzleMetaData, PuzzleResult};

pub fn metadata() -> PuzzleMetaData<'static> {
    PuzzleMetaData {
        year: 2024,
        day: 0,
        title: "",
        solution: ("0", "0", "0"),
        example_solutions: vec![("0", "0", "0")],
    }
}

pub fn solve(input: PuzzleInput) -> PuzzleResult {
    // ---------- Parse and Check input
    if input.len() != 1 {
        Err("input must have a single line")?;
    }
    // ---------- Part 1+2+3
    match part {
        1 => solve_part1(input),
        2 => solve_part2(input),
        3 => solve_part3(input),
        _ => Err("invalid part")?,
    }
}

fn solve_part1(input: PuzzleInput) -> PuzzleResult {
    let ans = 0;
    Ok(ans.to_string())
}

fn solve_part2(input: PuzzleInput) -> PuzzleResult {
    let ans = 0;
    Ok(ans.to_string())
}

fn solve_part3(input: PuzzleInput) -> PuzzleResult {
    let ans = 0;
    Ok(ans.to_string())
}

// ------------------------------------------------------------
#[cfg(test)]
mod tests {
    use super::*;
    use crate::ec::runner::tests::*;

    #[test]
    fn example1() {
        test_case(metadata, solve, 1);
    }

    #[test]
    fn puzzle() {
        test_case(metadata, solve, 0);
    }

    #[test]
    #[ignore]
    fn invalid_single_line() {
        test_invalid_msg(&[&"a", &"b"], solve, "input must have a single line");
    }

    #[test]
    #[ignore]
    fn invalid() {
        test_invalid_msg(&[&"a"], solve, "...");
    }
}
