//! [ec](https://everybody.codes/event/2024/quests/2)

use crate::ec::{PuzzleInput, PuzzleMetaData, PuzzleResult};
use std::collections::HashSet;

pub fn metadata() -> PuzzleMetaData<'static> {
    PuzzleMetaData {
        year: 2024,
        day: 2,
        title: "The Runes of Power",
        solution: ("29", "5236", "11818"),
        example_solutions: vec![("4", "42", "10")],
    }
}

pub fn solve(input: PuzzleInput, part: usize) -> PuzzleResult {
    // ---------- Parse and Check input
    if input.len() < 3 {
        Err("input must have at least 3 lines")?;
    }
    if !input[1].is_empty() {
        Err("input second line must be empty")?;
    }
    if !input[0].starts_with("WORDS:") {
        Err("input first line must start with `WORDS:`")?;
    }
    let words = input[0]
        .strip_prefix("WORDS:")
        .unwrap()
        .split(',')
        .collect::<Vec<_>>();
    // ---------- Part 1+2+3
    match part {
        1 => solve_part1(words, input[2]),
        2 => solve_part2(words, &input[2..]),
        3 => solve_part3(words, &input[2..]),
        _ => Err("invalid part")?,
    }
}

fn solve_part1(words: Vec<&str>, sentence: &str) -> PuzzleResult {
    let mut ans = 0;
    for word in words {
        ans += sentence
            .as_bytes()
            .windows(word.len())
            .filter(|&w| w == word.as_bytes())
            .count();
    }
    Ok(ans.to_string())
}

fn solve_part2(words: Vec<&str>, sentences: PuzzleInput) -> PuzzleResult {
    let mut symbols = HashSet::new();
    for (row, &sentence) in sentences.iter().enumerate() {
        let s = sentence.as_bytes();
        for &word in words.iter() {
            let rev_word = word.chars().rev().collect::<String>();
            for (start, sub) in s.windows(word.len()).enumerate() {
                if sub == word.as_bytes() || sub == rev_word.as_bytes() {
                    for i in 0..sub.len() {
                        symbols.insert((row, start + i));
                    }
                }
            }
        }
    }
    Ok(symbols.len().to_string())
}

fn solve_part3(words: Vec<&str>, sentences: PuzzleInput) -> PuzzleResult {
    let mut symbols = HashSet::new();
    let max_y = sentences.len() as i32;
    let max_x = sentences[0].len() as i32;
    for &word in words.iter() {
        for start_y in 0..max_y {
            for start_x in 0..max_x {
                for (dx, dy) in [(1, 0), (-1, 0), (0, 1), (0, -1)] {
                    let mut x = start_x;
                    let mut y = start_y;
                    let mut is_ok = true;
                    for &w in word.as_bytes() {
                        if y < 0 || y >= max_y {
                            is_ok = false;
                            break;
                        }
                        let c = sentences[y as usize].as_bytes()[x as usize];
                        if c != w {
                            is_ok = false;
                            break;
                        }
                        x = (x + dx + max_x) % max_x;
                        y += dy;
                    }
                    if is_ok {
                        x = start_x;
                        y = start_y;
                        for _ in 0..word.len() {
                            symbols.insert((x, y));
                            x = (x + dx + max_x) % max_x;
                            y += dy;
                        }
                    }
                }
            }
        }
    }
    Ok(symbols.len().to_string())
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
    fn invalid_must_have_lines() {
        test_invalid_msg(
            &["WORDS:AA,BB", ""],
            solve,
            "input must have at least 3 lines",
        );
    }

    #[test]
    fn invalid_second_line_must_be_empty() {
        test_invalid_msg(
            &["WORDS:AA,BB", "Z", "AABB"],
            solve,
            "input second line must be empty",
        );
    }

    #[test]
    fn invalid_first_line_must_start_with_words() {
        test_invalid_msg(
            &["AA,BB", "", "AABB"],
            solve,
            "input first line must start with `WORDS:`",
        );
    }
}
