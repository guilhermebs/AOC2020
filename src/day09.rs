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
        .collect::<Vec<&u64>>()
        .first()
        .unwrap()
        .to_owned();

    println!("Part 1 solution: {:?}", part1_sol);

    let part2_sol = solve_part2(&data_stream, part1_sol);

    println!("Part 2 solution: {:?}", part2_sol);
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

fn solve_part2(data_stream: &Vec<u64>, part1_sol: &u64) -> Option<u64> {
    for (i, v) in data_stream.iter().enumerate() {
        let mut acc = v.to_owned();
        let mut j = i;
        for v in &data_stream[i + 1..] {
            acc += v;
            j += 1;
            if &acc >= part1_sol {
                break;
            };
        }
        if &acc == part1_sol {
            return Some(data_stream[i..j].iter().min().unwrap() + data_stream[i..j].iter().max().unwrap());
        }
    }
    return None;
}
