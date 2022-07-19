use std::fs::File;
use std::io::{self, BufRead};


#[allow(dead_code)]
fn test_input() -> Vec<&'static str> {
    vec!["..##.......",
         "#...#...#..",
         ".#....#..#.",
         "..#.#...#.#",
         ".#...##..#.",
         "..#.##.....",
         ".#.#.#....#",
         ".#........#",
         "#.##...#...",
         "#...##....#",
         ".#..#...#.#"]
}

#[allow(dead_code)]
pub fn day03() {
    //let input = test_input(); 
    let mut input = Vec::<String>::new(); 
    let file = File::open("inputs/day03").unwrap();
    for line in io::BufReader::new(file).lines() {
        match line {
            Ok(ln) =>  input.push(ln),
            _ => println!("failed to read")
        }
    }

    let trees: Vec<Vec<bool>> = input.iter()
            .map(|x| x.chars().map(|c| c == '#').collect())
            .collect();
    
    let tree_count = slide(&trees, 3, 1);
    println!("Part 1: tree count: {}", tree_count);

    let part2_sol = {
        tree_count *
        slide(&trees, 1, 1) *
        slide(&trees, 5, 1) * 
        slide(&trees, 7, 1) * 
        slide(&trees, 1, 2)
    };
    println!("Part 2: tree count: {}", part2_sol);
}

fn slide(trees: &Vec<Vec<bool>>, x_slope: usize, y_slope: usize) -> u64 {
    let h = trees.len();
    let w = trees[0].len();
    let mut i = 0;
    let mut j = 0;
    let mut tree_count = 0;
    while i < h {
       tree_count += trees[i][j%w] as u64;
       i += y_slope;
       j += x_slope;
    }
    tree_count
}