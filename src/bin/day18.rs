use std::mem;

fn count_neighbours(grid: &[Vec<bool>], row: usize, col: usize) -> usize {
    [
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, -1),
        (0, 1),
        (1, -1),
        (1, 0),
        (1, 1),
    ]
    .into_iter()
    .map(|(drow, dcol)| {
        if let (Ok(crow), Ok(ccol)) = (
            usize::try_from(row as isize + drow),
            usize::try_from(col as isize + dcol),
        ) {
            grid.get(crow)
                .and_then(|r| r.get(ccol))
                .cloned()
                .unwrap_or_default()
        } else {
            false
        }
    })
    .filter(|b| *b)
    .count()
}

fn step(mut grid: Vec<Vec<bool>>, count: usize, fixed_corners: bool) -> Vec<Vec<bool>> {
    if fixed_corners {
        grid[0][0] = true;
        *grid[0].last_mut().unwrap() = true;
        *grid.last_mut().unwrap().first_mut().unwrap() = true;
        *grid.last_mut().unwrap().last_mut().unwrap() = true;
    }
    let mut source_grid = grid.clone();
    for _ in 0..count {
        mem::swap(&mut grid, &mut source_grid);
        for (row_idx, row) in grid.iter_mut().enumerate() {
            for (col_idx, field) in row.iter_mut().enumerate() {
                *field = matches!(
                    (
                        source_grid[row_idx][col_idx],
                        count_neighbours(&source_grid, row_idx, col_idx)
                    ),
                    (true, 2 | 3) | (false, 3)
                );
            }
        }
        if fixed_corners {
            grid[0][0] = true;
            *grid[0].last_mut().unwrap() = true;
            *grid.last_mut().unwrap().first_mut().unwrap() = true;
            *grid.last_mut().unwrap().last_mut().unwrap() = true;
        }
    }
    grid
}

fn count_on(grid: &[Vec<bool>]) -> usize {
    grid.iter()
        .flat_map(|row| row.iter())
        .filter(|b| **b)
        .count()
}

fn main() {
    let grid: Vec<Vec<bool>> = include_str!("../../inputs/day18.txt")
        .lines()
        .map(|line| line.chars().map(|c| c == '#').collect())
        .collect();
    println!("Part 1: {}", count_on(&step(grid.clone(), 100, false)));
    println!("Part 2: {}", count_on(&step(grid, 100, true)));
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = ".#.#.#\n...##.\n#....#\n..#...\n#.#..#\n####..";

    #[test]
    fn test_sample() {
        let mut grid: Vec<Vec<bool>> = SAMPLE
            .lines()
            .map(|line| line.chars().map(|c| c == '#').collect())
            .collect();
        assert_eq!(count_on(&grid), 15);

        grid = step(grid, 4, false);
        assert_eq!(count_on(&grid), 4);
    }

    #[test]
    fn test_sample_part2() {
        let mut grid: Vec<Vec<bool>> = SAMPLE
            .lines()
            .map(|line| line.chars().map(|c| c == '#').collect())
            .collect();
        assert_eq!(count_on(&grid), 15);

        grid = step(grid, 5, true);
        assert_eq!(count_on(&grid), 17);
    }
}
