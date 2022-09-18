use std::collections::hash_map::Entry;
use rustc_hash::FxHashMap;

#[allow(dead_code)]
pub fn day15() {
    let init_sequence: Vec<usize> = vec![0, 5, 4, 1, 10, 14, 7];

    let mut last_spoken = FxHashMap::<usize, usize>::from_iter(
        init_sequence[..init_sequence.len() - 1]
            .iter()
            .enumerate()
            .map(|(i, v)| (v.to_owned(), i + 1)),
    );

    let mut cur_value = init_sequence.last().unwrap().to_owned();

    let final_i = 2020;

    for i in init_sequence.len()..final_i {
        let new_value = match last_spoken.entry(cur_value) {
            Entry::Occupied(v) => i - v.get(),
            Entry::Vacant(_) => 0,
        };
        last_spoken.insert(cur_value, i);
        cur_value = new_value;
    }

    println!("Part 1: {:?}", cur_value);

    let final_i_part2 = 30000000;

    for i in final_i..final_i_part2 {
        let new_value = match last_spoken.entry(cur_value) {
            Entry::Occupied(v) => i - v.get(),
            Entry::Vacant(_) => 0,
        };
        last_spoken.insert(cur_value, i);
        cur_value = new_value;
    }

    println!("Part 2: {:?}", cur_value);
}
