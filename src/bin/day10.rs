use std::mem;

const INPUT: &str = "1321131112";

struct Uniq<I, T> {
    iter: I,
    next: Option<T>,
}

impl<I, T> Iterator for Uniq<I, T>
where
    I: Iterator<Item = T>,
    T: Eq,
{
    type Item = (usize, T);

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(current) = self.next.as_ref() {
            let mut count = 1;
            let mut new_next = self.iter.next();
            while new_next
                .as_ref()
                .map(|new_next| current == new_next)
                .unwrap_or_default()
            {
                count += 1;
                new_next = self.iter.next();
            }
            mem::swap(&mut self.next, &mut new_next);
            Some((count, new_next.expect("")))
        } else {
            None
        }
    }
}

trait AddUniq<I, T> {
    fn uniq(self) -> Uniq<I, T>;
}

impl<I, T> AddUniq<I, T> for I
where
    I: Iterator<Item = T>,
{
    fn uniq(mut self) -> Uniq<I, T> {
        let next = self.next();
        Uniq { iter: self, next }
    }
}

fn to_digits(s: &str) -> Result<Vec<usize>, ()> {
    s.chars()
        .map(|c| c.to_digit(10).map(|d| d as usize).ok_or(()))
        .collect()
}

fn look_and_say(digits: &[usize]) -> Vec<usize> {
    let mut result = Vec::new();
    for (count, digit) in digits.iter().cloned().uniq() {
        result.push(count);
        result.push(digit);
    }
    result
}

fn main() {
    let mut seq = to_digits(INPUT).expect("input should be just digits");
    for _ in 0..40 {
        seq = look_and_say(&seq);
    }
    println!("Part 1: {}", seq.len());
    for _ in 0..10 {
        seq = look_and_say(&seq);
    }
    println!("Part 2: {}", seq.len());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_uniq_empty() {
        let v: Vec<usize> = Vec::new();
        let mut iter = v.into_iter().uniq();
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn test_uniq() {
        let v = vec![1, 1, 2, 3, 3, 3, 1, 1];
        let mut iter = v.into_iter().uniq();
        assert_eq!(iter.next(), Some((2, 1)));
        assert_eq!(iter.next(), Some((1, 2)));
        assert_eq!(iter.next(), Some((3, 3)));
        assert_eq!(iter.next(), Some((2, 1)));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn test_look_and_say() {
        assert_eq!(look_and_say(&[1]), vec![1, 1]);
        assert_eq!(look_and_say(&[1, 1]), vec![2, 1]);
        assert_eq!(look_and_say(&[2, 1]), vec![1, 2, 1, 1]);
        assert_eq!(look_and_say(&[1, 2, 1, 1]), vec![1, 1, 1, 2, 2, 1]);
        assert_eq!(look_and_say(&[1, 1, 1, 2, 2, 1]), vec![3, 1, 2, 2, 1, 1]);
    }
}
