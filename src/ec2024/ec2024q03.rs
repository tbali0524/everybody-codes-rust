//! [ec](https://everybody.codes/event/2024/quests/3)

use crate::ec::{PuzzleInput, PuzzleMetaData, PuzzleResult};

pub fn metadata() -> PuzzleMetaData<'static> {
    PuzzleMetaData {
        year: 2024,
        day: 3,
        title: "Mining Maestro",
        solution: ("124", "2617", "10152"),
        example_solutions: vec![("35", "35", "29")],
    }
}

pub fn solve(input: PuzzleInput, part: usize) -> PuzzleResult {
    // ---------- Part 1+2+3
    let nb_part1 = vec![(1, 0), (0, 1), (-1, 0), (0, -1)];
    let nb_part3 = vec![
        (1, 0),
        (1, 1),
        (0, 1),
        (-1, 1),
        (-1, 0),
        (-1, -1),
        (0, -1),
        (1, -1),
    ];
    match part {
        1 => solve_part(input, &nb_part1),
        2 => solve_part(input, &nb_part1),
        3 => solve_part(input, &nb_part3),
        _ => Err("invalid part")?,
    }
}

fn solve_part(input: PuzzleInput, neighbors: &[(i32, i32)]) -> PuzzleResult {
    let mut ans = 0;
    let max_y = input.len();
    let max_x = if max_y > 0 { input[0].len() } else { 0 };
    let mut depth_grid = vec![vec![0; max_x]; max_y];
    let mut candidates = Vec::new();
    for (y, &row) in input.iter().enumerate() {
        depth_grid.push(Vec::new());
        for (x, c) in row.chars().enumerate() {
            match c {
                '.' => (),
                '#' => {
                    candidates.push((x, y));
                }
                _ => Err("input must contain only .#")?,
            }
        }
    }
    while !candidates.is_empty() {
        let mut next_candidates = Vec::new();
        for (x, y) in candidates.iter() {
            let mut is_ok = true;
            for (dx, dy) in neighbors {
                let x1 = *x as i32 + *dx;
                let y1 = *y as i32 + *dy;
                let nb_depth = if x1 < 0 || x1 >= max_x as i32 || y1 < 0 || y1 >= max_y as i32 {
                    0
                } else {
                    depth_grid[y1 as usize][x1 as usize]
                };
                if nb_depth < depth_grid[*y][*x] {
                    is_ok = false;
                    break;
                }
            }
            if !is_ok {
                continue;
            }
            depth_grid[*y][*x] += 1;
            next_candidates.push((*x, *y));
            ans += 1;
        }
        candidates = next_candidates;
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
    fn invalid_char() {
        test_invalid_msg(&[&"a"], solve, "input must contain only .#");
    }
}
