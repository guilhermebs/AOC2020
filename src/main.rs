#[macro_use]
extern crate lazy_static;

use std::time::Instant;

mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
mod day08;
mod day09;
mod day10;
mod day11;
mod day12;
mod day13;

fn main() {
    let start = Instant::now();
    //day01::day01();
    //day02::day02();
    //day03::day03();
    //day04::day04();
    //day05::day05();
    //day06::day06();
    //day07::day07();
    //day08::day08();
    //day09::day09();
    //day10::day10();
    //day11::day11();
    //day12::day12(); 
    day13::day13(); 
    let duration = start.elapsed();
    println!("Time elapsed: {:?}", duration)
}
