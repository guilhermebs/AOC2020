use std::time::Instant;

mod day01;
mod day02;
mod day03;

fn main() {
    let start = Instant::now();
    //day01::day01();
    //day02::day02();
    day03::day03();
    let duration = start.elapsed();
    println!("Time elapsed: {:?}", duration)
}
