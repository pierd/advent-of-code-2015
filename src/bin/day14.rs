use std::str::FromStr;

#[derive(Clone, Copy, Debug)]
struct Reindeer {
    speed: usize,
    run_time: usize,
    rest_time: usize,
}

impl FromStr for Reindeer {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // 0     1   2   3  4    5   6 7        8   9    10   11   12  13  14
        // Vixen can fly 19 km/s for 7 seconds, but then must rest for 124 seconds.
        let parts = s.split_whitespace().collect::<Vec<_>>();
        let parse_usize = |idx: usize| -> Result<usize, ()> {
            parts.get(idx).ok_or(())?.parse::<usize>().map_err(|_| ())
        };
        let speed = parse_usize(3)?;
        let run_time = parse_usize(6)?;
        let rest_time = parse_usize(13)?;
        Ok(Self {
            speed,
            run_time,
            rest_time,
        })
    }
}

impl Reindeer {
    fn distance_after(&self, time: usize) -> usize {
        let runs = time / (self.run_time + self.rest_time);
        let seconds_left = time % (self.run_time + self.rest_time);
        (runs * self.run_time + [seconds_left, self.run_time].into_iter().min().unwrap())
            * self.speed
    }
}

fn calculate_points(reindeers: &[Reindeer], time: usize) -> Vec<usize> {
    let mut points = vec![0; reindeers.len()];
    for t in 1..=time {
        let distances = reindeers
            .iter()
            .map(|r| r.distance_after(t))
            .collect::<Vec<_>>();
        let best_distance = distances.iter().cloned().max().unwrap();
        for (p, dist) in points.iter_mut().zip(distances.into_iter()) {
            if dist == best_distance {
                *p += 1;
            }
        }
    }
    points
}

fn main() {
    let reindeers = include_str!("../../inputs/day14.txt")
        .lines()
        .map(Reindeer::from_str)
        .collect::<Result<Vec<Reindeer>, ()>>()
        .expect("input should parse");
    println!(
        "Part 1: {}",
        reindeers
            .iter()
            .map(|r| r.distance_after(2503))
            .max()
            .expect("there should be at least one")
    );
    println!(
        "Part 2: {}",
        calculate_points(&reindeers, 2503)
            .into_iter()
            .max()
            .expect("there should be at least one")
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample() {
        let comet = "Comet can fly 14 km/s for 10 seconds, but then must rest for 127 seconds."
            .parse::<Reindeer>()
            .unwrap();
        assert_eq!(comet.distance_after(1000), 1120);
    }
}
