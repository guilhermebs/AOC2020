use std::fs;

use regex::Regex;

const SIZE: usize = 36;

lazy_static! {
    static ref RE_NUMBER: Regex = Regex::new(r"^mem\[(\d+)\] = (\d+)$").unwrap();
    static ref RE_MASK: Regex = Regex::new(r"^mask = ([01X]+)$").unwrap();
}

#[derive(Clone)]
struct BinaryNumber {
    bits: [bool; SIZE]
}

impl BinaryNumber {
    pub fn from_number(number: u64) -> BinaryNumber {
        let mut bits = [false; SIZE];
        let mut remainder = number;
        for i in (0..SIZE).rev() {
            bits[i] = (remainder/2u64.pow(i as u32)) == 1;
            remainder -= (bits[i] as u64) * (2u64.pow(i as u32));
        }
        BinaryNumber { bits: bits }
    }
    pub fn to_number(&self) -> u64 {
        let mut result = 0u64;
        for i in (0..SIZE).rev() {
            result += (self.bits[i] as u64) * (2u64.pow(i as u32));
        }
        result
    }
}

struct BinaryMask {
    bits: [Option<bool>; SIZE]
}

impl BinaryMask {
    pub fn from_string(string: &str) -> BinaryMask {
        let mut bits = [Some(true); SIZE];
        for (i, c) in string.chars().rev().enumerate() {
            bits[i] = match c {
               '1' => Some(true),
               '0' => Some(false),
               'X' => None,
               _ => panic!("Invalid character {:?}", c)
            }
        }
        BinaryMask { bits: bits }
    }

    pub fn apply(&self, number: BinaryNumber) -> BinaryNumber {
        let mut result = number.clone();
        for (i, mask_bit) in self.bits.iter().enumerate() {
            match mask_bit {
                Some(v) => result.bits[i] = *v,
                None => ()
            }
        }
        result
    }
}


#[allow(dead_code)]
pub fn day14() {
    let decimal = 16121;
    let sut = BinaryNumber::from_number(decimal);
    assert_eq!(decimal, sut.to_number());

    let input_contents = fs::read_to_string("inputs/day14").unwrap();
    let mut mask = BinaryMask{bits: [Some(false); SIZE]};
    let mut memory: Vec<Option<BinaryNumber>> = Vec::new();

    for line in input_contents.split("\n") {
        if line.len() == 0 { continue; }
        match RE_MASK.captures(line) {
            Some(c) => mask = BinaryMask::from_string(&c[1]),
            None => (),
        }
        match RE_NUMBER.captures(line) {
            Some(c) => {
                let address = c[1].parse::<usize>().unwrap();
                let value = BinaryNumber::from_number(c[2].parse::<u64>().unwrap());
                if address > memory.len().saturating_sub(1) { memory.resize(address + 1, None) }
                memory[address] = Some(mask.apply(value));
            }
            None => ()
        }
    }

    let part1_sol = memory.iter()
        .fold(0u64, |acc, val| match val {
            Some(n) => acc + n.to_number(),
            None => acc
        });
    println!("Part 1: {:?}", part1_sol);

}