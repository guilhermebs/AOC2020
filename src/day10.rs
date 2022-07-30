use std::fs;

#[allow(dead_code)]
pub fn day10() {
    let buffer = fs::read_to_string("inputs/day10").unwrap();
    let adapters = buffer
        .split('\n')
        .filter(|x| x.len() > 0)
        .into_iter()
        .map(|x| x.parse::<u64>().unwrap())
        .collect::<Vec<u64>>();
    
    let mut joltages = adapters.to_owned();
    joltages.push(0);
    joltages.push(adapters.iter().max().unwrap() + 3);
    joltages.sort();

    let differences = joltages
        .iter()
        .zip(joltages[1..].iter())
        .map(|(x1, x2)| x2 - x1)
        .collect::<Vec<u64>>();

    println!("{:?}", differences);
    let prob1_sol =
        differences.iter().filter(|&&x| x == 1).count() *
        differences.iter().filter(|&&x| x == 3).count();
    
    println!("Part 1: {:?}", prob1_sol)
}
