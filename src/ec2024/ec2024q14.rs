//! [ec](https://everybody.codes/event/2024/quests/14)
//!
//! @todo: solve

use crate::ec::{PuzzleInput, PuzzleMetaData, PuzzleResult};

pub fn metadata() -> PuzzleMetaData<'static> {
    PuzzleMetaData {
        year: 2024,
        day: 14,
        title: "",
        solution: ("0", "0", "0"),
        example_solutions: vec![("0", "0", "0")],
    }
}

pub fn solve(input: PuzzleInput, part: usize) -> PuzzleResult {
    // ---------- Check and Parse input
    // ---------- Part 1+2+3
    match part {
        1 => solve_part1(input),
        2 => solve_part2(input),
        3 => solve_part3(input),
        _ => Err("invalid part")?,
    }
}

fn solve_part1(_input: PuzzleInput) -> PuzzleResult {
    let ans = 0;
    Ok(ans.to_string())
}

fn solve_part2(_input: PuzzleInput) -> PuzzleResult {
    let ans = 0;
    Ok(ans.to_string())
}

fn solve_part3(_input: PuzzleInput) -> PuzzleResult {
    let ans = 0;
    Ok(ans.to_string())
}

// ------------------------------------------------------------
#[cfg(test)]
mod tests {
    use super::*;
    use crate::ec::runner::tests::*;

    #[test]
    #[ignore]
    fn example1() {
        test_case(metadata, solve, 1);
    }

    #[test]
    #[ignore]
    fn puzzle() {
        test_case(metadata, solve, 0);
    }

    #[test]
    #[ignore]
    fn invalid() {
        test_invalid_msg(&[&"a"], solve, "...");
    }
}
