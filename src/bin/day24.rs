fn find_sum<F: FnMut(&[usize])>(nums: &[usize], target: usize, mut result_callback: F) {
    let mut result = Vec::new();
    find_sum_internal(nums, target, &mut result, &mut result_callback);
}

fn find_sum_internal<F: FnMut(&[usize])>(
    nums: &[usize],
    target: usize,
    so_far: &mut Vec<usize>,
    result_callback: &mut F,
) {
    if target == 0 {
        result_callback(so_far);
        return;
    }
    if let Some((first, rest)) = nums.split_first() {
        if target >= *first {
            so_far.push(*first);
            find_sum_internal(rest, target - *first, so_far, result_callback);
            so_far.pop();
        }
        find_sum_internal(rest, target, so_far, result_callback);
    }
}

fn find_best_split(weights: &[usize], splits: usize) -> usize {
    let mut best_length = weights.len();
    let mut best_qe = usize::MAX;
    find_sum(
        weights,
        weights.iter().cloned().sum::<usize>() / splits,
        |nums| {
            if nums.len() < best_length {
                best_length = nums.len();
                best_qe = nums.iter().cloned().product();
            }
            if nums.len() == best_length {
                let qe = nums.iter().cloned().product();
                if qe < best_qe {
                    best_qe = qe;
                }
            }
        },
    );
    best_qe
}

fn main() {
    let mut weights = include_str!("../../inputs/day24.txt")
        .lines()
        .map(|line| line.parse::<usize>())
        .collect::<Result<Vec<_>, _>>()
        .expect("input should parse");
    weights.sort_unstable();
    weights.reverse();

    println!("Part 1: {}", find_best_split(&weights, 3));
    println!("Part 2: {}", find_best_split(&weights, 4));
}
