use std::collections::HashMap;

struct Distances<'a> {
    cities: Vec<&'a str>,
    distances: HashMap<(&'a str, &'a str), usize>,
}

impl<'a> Distances<'a> {
    fn from_str(s: &'a str) -> Result<Self, ()> {
        let mut cities = Vec::new();
        let mut distances = HashMap::new();
        for line in s.lines() {
            let (cities_pair, distance_str) = line.split_once(" = ").ok_or(())?;
            let distance = distance_str.parse::<usize>().map_err(|_| ())?;
            let (city1, city2) = cities_pair.split_once(" to ").ok_or(())?;
            if !cities.contains(&city1) {
                cities.push(city1);
            }
            if !cities.contains(&city2) {
                cities.push(city2);
            }
            distances.insert((city1, city2), distance);
            distances.insert((city2, city1), distance);
        }
        Ok(Self { cities, distances })
    }

    fn path_length(&self, path: &[&'a str]) -> usize {
        path.windows(2)
            .map(|pair| {
                self.distances
                    .get(&(pair[0], pair[1]))
                    .expect("all distances should be specified")
            })
            .sum()
    }

    fn find_shortest_and_longest_path(&self) -> (usize, usize) {
        let all_cities = self.cities.clone();
        let mut best_length = self.path_length(&all_cities);
        let mut worst_length = best_length;

        permutations(all_cities, |candidate| {
            let new_length = self.path_length(candidate);
            if new_length < best_length {
                best_length = new_length;
            } else if worst_length < new_length {
                worst_length = new_length;
            }
        });

        (best_length, worst_length)
    }
}

fn permutations<T, F>(elements: Vec<T>, callback: F) -> Vec<T>
where
    F: FnMut(&Vec<T>),
{
    gen_permutations(elements.len(), elements, callback).0
}

fn gen_permutations<T, F>(k: usize, mut elements: Vec<T>, mut callback: F) -> (Vec<T>, F)
where
    F: FnMut(&Vec<T>),
{
    if k == 1 {
        callback(&elements);
    } else {
        (elements, callback) = gen_permutations(k - 1, elements, callback);
        for i in 0..k - 1 {
            elements.swap(if k % 2 == 0 { i } else { 0 }, k - 1);
            (elements, callback) = gen_permutations(k - 1, elements, callback);
        }
    }
    (elements, callback)
}

fn main() {
    let distances = Distances::<'_>::from_str(include_str!("../../inputs/day09.txt"))
        .expect("input should parse correctly");
    let (shortest, longest) = distances.find_shortest_and_longest_path();
    println!("Part 1: {}", shortest);
    println!("Part 1: {}", longest);
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = "London to Dublin = 464\nLondon to Belfast = 518\nDublin to Belfast = 141";

    #[test]
    fn test_sample_path_length() {
        let distances = Distances::<'_>::from_str(SAMPLE).unwrap();
        assert_eq!(distances.path_length(&["London", "Dublin", "Belfast"]), 605);
        assert_eq!(distances.path_length(&["London", "Belfast", "Dublin"]), 659);
    }

    #[test]
    fn test_sample() {
        let distances = Distances::<'_>::from_str(SAMPLE).unwrap();
        assert_eq!(distances.find_shortest_and_longest_path(), (605, 982));
    }

    #[test]
    fn test_permutations() {
        let mut perms = Vec::new();
        permutations(vec![0, 1, 2], |x| perms.push(x.clone()));
        assert_eq!(perms.len(), 6);
        assert_eq!(
            perms,
            vec![
                vec![0, 1, 2],
                vec![1, 0, 2],
                vec![2, 0, 1],
                vec![0, 2, 1],
                vec![1, 2, 0],
                vec![2, 1, 0],
            ]
        );
    }
}
