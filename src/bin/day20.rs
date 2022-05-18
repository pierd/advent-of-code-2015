use std::collections::HashMap;

const INPUT: usize = 33100000;

fn divisors_sum(n: usize) -> usize {
    let mut sum = 0;
    let mut i = 1;
    while i * i <= n {
        if n % i == 0 {
            sum += i;
            if n / i != i {
                sum += n / i;
            }
        }
        i += 1;
    }
    sum
}

fn part2(target: usize) -> Option<usize> {
    let mut divisors_used: HashMap<usize, usize> = Default::default();
    let mut use_divisor = |divisor: usize| {
        let times = divisors_used.entry(divisor).or_default();
        if *times >= 50 {
            0
        } else {
            *times += 1;
            divisor
        }
    };

    for n in 1.. {
        let mut presents = 0;
        let mut i = 1;
        while i * i <= n {
            if n % i == 0 {
                presents += use_divisor(i) * 11;
                if n / i != i {
                    presents += use_divisor(n / i) * 11;
                }
            }
            i += 1;
        }
        if presents >= target {
            return Some(n);
        }
    }
    None
}

fn main() {
    let smallest = (10..)
        .map(|n| (n, divisors_sum(n) * 10))
        .find(|(_, s)| *s >= INPUT)
        .unwrap()
        .0;
    println!("Part 1: {}", smallest);
    println!("Part 2: {}", part2(INPUT).unwrap());
}
