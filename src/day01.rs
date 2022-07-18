use std::fs::File;
use std::io::{self, BufRead};

use itertools::Itertools;

pub fn day01() {

    let test_vec = vec![1721, 979, 366, 299, 675, 1456];

    match find_sum_n(test_vec, 2020, 3) {
        Some(res) => println!("Solution: {}", res),
        None => print!("No solution!")
    }

    let mut problem_vec: Vec<i32> = Vec::new();
    let file = File::open("inputs/day01").unwrap();

    for line in io::BufReader::new(file).lines() {
        match line {
            Ok(ln) =>  problem_vec.push(ln.parse().unwrap()),
            _ => println!("failed to read")
        }
    }

    match find_sum_n(problem_vec, 2020, 3) {
        Some(res) => println!("Solution: {}", res),
        None => print!("No solution!")
    }

}


fn find_sum_n(values: Vec<i32>, target: i32, n: usize) -> Option<i32> {
    for v in values.into_iter().combinations(n) {
        if v.iter().sum::<i32>() == target {
            return Some(v.iter().product());
        }
    }
    None
}