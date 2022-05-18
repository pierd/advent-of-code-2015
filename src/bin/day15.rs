const INGREDIENTS: [([isize; 4], isize); 4] = [
    ([5, -1, 0, 0], 5),
    ([-1, 3, 0, 0], 1),
    ([0, -1, 4, 0], 6),
    ([-1, 0, 0, 2], 8),
];

fn solve(calories: Option<isize>) -> usize {
    let mut best = 0;
    for i in 0isize..=100 {
        for j in 0..=(100 - i) {
            for k in 0..=(100 - j) {
                let l = 100 - i - j - k;
                if let Some(cal) = calories {
                    let cal_value: isize = [i, j, k, l]
                        .iter()
                        .zip(INGREDIENTS.iter())
                        .map(|(frac, (_props, calories))| *frac * *calories)
                        .sum();
                    if cal != cal_value {
                        continue;
                    }
                }
                let mut properties = Vec::with_capacity(4);
                for p in 0..4 {
                    let prop_value: isize = [i, j, k, l]
                        .iter()
                        .zip(INGREDIENTS.iter())
                        .map(|(frac, (props, _calories))| *frac * props[p])
                        .sum();
                    properties.push(if prop_value < 0 {
                        0
                    } else {
                        prop_value as usize
                    });
                }
                let score = properties.into_iter().product();
                if best < score {
                    best = score;
                }
            }
        }
    }
    best
}

fn main() {
    println!("Part 1: {}", solve(None));
    println!("Part 2: {}", solve(Some(500)));
}
