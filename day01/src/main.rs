use std::fs::File;
use std::io::{self, BufRead};

fn main() {

    let test_vec = vec![1721, 979, 366, 299, 675, 1456];
    let result = find_sum_3(test_vec, 2020);

    match result {
        Some(res) => println!("Solution: {}", res),
        None => print!("No solution!")
    }

    let mut problem_vec: Vec<i32> = Vec::new();
    let file = File::open("input").unwrap();

    for line in io::BufReader::new(file).lines() {
        match line {
            Ok(ln) =>  problem_vec.push(ln.parse().unwrap()),
            _ => println!("failed to read")
        }
    }

    let result = find_sum_3(problem_vec, 2020);

    match result {
        Some(res) => println!("Solution: {}", res),
        None => print!("No solution!")
    }

}


fn find_sum(values: Vec<i32>, target: i32) -> Option<i32> {
    for (i, v1) in values.iter().enumerate() {
        for v2 in values[i+1..].iter() {
            if *v1 + *v2 == target {
                return Some(*v1 * *v2);
            }
        }
    }
    None
}

fn find_sum_3(values: Vec<i32>, target: i32) -> Option<i32> {
    for (i, v1) in values.iter().enumerate() {
        for (j, v2) in values[i+1..].iter().enumerate() {
            for v3 in values[i+j+1..].iter() {
                if *v1 + *v2 + *v3 == target {
                    return Some(*v1 * *v2 * *v3);
                }
            }
        }
    }
    None
}