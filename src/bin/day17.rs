use std::collections::HashMap;

fn ways_to_store(containers: &[usize], volume: usize) -> usize {
    let mut ways = vec![0; volume + 1];
    ways[0] = 1;
    for c in containers {
        for i in (1..=volume).rev() {
            if i >= *c {
                ways[i] += ways[i - c];
            }
        }
    }
    ways[volume]
}

fn ways_to_store_min(containers: &[usize], volume: usize) -> usize {
    let mut ways: Vec<HashMap<usize, usize>> = vec![Default::default(); volume + 1];
    ways[0].insert(0, 1);
    for c in containers {
        for i in (1..=volume).rev() {
            if i >= *c {
                let prev_ways = ways[i - c].clone();
                for (containers_count, ways_count) in prev_ways {
                    *ways[i].entry(containers_count + 1).or_default() += ways_count;
                }
            }
        }
    }
    *ways[volume].iter().min().expect("there should be ways").1
}

fn main() {
    let containers = include_str!("../../inputs/day17.txt")
        .lines()
        .map(|s| s.parse::<usize>())
        .collect::<Result<Vec<usize>, _>>()
        .expect("input should parse");
    println!("Part 1: {}", ways_to_store(&containers, 150));
    println!("Part 2: {}", ways_to_store_min(&containers, 150));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample() {
        assert_eq!(ways_to_store(&[20, 15, 10, 5, 5], 25), 4);
        assert_eq!(ways_to_store_min(&[20, 15, 10, 5, 5], 25), 3);
    }
}
