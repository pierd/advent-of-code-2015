const INPUT: &str = "vzbxkghb";

fn has_straight(s: &[u8]) -> bool {
    for win in s.windows(3) {
        if win[0] == win[1] + 1 && win[1] == win[2] + 1 {
            return true;
        }
    }
    false
}

fn valid_chars(s: &[u8]) -> bool {
    !s.contains(&b'i') && !s.contains(&b'o') && !s.contains(&b'l')
}

fn has_2_pairs(s: &[u8]) -> bool {
    let mut first_pair = None;
    for pair in s.windows(2) {
        if pair[0] == pair[1] {
            if let Some(x) = first_pair {
                if x != pair[0] {
                    return true;
                }
            } else {
                first_pair = Some(pair[0]);
            }
        }
    }
    false
}

fn next_pass(p: &mut [u8]) {
    for c in p.iter_mut() {
        *c += 1;
        if *c > b'z' {
            *c = b'a';
        } else {
            return;
        }
    }
}

fn next_valid_pass(p: &mut [u8]) {
    next_pass(p);
    while !has_straight(p) || !valid_chars(p) || !has_2_pairs(p) {
        next_pass(p);
    }
}

fn str2pass(s: &str) -> Option<Vec<u8>> {
    s.chars().rev().map(|c| u8::try_from(c).ok()).collect()
}

fn pass2str(p: &[u8]) -> String {
    p.iter().rev().map(|d| char::from(*d)).collect()
}

fn find_next_valid_pass(s: &str) -> String {
    let mut raw_pass = str2pass(s).expect("must be a-z");
    next_valid_pass(&mut raw_pass);
    pass2str(&raw_pass)
}

fn main() {
    let first = find_next_valid_pass(INPUT);
    println!("Part 1: {}", &first);
    let second = find_next_valid_pass(&first);
    println!("Part 2: {}", &second);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample_validation_failure() {
        assert!(has_straight(&str2pass("abcdffaa").unwrap()));
    }

    #[test]
    fn test_sample() {
        assert_eq!(find_next_valid_pass("abcdefgh"), "abcdffaa");
        assert_eq!(find_next_valid_pass("ghijklmn"), "ghjaabcc");
    }
}
