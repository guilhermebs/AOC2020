use std::{collections::HashMap, fs};

type TilePos = (i32, i32);
type TileRegister = HashMap<TilePos, bool>;

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

fn art_exibit(tiles: &mut TileRegister) {
    let mut adjacent_black = HashMap::<TilePos, u32>::new();
    const adjacent_pos: [TilePos; 6] = [(1, 1), (2, 0), (1, -1), (-1, -1), (-2, 0), (-1, 1)];
    for (&pos, _) in tiles.iter().filter(|(_, &x)| x) {
        for step in adjacent_pos {
            *adjacent_black
                .entry((pos.0 + step.0, pos.1 + step.1))
                .or_insert(0) += 1
        }
    }
    // flip black tiles
    for (pos, value) in tiles.iter_mut().filter(|(_, x)| **x) {
        let adjacent_b = match adjacent_black.get(pos) {
            Some(n) => n,
            None => &0,
        };
        if adjacent_b == &0 || adjacent_b > &2 {
            *value = false;
        }
    }
    // flip white tiles
    for (pos, _) in adjacent_black.iter().filter(|(_, &n)| n == 2) {
        let tile = tiles.entry(*pos).or_insert(false);
        if !*tile {
            *tile = true
        }
    }
}

#[allow(dead_code)]
pub fn day24() {
    let file_contents = fs::read_to_string("inputs/day24_test").unwrap();
    let mut tiles = TileRegister::new();
    for line in file_contents.split('\n').filter(|ln| ln.len() > 0) {
        flip_tile(line, &mut tiles);
    }
    let part1_sol = tiles.values().fold(0, |acc, &v| acc + v as i32);
    println!("{:?}", part1_sol)
}
