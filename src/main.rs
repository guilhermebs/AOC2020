use std::time::Instant;

mod day01;
mod day02;

fn main() {
    let start = Instant::now();
    //day01::day01();
    day02::day02();
    let duration = start.elapsed();
    println!("Time elapsed: {:?}", duration)
}
