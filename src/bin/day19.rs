use std::collections::HashSet;

type Rule<'a> = (&'a str, &'a str);

fn flipped<'a>((rule_from, rule_to): &'a Rule<'a>) -> Rule<'a> {
    (rule_to, rule_from)
}

fn parse_rule(s: &str) -> Result<Rule<'_>, ()> {
    s.split_once(" => ").ok_or(())
}

fn parse_input(s: &str) -> Result<(Vec<Rule<'_>>, &str), ()> {
    let (raw_rules, end_result) = s.split_once("\n\n").ok_or(())?;
    let rules = raw_rules
        .lines()
        .map(parse_rule)
        .collect::<Result<_, _>>()?;
    Ok((rules, end_result.trim()))
}

fn apply_rule<'a>(
    (rule_from, rule_to): Rule<'a>,
    molecule: &'a str,
) -> impl Iterator<Item = String> + 'a {
    molecule.match_indices(rule_from).map(|(idx, _)| {
        let (start, end) = molecule.split_at(idx);
        let (_, end) = end.split_at(rule_from.len());
        let mut result = String::with_capacity(molecule.len() - rule_from.len() + rule_to.len());
        result.push_str(start);
        result.push_str(rule_to);
        result.push_str(end);
        result
    })
}

fn parse(rules: &[Rule<'_>], molecule: &str) -> Option<usize> {
    if molecule == "e" {
        return Some(0);
    }
    for rule in rules {
        for applied in apply_rule(flipped(rule), molecule) {
            if let Some(path_length) = parse(rules, &applied) {
                return Some(1 + path_length);
            }
        }
    }
    None
}

fn main() {
    let (rules, molecule) =
        parse_input(include_str!("../../inputs/day19.txt")).expect("input should parse");
    let all_applications: HashSet<String> = rules
        .iter()
        .flat_map(|rule| apply_rule(*rule, molecule))
        .collect();
    println!("Part 1: {}", all_applications.len());
    println!("Part 2: {}", parse(&rules, molecule).unwrap());
}
