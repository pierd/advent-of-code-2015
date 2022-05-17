use std::collections::HashMap;

struct Happiness<'a> {
    persons: Vec<&'a str>,
    happiness: HashMap<(&'a str, &'a str), isize>,
}

impl<'a> Happiness<'a> {
    const SELF: &'a str = "";

    fn parse(s: &'a str) -> Result<Happiness<'a>, ()> {
        // 0     1     2    3  4         5     6  7       8    9  10
        // Alice would gain 54 happiness units by sitting next to Bob.
        // Alice would lose 79 happiness units by sitting next to Carol.
        let mut happiness = HashMap::new();
        let mut persons = Vec::new();
        let mut add_person = |person: &'a str| {
            if !persons.contains(&person) {
                persons.push(person);
            }
        };

        for line in s.lines() {
            let parts = line.split_whitespace().collect::<Vec<_>>();
            if parts.len() < 11 {
                return Err(());
            }
            let person1 = parts[0];
            let person2 = parts[10].strip_suffix('.').ok_or(())?;
            add_person(person1);
            add_person(person2);

            let delta_sign = match parts[2] {
                "gain" => 1,
                "lose" => -1,
                _ => return Err(()),
            };
            let delta = parts[3].parse::<isize>().map_err(|_| ())?;
            happiness.insert((person1, person2), delta_sign * delta);
        }

        Ok(Happiness { persons, happiness })
    }

    fn get_happiness(&self, person1: &str, person2: &str) -> isize {
        self.happiness
            .get(&(person1, person2))
            .cloned()
            .unwrap_or_default()
            + self
                .happiness
                .get(&(person2, person1))
                .cloned()
                .unwrap_or_default()
    }

    fn calculate_happiness(&self, order: &[&str]) -> isize {
        let wrapped = self.get_happiness(order.last().unwrap(), order.first().unwrap());
        let in_order: isize = order
            .windows(2)
            .map(|ppl| self.get_happiness(ppl[0], ppl[1]))
            .sum();
        wrapped + in_order
    }

    fn find_best_happiness(&self) -> isize {
        let mut best = isize::MIN;
        permutations(self.persons.clone(), |perm| {
            let candidate = self.calculate_happiness(perm);
            if best < candidate {
                best = candidate;
            }
        });
        best
    }

    fn add_self(&mut self) {
        for person in &self.persons {
            self.happiness.insert((Self::SELF, person), 0);
            self.happiness.insert((person, Self::SELF), 0);
        }
        self.persons.push(Self::SELF);
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
    let mut happiness =
        Happiness::parse(include_str!("../../inputs/day13.txt")).expect("input should parse");
    println!("Part 1: {}", happiness.find_best_happiness());
    happiness.add_self();
    println!("Part 2: {}", happiness.find_best_happiness());
}
