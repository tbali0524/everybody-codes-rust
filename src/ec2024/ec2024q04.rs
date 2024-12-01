//! [ec](https://everybody.codes/event/2024/quests/4)

use crate::ec::{PuzzleInput, PuzzleMetaData, PuzzleResult};

pub fn metadata() -> PuzzleMetaData<'static> {
    PuzzleMetaData {
        year: 2024,
        day: 4,
        title: "Royal Smith's Puzzle",
        solution: ("88", "987977", "128994457"),
        example_solutions: vec![("10", "10", "8")],
    }
}

type ItemType = i32;

pub fn solve(input: PuzzleInput, part: usize) -> PuzzleResult {
    // ---------- Check and Parse input
    let data = input
        .iter()
        .map(|&line| {
            line.parse::<ItemType>()
                .map_err(|_| format!("input must contain only integers, found `{}`", line))
        })
        .collect::<Result<Vec<_>, _>>()?;
    // ---------- Part 1+2+3
    match part {
        1 => solve_part1(&data),
        2 => solve_part1(&data),
        3 => solve_part3(&data),
        _ => Err("invalid part")?,
    }
}

fn solve_part1(data: &[ItemType]) -> PuzzleResult {
    let min = data.iter().min().unwrap();
    let ans = data.iter().map(|&x| (x - min).abs()).sum::<ItemType>();
    Ok(ans.to_string())
}

fn solve_part3(data: &[ItemType]) -> PuzzleResult {
    let mut sorted = data.to_vec();
    sorted.sort();
    let median = if sorted.len() % 2 == 1 {
        sorted[sorted.len() / 2]
    } else {
        (sorted[sorted.len() / 2 - 1] + sorted[sorted.len() / 2]) / 2
    };
    let ans = data.iter().map(|&x| (x - median).abs()).sum::<ItemType>();
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
    fn invalid_only_contains_int() {
        test_invalid_msg(&[&"a"], solve, "input must contain only integers, found ");
    }
}
