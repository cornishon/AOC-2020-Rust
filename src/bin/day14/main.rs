use itertools::Itertools;
use std::{collections::HashMap, env::args, error::Error, fs, str};

use nom::{
    branch,
    bytes::complete::tag,
    character::complete::{alphanumeric1, char, u64},
    sequence::delimited,
    IResult,
};

fn main() -> Result<(), Box<dyn Error>> {
    let path = args()
        .nth(1)
        .ok_or("path to input file not provided".to_owned())?;
    let input = fs::read_to_string(path)?;
    let program = parse(&input)?;

    let mut sys = SystemV1::new();
    let ans1 = sys.run(&program);
    println!("Part1: {ans1}");

    let mut sys = SystemV2::new();
    let ans2 = sys.run(&program);
    println!("Part1: {ans2}");

    Ok(())
}

#[derive(Default)]
struct SystemV1 {
    mem: HashMap<u64, u64>,
    mask1: u64,
    mask2: u64,
}

#[derive(Default)]
struct SystemV2<'a> {
    mem: HashMap<u64, u64>,
    mask: &'a str,
}

impl<'a> SystemV2<'a> {
    fn new() -> Self {
        Self::default()
    }

    fn step(&mut self, instr: &Instruction<'a>) {
        match *instr {
            Instruction::Mask(mask) => {
                self.mask = mask;
            }
            Instruction::Mem(address, value) => {
                for addr in generate_addresses(self.mask, address) {
                    self.mem.insert(addr, value);
                }
            }
        }
    }

    fn run(&mut self, program: &[Instruction<'a>]) -> u64 {
        for instruction in program {
            self.step(instruction);
        }

        self.mem.values().sum()
    }
}

impl SystemV1 {
    fn new() -> Self {
        Self::default()
    }

    fn step(&mut self, instr: &Instruction) {
        match *instr {
            Instruction::Mask(mask) => {
                self.mask1 = u64::from_str_radix(&mask.replace('X', "1"), 2).unwrap();
                self.mask2 = u64::from_str_radix(&mask.replace('X', "0"), 2).unwrap();
            }
            Instruction::Mem(address, value) => {
                self.mem.insert(address, (value & self.mask1) | self.mask2);
            }
        }
    }

    fn run(&mut self, program: &[Instruction]) -> u64 {
        for instruction in program {
            self.step(instruction);
        }

        self.mem.values().sum()
    }
}

fn generate_addresses(mask: &str, address: u64) -> impl Iterator<Item = u64> + '_ {
    let bytes = mask.as_bytes();
    let n = bytes.iter().filter(|x| **x == b'X').count();

    // map over all possible combinations of 0's an 1's for the 'X' bytes
    (0..n)
        .map(|_| 0..=1)
        .multi_cartesian_product()
        .map(move |v| {
            let mut addr = address;
            let mut v = v.iter();
            for (i, byte) in bytes.iter().rev().enumerate() {
                if *byte == b'X' {
                    // if the byte was 'floating' we must set the corresponding byte to the relevant 0 or 1
                    // we calculated with the cartesian product. This is how we could achieve it:
                    //addr = (addr & !(1 << i)) | (x << i);
                    // (the first part clears the byte and the second then sets it to x).
                    //
                    // But since we are going over all possible combinations of 0's and 1's,
                    // there will be the same amount of either of them and so by xoring the address bit
                    // with the current value of 'X' and we will end up with the same combinations, just
                    // in a different order
                    let x = *v.next().unwrap() as u64;
                    addr ^= x << i;
                } else {
                    // if the byte was a 0 or 1 we *or* it with the corresponding byte of the memory
                    // address and set it to the result (1 sets the bit to 1, 0 leaves it unchanged)
                    let bit = (*byte - b'0') as u64;
                    addr |= bit << i;
                }
            }
            addr
        })
}

#[derive(PartialEq, Debug)]
enum Instruction<'a> {
    Mask(&'a str),
    Mem(u64, u64),
}

fn parse(input: &str) -> Result<Vec<Instruction>, String> {
    let mut instructions = Vec::new();
    for line in input.lines() {
        let (_, instr) = parse_line(line)
            .map_err(|_| format!("Encountered unexpected line while parsing: `{}`", line))?;
        instructions.push(instr);
    }
    Ok(instructions)
}

fn parse_line(input: &str) -> IResult<&str, Instruction> {
    branch::alt((parse_mask, parse_mem))(input)
}

fn parse_mem(input: &str) -> IResult<&str, Instruction> {
    let (input, _) = tag("mem")(input)?;
    let (input, index) = delimited(char('['), u64, char(']'))(input)?;
    let (input, _) = tag(" = ")(input)?;
    let (input, value) = u64(input)?;
    Ok((input, Instruction::Mem(index, value)))
}

#[test]
fn mem() {
    let input = "mem[7] = 101";
    assert_eq!(parse_mem(input), Ok(("", Instruction::Mem(7u64, 101u64))));
}

fn parse_mask(input: &str) -> IResult<&str, Instruction> {
    let (input, _) = tag("mask = ")(input)?;
    let (input, output) = alphanumeric1(input)?;
    Ok((input, Instruction::Mask(output)))
}

#[test]
fn mask() {
    let input = "mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X";
    assert_eq!(
        parse_mask(input),
        Ok((
            "",
            Instruction::Mask("XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X")
        ))
    );
}
