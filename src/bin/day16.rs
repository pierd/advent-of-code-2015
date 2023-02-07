use std::collections::HashMap;

struct Matcher<'a> {
    props: HashMap<&'a str, usize>,
}

impl<'a> Matcher<'a> {
    fn new(props: HashMap<&'a str, usize>) -> Self {
        Matcher { props }
    }

    fn matches(&self, other: &HashMap<&str, usize>) -> bool {
        for (k, v) in other {
            if self.props.get(k).expect("matcher should have all fields") != v {
                return false;
            }
        }
        true
    }

    fn complex_comparison_direction(key: &str) -> std::cmp::Ordering {
        match key {
            "cats" | "trees" => std::cmp::Ordering::Greater,
            "pomeranians" | "goldfish" => std::cmp::Ordering::Less,
            _ => std::cmp::Ordering::Equal,
        }
    }

    fn matches_complex(&self, other: &HashMap<&str, usize>) -> bool {
        for (k, v) in other {
            if v.cmp(self.props.get(k).expect("matcher should have all fields"))
                != Self::complex_comparison_direction(k)
            {
                return false;
            }
        }
        true
    }
}

fn parse(s: &str) -> Result<(usize, HashMap<&str, usize>), ()> {
    // 0   1 2 3      4 5  7       8 9  11  12
    //                   6            10      13
    // Sue 11: vizslas: 5, perfumes: 8, cars: 10
    let mut parts = s.split(&[' ', ':', ',']);

    // skip "Sue"
    parts.next();

    // parse index
    let idx = parts.next().ok_or(())?.parse::<usize>().map_err(|_| ())?;

    // skip separator
    parts.next();

    // parse components
    let mut components: HashMap<&str, usize> = Default::default();
    while let Some(name) = parts.next() {
        parts.next();
        let count = parts.next().ok_or(())?.parse::<usize>().map_err(|_| ())?;
        components.insert(name, count);
        parts.next();
    }
    Ok((idx, components))
}

fn main() {
    let matcher = Matcher::new(
        [
            ("children", 3),
            ("cats", 7),
            ("samoyeds", 2),
            ("pomeranians", 3),
            ("akitas", 0),
            ("vizslas", 0),
            ("goldfish", 5),
            ("trees", 3),
            ("cars", 2),
            ("perfumes", 1),
        ]
        .into_iter()
        .collect(),
    );
    let sues: Vec<_> = include_str!("../../inputs/day16.txt")
        .lines()
        .map(parse)
        .collect::<Result<_, _>>()
        .expect("input should parse");
    for (sue_idx, sue_props) in &sues {
        if matcher.matches(sue_props) {
            println!("Part 1: {sue_idx}");
        }
    }
    for (sue_idx, sue_props) in &sues {
        if matcher.matches_complex(sue_props) {
            println!("Part 2: {sue_idx}");
        }
    }
}
