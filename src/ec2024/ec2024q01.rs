//! [ec](https://everybody.codes/event/2024/quests/1)

use crate::ec::{PuzzleInput, PuzzleMetaData, PuzzleResult};

pub fn metadata() -> PuzzleMetaData<'static> {
    PuzzleMetaData {
        year: 2024,
        day: 1,
        title: "The Battle for the Farmlands",
        solution: ("1404", "5237", "28225"),
        example_solutions: vec![("5", "28", "30")],
    }
}

pub fn solve(input: PuzzleInput, part: usize) -> PuzzleResult {
    // ---------- Check input
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
    let mut ans = 0;
    for c in input[0].chars() {
        ans += match c {
            'A' => 0,
            'B' => 1,
            'C' => 3,
            _ => Err("input must contain only ABC")?,
        };
    }
    Ok(ans.to_string())
}

fn solve_part2(input: PuzzleInput) -> PuzzleResult {
    let mut ans = 0;
    for c in input[0].chars() {
        ans += match c {
            'A' => 0,
            'B' => 1,
            'C' => 3,
            'D' => 5,
            'x' => 0,
            _ => Err("input must contain only ABCDx")?,
        };
    }
    let v = input[0].chars().collect::<Vec<_>>();
    for (i, &c) in v.iter().enumerate() {
        let pair = if i % 2 == 0 { i + 1 } else { i - 1 };
        if c != 'x' && v[pair] != 'x' {
            ans += 1;
        }
    }
    Ok(ans.to_string())
}

fn solve_part3(input: PuzzleInput) -> PuzzleResult {
    let mut ans = 0;
    for c in input[0].chars() {
        ans += match c {
            'A' => 0,
            'B' => 1,
            'C' => 3,
            'D' => 5,
            'x' => 0,
            _ => Err("input must contain only ABCDx")?,
        };
    }
    let v = input[0].chars().collect::<Vec<_>>();
    for (i, &c) in v.iter().enumerate() {
        if c == 'x' {
            continue;
        }
        let mut other1 = 0;
        let mut other2 = 0;
        match i % 3 {
            0 => {
                other1 = i + 1;
                other2 = i + 2;
            }
            1 => {
                other1 = i - 1;
                other2 = i + 1;
            }
            2 => {
                other1 = i - 2;
                other2 = i - 1;
            }
            _ => (),
        }
        if v[other1] != 'x' {
            ans += 1;
        }
        if v[other2] != 'x' {
            ans += 1;
        }
    }
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
    fn invalid_single_line() {
        test_invalid_msg(&["a", "b"], solve, "input must have a single line");
    }

    #[test]
    fn invalid() {
        test_invalid_msg(&["Z"], solve, "input must contain only ABC");
    }
}
