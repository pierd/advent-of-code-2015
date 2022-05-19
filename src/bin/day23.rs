use std::str::FromStr;

#[derive(Clone, Debug)]
enum Error {
    UnexpectedEndOfString,
    UnknownRegister(String),
    UnknownInstruction(String),
    IntegerParseFailed(String),
}

#[derive(Clone, Copy, Debug)]
enum Register {
    A,
    B,
}

impl FromStr for Register {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "a" => Ok(Self::A),
            "b" => Ok(Self::B),
            _ => Err(()),
        }
    }
}

#[derive(Clone, Copy, Debug, Default)]
struct State {
    a: usize,
    b: usize,
    instr_pointer: isize,
}

impl State {
    fn with_register_modification<F: Fn(usize) -> usize>(
        self,
        register: Register,
        modification: F,
    ) -> Self {
        match register {
            Register::A => Self {
                a: modification(self.a),
                instr_pointer: self.instr_pointer + 1,
                ..self
            },
            Register::B => Self {
                b: modification(self.b),
                instr_pointer: self.instr_pointer + 1,
                ..self
            },
        }
    }

    fn with_register_jump<F: Fn(usize) -> isize>(self, register: Register, jump: F) -> Self {
        match register {
            Register::A => Self {
                instr_pointer: self.instr_pointer + jump(self.a),
                ..self
            },
            Register::B => Self {
                instr_pointer: self.instr_pointer + jump(self.b),
                ..self
            },
        }
    }

    fn with_jump(self, offset: isize) -> Self {
        Self {
            instr_pointer: self.instr_pointer + offset,
            ..self
        }
    }
}

#[derive(Clone, Copy, Debug)]
enum Instruction {
    Halve(Register),
    Triple(Register),
    Increment(Register),
    Jump(isize),
    JumpIfEven(Register, isize),
    JumpIfOne(Register, isize),
}

impl Instruction {
    fn execute(&self, state: State) -> State {
        match self {
            Instruction::Halve(r) => state.with_register_modification(*r, |x| x / 2),
            Instruction::Triple(r) => state.with_register_modification(*r, |x| x * 3),
            Instruction::Increment(r) => state.with_register_modification(*r, |x| x + 1),
            Instruction::Jump(offset) => state.with_jump(*offset),
            Instruction::JumpIfEven(r, offset) => {
                state.with_register_jump(*r, |x| if x % 2 == 0 { *offset } else { 1 })
            }
            Instruction::JumpIfOne(r, offset) => {
                state.with_register_jump(*r, |x| if x == 1 { *offset } else { 1 })
            }
        }
    }

    fn execute_all(mut state: State, instrs: &[Self]) -> State {
        while let Some(instr) = usize::try_from(state.instr_pointer)
            .ok()
            .and_then(|idx| instrs.get(idx))
        {
            state = instr.execute(state);
        }
        state
    }
}

impl FromStr for Instruction {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split(&[' ', ',']).filter(|s| !s.is_empty());
        let parse_register = |s: Option<&str>| {
            s.ok_or(Error::UnexpectedEndOfString)?
                .parse::<Register>()
                .map_err(|_| Error::UnknownRegister(s.unwrap().to_owned()))
        };
        let parse_int = |s: Option<&str>| {
            s.ok_or(Error::UnexpectedEndOfString)?
                .parse::<isize>()
                .map_err(|_| Error::IntegerParseFailed(s.unwrap().to_owned()))
        };
        match parts.next() {
            Some("hlf") => Ok(Self::Halve(parse_register(parts.next())?)),
            Some("tpl") => Ok(Self::Triple(parse_register(parts.next())?)),
            Some("inc") => Ok(Self::Increment(parse_register(parts.next())?)),
            Some("jmp") => Ok(Self::Jump(parse_int(parts.next())?)),
            Some("jie") => Ok(Self::JumpIfEven(
                parse_register(parts.next())?,
                parse_int(parts.next())?,
            )),
            Some("jio") => Ok(Self::JumpIfOne(
                parse_register(parts.next())?,
                parse_int(parts.next())?,
            )),
            Some(wtf) => Err(Error::UnknownInstruction(wtf.to_owned())),
            None => Err(Error::UnexpectedEndOfString),
        }
    }
}

fn main() {
    let instrs = include_str!("../../inputs/day23.txt")
        .lines()
        .map(|line| line.parse::<Instruction>())
        .collect::<Result<Vec<_>, _>>()
        .expect("input should parse");
    println!(
        "Part 1: {}",
        Instruction::execute_all(Default::default(), &instrs).b
    );
    println!(
        "Part 2: {}",
        Instruction::execute_all(
            State {
                a: 1,
                b: 0,
                instr_pointer: 0
            },
            &instrs
        )
        .b
    );
}
