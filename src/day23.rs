
use itertools::Itertools;

fn move_cups(current_cup: usize, next_cups: &mut Vec<usize>) -> usize {
    let pickup = [next_cups[current_cup], next_cups[next_cups[current_cup]], next_cups[next_cups[next_cups[current_cup]]]];
    let mut destination = if current_cup > 0 {current_cup - 1} else {next_cups.len() - 1};
    while pickup.contains(&destination) {
        if destination == 0 {
            destination = next_cups.len() - 1;
        }
        else {
            destination -= 1; 
        }
    }
    // Move the next cup
    let next_in_sequence = next_cups[pickup[2]];
    next_cups[current_cup] = next_in_sequence;
    // Move the pickups
    let after_destination = next_cups[destination];
    next_cups[destination] = pickup[0];
    next_cups[pickup[2]] = after_destination;

    return next_in_sequence;

}

fn next_cups2sequence(start_cup: usize, next_cups: &Vec<usize>) -> Vec<usize> {
    let mut result = vec![];
    let mut cup = start_cup;
    while next_cups[cup] != start_cup {
        cup = next_cups[cup];
       result.push(cup + 1) 
    }

    return result;
}



#[allow(dead_code)]
pub fn day23() {
    const RADIX: u32 = 10;
    let input = "284573961";

    let init_cups = input
        .chars()
        .map(|x| x.to_digit(RADIX).unwrap() as usize)
        .collect_vec();

    let mut next_cups = vec![0; init_cups.len()];
    for (&c, &nc) in init_cups.iter().zip(init_cups[1..].iter()) {
        next_cups[c - 1] = nc - 1;
    }
    next_cups[init_cups[init_cups.len()-1] - 1] = init_cups[0] - 1;

    let mut current_cup = init_cups[0] - 1;

    for _ in 0..100 {
        current_cup = move_cups(current_cup, &mut next_cups);
    }
    println!("Part 1: {:}", next_cups2sequence(0, &next_cups).iter().map(|x| x.to_string()).join(""));

    let cups = [init_cups, (10..1000001).into_iter().collect_vec()].concat();
    let mut next_cups = vec![0; cups.len()];
    for (&c, &nc) in cups.iter().zip(cups[1..].iter()) {
        next_cups[c - 1] = nc - 1;
    }
    next_cups[cups[cups.len()-1] - 1] = cups[0] - 1;
    let mut current_cup = cups[0] - 1;

    for _ in 0..10000000 {
        current_cup = move_cups(current_cup, &mut next_cups);
    }
    println!("Part 2: {:}", (next_cups[0] + 1) * (next_cups[next_cups[0]] + 1));
}
