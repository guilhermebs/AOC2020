use itertools::Itertools;

fn move_cups(cups: Vec<u32>) -> Vec<u32> {
    let current_cup = cups[0];
    let mut pickup = cups[1..4].to_vec();
    let max_val = cups[4..].iter().max().unwrap();
    let mut destination = current_cup - 1;
    let insert_position = loop {
        let index = cups[4..].iter().position(|&c| c == destination);
        match index {
            Some(i) => break i + 5,
            None => (),
        }
        if destination == 0 {
            destination = *max_val;
        } else {
            destination -= 1;
        }
    };

    let mut result = cups[4..insert_position].to_vec();
    result.append(&mut pickup);
    result.append(&mut cups[insert_position..].to_vec());
    result.push(current_cup);
    return result;
}

fn final_order(cups: Vec<u32>) -> String {
    let index_1 = cups.iter().position(|&c| c == 1).unwrap();
    let mut result = cups[index_1 + 1..]
        .iter()
        .map(|x| x.to_string())
        .collect::<String>();
    result.push_str(
        &cups[0..index_1]
            .iter()
            .map(|x| x.to_string())
            .collect::<String>(),
    );
    return result;
}



#[allow(dead_code)]
pub fn day23() {
    const RADIX: u32 = 10;
    let input = "284573961"; // "389125467";

    let init_cups = input
        .chars()
        .map(|x| x.to_digit(RADIX).unwrap())
        .collect_vec();

    let mut cups = init_cups.clone();

    for _ in 0..100 {
        cups = move_cups(cups);
    }
    println!("Part 1: {:}", final_order(cups));

    let mut cups = [init_cups, (10..1000001).into_iter().collect_vec()].concat();
    for _i in 0..1000 {
        cups = move_cups(cups);
    }

    let index_1 = cups.iter().position(|&c| c == 1).unwrap();
    println!("Part 2: {:}", cups[index_1 + 1] * cups[index_1 + 2]);
}
