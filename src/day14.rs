use std::{fs, collections::HashMap};

use regex::Regex;

const SIZE: usize = 36;

lazy_static! {
    static ref RE_NUMBER: Regex = Regex::new(r"^mem\[(\d+)\] = (\d+)$").unwrap();
    static ref RE_MASK: Regex = Regex::new(r"^mask = ([01X]+)$").unwrap();
}

#[derive(Clone, Copy)]
struct BinaryNumber {
    bits: [bool; SIZE],
}

impl BinaryNumber {
    pub fn from_number(number: u64) -> BinaryNumber {
        let mut bits = [false; SIZE];
        let mut remainder = number;
        for i in (0..SIZE).rev() {
            bits[i] = (remainder / 2u64.pow(i as u32)) == 1;
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
    bits: [Option<bool>; SIZE],
}

impl BinaryMask {
    pub fn from_string(string: &str) -> BinaryMask {
        let mut bits = [Some(true); SIZE];
        for (i, c) in string.chars().rev().enumerate() {
            bits[i] = match c {
                '1' => Some(true),
                '0' => Some(false),
                'X' => None,
                _ => panic!("Invalid character {:?}", c),
            }
        }
        BinaryMask { bits: bits }
    }

    pub fn apply(&self, number: BinaryNumber) -> BinaryNumber {
        let mut result = number.clone();
        for (i, mask_bit) in self.bits.iter().enumerate() {
            match mask_bit {
                Some(v) => result.bits[i] = *v,
                None => (),
            }
        }
        result
    }

    pub fn apply_address(&self, address: u64) -> Vec<u64> {
        let mut masked_address: Vec<BinaryNumber> = vec![BinaryNumber::from_number(address)];
        for (i, mask_bit) in self.bits.iter().enumerate() {
            match mask_bit {
                Some(true) => masked_address.iter_mut().for_each(|x| x.bits[i] = true),
                Some(false) => (),
                None => {
                    masked_address.iter_mut().for_each(|x| x.bits[i] = true);
                    let mut results_false = masked_address.iter().map(|x| {
                        let mut y = x.clone();
                        y.bits[i] = false;
                        y
                    }).collect::<Vec<BinaryNumber>>();
                    masked_address.append(&mut results_false);
                }
            }
        }
        masked_address.iter().map(|x| x.to_number()).collect::<Vec<u64>>()
    }
}

#[allow(dead_code)]
pub fn day14() {
    let decimal = 16121;
    let sut = BinaryNumber::from_number(decimal);
    assert_eq!(decimal, sut.to_number());

    let input_contents = fs::read_to_string("inputs/day14").unwrap();
    let mut mask = BinaryMask {
        bits: [Some(false); SIZE],
    };

    let mut memory: HashMap<u64, u64> = HashMap::new();

    for line in input_contents.split("\n") {
        if line.len() == 0 {
            continue;
        }
        match RE_MASK.captures(line) {
            Some(c) => mask = BinaryMask::from_string(&c[1]),
            None => (),
        }
        match RE_NUMBER.captures(line) {
            Some(c) => {
                let address = c[1].parse::<u64>().unwrap();
                let value = BinaryNumber::from_number(c[2].parse::<u64>().unwrap());
                memory.insert(address, mask.apply(value).to_number());
            }
            None => (),
        }
    }

    let part1_sol = memory.iter().fold(0u64, |acc, (_, val)| acc + val);
    println!("Part 1: {:?}", part1_sol);


    let mut memory: HashMap<u64, u64> = HashMap::new();

    for line in input_contents.split("\n") {
        if line.len() == 0 {
            continue;
        }
        match RE_MASK.captures(line) {
            Some(c) => mask = BinaryMask::from_string(&c[1]),
            None => (),
        }
        match RE_NUMBER.captures(line) {
            Some(c) => {
                let base_address = c[1].parse::<u64>().unwrap();
                let value = c[2].parse::<u64>().unwrap();
                for address in mask.apply_address(base_address) {
                    memory.insert(address, value);
                }
            }
            None => (),
        }
    }

    let part2_sol = memory.iter().fold(0u64, |acc, (_, val)| acc + val );
    println!("Part 2: {:?}", part2_sol);

}
