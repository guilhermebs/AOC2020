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

    let prob1_sol = differences.iter().filter(|&&x| x == 1).count()
        * differences.iter().filter(|&&x| x == 3).count();

    println!("Part 1: {:?}", prob1_sol);

    let mut number_with_joltage = vec![0 as u64; (joltages.last().unwrap() + 1) as usize];
    number_with_joltage[0] = 1;

    for ad in joltages.iter().map(|x| *x as usize) {
        let lb = 0.max((ad as i32) - 3) as usize;
        for joltage in lb..ad {
            number_with_joltage[ad] += number_with_joltage[joltage];
        }
    }

    println!("Part 2: {:?}", number_with_joltage.last());
}
