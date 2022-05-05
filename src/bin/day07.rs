use std::{collections::HashMap, str::FromStr};

#[derive(Clone, Debug, PartialEq, Eq)]
enum Operand {
    Val(u16),
    Wire(String),
}

impl Operand {
    fn eval(&self, wires: &HashMap<String, u16>) -> Option<u16> {
        match self {
            Self::Val(n) => Some(*n),
            Self::Wire(wire) => wires.get(wire).cloned(),
        }
    }

    fn eval2(oper1: &Self, oper2: &Self, wires: &HashMap<String, u16>) -> Option<(u16, u16)> {
        if let (Some(a), Some(b)) = (oper1.eval(wires), oper2.eval(wires)) {
            Some((a, b))
        } else {
            None
        }
    }
}

impl FromStr for Operand {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.parse::<u16>() {
            Ok(n) => Ok(Self::Val(n)),
            Err(_) => Ok(Self::Wire(s.to_owned())),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
enum Instr {
    Just(Operand),
    Not(Operand),
    And(Operand, Operand),
    Or(Operand, Operand),
    Lshift(Operand, Operand),
    Rshift(Operand, Operand),
}

impl Instr {
    fn eval(&self, wires: &HashMap<String, u16>) -> Option<u16> {
        match self {
            Self::Just(operand) => operand.eval(wires),
            Self::Not(operand) => operand.eval(wires).map(|v| !v),
            Self::And(oper1, oper2) => Operand::eval2(oper1, oper2, wires).map(|(a, b)| a & b),
            Self::Or(oper1, oper2) => Operand::eval2(oper1, oper2, wires).map(|(a, b)| a | b),
            Self::Lshift(oper1, oper2) => Operand::eval2(oper1, oper2, wires).map(|(a, b)| a << b),
            Self::Rshift(oper1, oper2) => Operand::eval2(oper1, oper2, wires).map(|(a, b)| a >> b),
        }
    }
}

impl FromStr for Instr {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<_> = s.split_whitespace().collect();
        match parts.len() {
            1 => Ok(Self::Just(parts[0].parse::<Operand>()?)),
            2 => {
                if parts[0] == "NOT" {
                    Ok(Self::Not(parts[1].parse::<Operand>()?))
                } else {
                    Err(())
                }
            }
            3 => {
                let first = parts[0].parse::<Operand>()?;
                let second = parts[2].parse::<Operand>()?;
                match parts[1] {
                    "AND" => Ok(Self::And(first, second)),
                    "OR" => Ok(Self::Or(first, second)),
                    "LSHIFT" => Ok(Self::Lshift(first, second)),
                    "RSHIFT" => Ok(Self::Rshift(first, second)),
                    _ => Err(()),
                }
            }
            _ => Err(()),
        }
    }
}

fn parse_input(input: &str) -> Result<Vec<(Instr, String)>, ()> {
    input
        .lines()
        .map(|line| {
            let parts: Vec<_> = line.split(" -> ").collect();
            if parts.len() != 2 {
                return Err(());
            }
            let instr = parts[0].parse::<Instr>()?;
            let output = parts[1].to_owned();
            Ok((instr, output))
        })
        .collect()
}

fn calculate(instrs: &[(Instr, String)], target: &str) -> HashMap<String, u16> {
    let mut result: HashMap<String, u16> = Default::default();
    let mut calculated_instrs = vec![false; instrs.len()];
    while !result.contains_key(target) {
        for (calculated, (instr, output)) in calculated_instrs.iter_mut().zip(instrs) {
            if !*calculated {
                if let Some(val) = instr.eval(&result) {
                    result.insert(output.clone(), val);
                    *calculated = true;
                }
            }
        }
    }
    result
}

fn main() {
    let instrs = parse_input(include_str!("../../inputs/day07.txt")).expect("input should parse");
    let values = calculate(&instrs, "a");
    let wire_a = *values.get("a").expect("'a' should be evaluated");
    println!("Part 1: {}", wire_a);

    let new_instrs: Vec<_> = instrs
        .iter()
        .map(|(instr, output)| {
            if output == "b" {
                (Instr::Just(Operand::Val(wire_a)), output.clone())
            } else {
                (instr.clone(), output.clone())
            }
        })
        .collect();
    let values = calculate(&new_instrs, "a");
    println!(
        "Part 2: {}",
        values.get("a").expect("'a' should be evaluated")
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &'static str = "123 -> x\n456 -> y\nx AND y -> d\nx OR y -> e\nx LSHIFT 2 -> f\ny RSHIFT 2 -> g\nNOT x -> h\nNOT y -> i";

    #[test]
    fn sample_parses() {
        assert_eq!(
            parse_input(SAMPLE),
            Ok(vec![
                (Instr::Just(Operand::Val(123)), "x".to_owned()),
                (Instr::Just(Operand::Val(456)), "y".to_owned()),
                (
                    Instr::And(Operand::Wire("x".to_owned()), Operand::Wire("y".to_owned())),
                    "d".to_owned()
                ),
                (
                    Instr::Or(Operand::Wire("x".to_owned()), Operand::Wire("y".to_owned())),
                    "e".to_owned()
                ),
                (
                    Instr::Lshift(Operand::Wire("x".to_owned()), Operand::Val(2)),
                    "f".to_owned()
                ),
                (
                    Instr::Rshift(Operand::Wire("y".to_owned()), Operand::Val(2)),
                    "g".to_owned()
                ),
                (Instr::Not(Operand::Wire("x".to_owned())), "h".to_owned()),
                (Instr::Not(Operand::Wire("y".to_owned())), "i".to_owned()),
            ])
        );
    }

    #[test]
    fn sample_evaluates() {
        let instrs = parse_input(SAMPLE).unwrap();
        let values = calculate(&instrs, "i");
        assert_eq!(values.get("d"), Some(&72));
        assert_eq!(values.get("e"), Some(&507));
        assert_eq!(values.get("f"), Some(&492));
        assert_eq!(values.get("g"), Some(&114));
        assert_eq!(values.get("h"), Some(&65412));
        assert_eq!(values.get("i"), Some(&65079));
        assert_eq!(values.get("x"), Some(&123));
        assert_eq!(values.get("y"), Some(&456));
    }
}
