use std::{collections::HashMap, fs};

type TileRegister = HashMap<(i32, i32), bool>;

fn flip_tile(instructions: &str, tiles: &mut TileRegister) {
    let mut iter = instructions.chars();
    let mut pos = (0, 0);
    loop {
        let step = match iter.next() {
            Some('e') => (2, 0),
            Some('w') => (-2, 0),
            Some('n') => match iter.next() {
                Some('e') => (1, 1),
                Some('w') => (-1, 1),
                _ => panic!("Invalid input!"),
            },
            Some('s') => match iter.next() {
                Some('e') => (1, -1),
                Some('w') => (-1, -1),
                _ => panic!("Invalid input!"),
            },
            None => break,
            _ => panic!("Invalid input!"),
        };
        pos.0 += step.0;
        pos.1 += step.1;
    }
    let tile_at_pos = tiles.entry(pos).or_insert(false);
    *tile_at_pos = !*tile_at_pos;
    println!("fliped {:?} to {:?}", pos, tile_at_pos)
}

#[allow(dead_code)]
pub fn day24() {
    let file_contents = fs::read_to_string("inputs/day24").unwrap();
    let mut tiles = TileRegister::new();
    for line in file_contents.split('\n').filter(|ln| ln.len() > 0) {
        flip_tile(line, &mut tiles);
    }
    let part1_sol = tiles.values().fold(0, |acc, &v| acc + v as i32);
    println!("{:?}", part1_sol)
}
