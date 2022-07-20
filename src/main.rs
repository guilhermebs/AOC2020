#[macro_use]
extern crate lazy_static;

use std::time::Instant;

mod day01;
mod day02;
mod day03;
mod day04;
mod day05;

fn main() {
    let start = Instant::now();
    //day01::day01();
    //day02::day02();
    //day03::day03();
    //day04::day04();
    day05::day05();
    let duration = start.elapsed();
    println!("Time elapsed: {:?}", duration)
}
