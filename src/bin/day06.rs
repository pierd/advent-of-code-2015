use std::ops::RangeInclusive;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Action {
    Toggle,
    On,
    Off,
}

impl Action {
    fn parse_and_chop(s: &str) -> (Self, &str) {
        if let Some(rest) = s.strip_prefix("toggle ") {
            (Self::Toggle, rest)
        } else if let Some(rest) = s.strip_prefix("turn on ") {
            (Self::On, rest)
        } else if let Some(rest) = s.strip_prefix("turn off ") {
            (Self::Off, rest)
        } else {
            unreachable!()
        }
    }

    fn apply(&self, bit: &mut bool) {
        *bit = match self {
            Self::Toggle => !*bit,
            Self::On => true,
            Self::Off => false,
        };
    }

    fn apply_brightness(&self, b: &mut usize) {
        match self {
            Self::Toggle => *b += 2,
            Self::On => *b += 1,
            Self::Off => {
                if *b > 0 {
                    *b -= 1
                }
            }
        }
    }
}

fn parse_pair(s: &str) -> (usize, usize) {
    let mut nums = s.split(",");
    (
        nums.next().unwrap().parse::<usize>().unwrap(),
        nums.next().unwrap().parse::<usize>().unwrap(),
    )
}

fn parse(input: &str) -> Vec<(Action, RangeInclusive<usize>, RangeInclusive<usize>)> {
    let mut actions = Vec::new();
    for line in input.lines() {
        let (action, raw_range) = Action::parse_and_chop(line);
        let mut ranges = raw_range.split(" through ");
        let (row_start, col_start) = parse_pair(ranges.next().unwrap());
        let (row_end, col_end) = parse_pair(ranges.next().unwrap());
        actions.push((action, row_start..=row_end, col_start..=col_end));
    }
    actions
}

fn apply_all(
    m: &mut [[bool; 1000]; 1000],
    actions: &Vec<(Action, RangeInclusive<usize>, RangeInclusive<usize>)>,
) {
    for (action, row_range, col_range) in actions {
        for row in row_range.clone() {
            for col in col_range.clone() {
                action.apply(&mut m[row][col]);
            }
        }
    }
}

fn count_on(m: &[[bool; 1000]; 1000]) -> usize {
    m.iter()
        .flat_map(|row| row.iter())
        .map(|b| if *b { 1 } else { 0 })
        .sum()
}

fn apply_all_brightness(
    m: &mut [[usize; 1000]; 1000],
    actions: &Vec<(Action, RangeInclusive<usize>, RangeInclusive<usize>)>,
) {
    for (action, row_range, col_range) in actions {
        for row in row_range.clone() {
            for col in col_range.clone() {
                action.apply_brightness(&mut m[row][col]);
            }
        }
    }
}

fn sum_brightness(m: &[[usize; 1000]; 1000]) -> usize {
    m.iter().flat_map(|row| row.iter()).map(|n| *n).sum()
}

fn solve(actions: &Vec<(Action, RangeInclusive<usize>, RangeInclusive<usize>)>) -> usize {
    let mut m: [[bool; 1000]; 1000] = [[false; 1000]; 1000];
    apply_all(&mut m, actions);
    count_on(&m)
}

fn solve_brightness(
    actions: &Vec<(Action, RangeInclusive<usize>, RangeInclusive<usize>)>,
) -> usize {
    let mut m: [[usize; 1000]; 1000] = [[0; 1000]; 1000];
    apply_all_brightness(&mut m, actions);
    sum_brightness(&m)
}

fn main() {
    let actions = parse(include_str!("../../inputs/day06.txt"));
    println!("Part 1: {}", solve(&actions));
    println!("Part 2: {}", solve_brightness(&actions));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        assert_eq!(
            parse("turn on 0,0 through 999,999"),
            vec![(Action::On, 0..=999, 0..=999),]
        );
        assert_eq!(
            parse("toggle 0,0 through 999,0"),
            vec![(Action::Toggle, 0..=999, 0..=0),]
        );
        assert_eq!(
            parse("turn off 499,499 through 500,500"),
            vec![(Action::Off, 499..=500, 499..=500),]
        );
    }
}
