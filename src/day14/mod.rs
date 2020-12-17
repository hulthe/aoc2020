use std::collections::HashMap;

struct Memory {
    data: HashMap<u64, u64>,
}

pub enum Instr {
    Mask { ones: u64, zeros: u64, xes: u64 },
    Mem { at: u64, value: u64 },
}

impl Memory {
    pub fn new() -> Self {
        Self {
            data: HashMap::with_capacity(16000),
        }
    }

    pub fn put(&mut self, at: u64, value: u64) {
        self.data.insert(at, value);
    }

    pub fn sum(&self) -> u64 {
        self.data.values().sum()
    }
}

const BITS: usize = 36;

pub fn parse<'a>(input: &'a str) -> impl Iterator<Item = Instr> + 'a {
    input
        .lines()
        .filter_map(|line| line.split_once(" = "))
        .map(|(op, value)| match op {
            "mask" => {
                let mut xes: u64 = 0;
                let mut zeros: u64 = 0;
                let mut ones: u64 = 0;

                for (i, c) in value.chars().rev().enumerate() {
                    match c {
                        'X' => xes = set_bit(xes, i),
                        '0' => zeros = set_bit(zeros, i),
                        '1' => ones = set_bit(ones, i),
                        _ => panic!("unknown mask character: {}", c),
                    }
                }

                Instr::Mask { zeros, ones, xes }
            }

            mem => {
                let (op, at) = mem.trim_end_matches(']').split_once('[').unwrap();
                debug_assert_eq!(op, "mem");

                Instr::Mem {
                    at: at.parse().expect("failed to parse memory address"),
                    value: value.parse().expect("failed to parse instruction argument"),
                }
            }
        })
}

pub fn part1(input: &str) -> u64 {
    let mut mem = Memory::new();

    let mut and_mask = u64::MAX;
    let mut or_mask = 0;

    for instr in parse(input) {
        match instr {
            Instr::Mask { zeros, ones, .. } => {
                and_mask = !zeros;
                or_mask = ones;
            }
            Instr::Mem { at, value } => mem.put(at, (value & and_mask) | or_mask),
        }
    }

    mem.sum()
}

#[inline(always)]
fn set_bit(val: u64, bit: usize) -> u64 {
    val | 1 << bit
}

#[inline(always)]
fn unset_bit(val: u64, bit: usize) -> u64 {
    val & !(1 << bit)
}

#[inline(always)]
fn get_bit(val: u64, bit: usize) -> bool {
    (val >> bit) & 1 == 1
}

pub fn part2(input: &str) -> u64 {
    let mut mem = Memory::new();

    let mut or_mask = 0;
    let mut float_mask = 0;

    for instr in parse(input) {
        match instr {
            Instr::Mask { xes, ones, .. } => {
                float_mask = xes;
                or_mask = ones;
            }
            Instr::Mem { at, value } => {
                fn set_floating(at: u64, i: usize, float_mask: u64, value: u64, mem: &mut Memory) {
                    for f in i..BITS {
                        if get_bit(float_mask, f) {
                            set_floating(set_bit(at, f), f + 1, float_mask, value, mem);
                            set_floating(unset_bit(at, f), f + 1, float_mask, value, mem);
                            return;
                        }
                    }

                    mem.put(at, value);
                }

                set_floating(at | or_mask, 0, float_mask, value, &mut mem);
            }
        }
    }

    mem.sum()
}

#[cfg(test)]
mod tests {
    use super::{part1, part2};

    #[test]
    pub fn test_part1() {
        let input = include_str!("test-input");
        assert_eq!(part1(input), 165);
    }

    #[test]
    pub fn test_part2() {
        let input = include_str!("test-input2");
        assert_eq!(part2(input), 208);
    }
}
