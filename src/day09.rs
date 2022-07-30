use std::{fs, iter::Sum};

use itertools::Itertools;

#[allow(dead_code)]
pub fn day09() {
    let buffer = fs::read_to_string("inputs/day09").unwrap();
    let data_stream = buffer
        .split('\n')
        .filter(|x| x.len() > 0)
        .into_iter()
        .map(|x| x.parse::<u64>().unwrap())
        .collect::<Vec<u64>>();
    let preamble_len: usize = 25;

    let part1_sol = data_stream
        .windows(preamble_len + 1)
        .filter_map(|w| {
            let (new_val, preamble) = w.split_last().unwrap();
            if is_sum(new_val, preamble) {
                None
            } else {
                Some(new_val)
            }
        })
        .collect::<Vec<&u64>>();

    println!("Part 1 solution: {:?}", part1_sol)
}

fn is_sum<'a, T>(new_val: &T, preamble: &'a [T]) -> bool
where
    T: PartialEq,
    T: Sum<&'a T>,
{
    for pair in preamble.iter().combinations(2) {
        if pair.into_iter().sum::<T>() == *new_val {
            return true;
        }
    }
    return false;
}
