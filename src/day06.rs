use std::{fs, collections::{HashSet}};

use itertools::Itertools;

#[allow(dead_code)]
pub fn day06() {
    let buffer = fs::read_to_string("inputs/day06").unwrap();
    let part1_sol: usize = buffer.split("\n\n")
        .map(|s| s.replace("\n", "").chars().unique().count())
        .sum();
    println!("Part 1 solution: {}", part1_sol);
    let part2_sol: usize = buffer.split("\n\n")
        .map(|s| {
            println!("{}\n", s);
            println!("{:?}\n", s.split("\n").filter(|s| s.len() > 0).map(|a| HashSet::<char>::from_iter(a.chars())).collect::<Vec<HashSet<char>>>());
            s.split("\n").filter(|s| s.len() > 0)
            .map(|a| HashSet::from_iter(a.chars()))
            .reduce(|accum: HashSet<char>, hm| HashSet::from_iter(accum.intersection(&hm).cloned()))
            .unwrap().len()
        })
        .sum();
    println!("Part 2 solution: {}", part2_sol);
}
