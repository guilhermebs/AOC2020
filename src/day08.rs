use std::fs;

use regex::Regex;

#[allow(dead_code)]
pub fn day08() {
    let buffer = fs::read_to_string("inputs/day08").unwrap();
    let re: Regex = Regex::new(r"(\w+) ([+-]\d+)").unwrap();

    let instructions = buffer
        .split("\n")
        .filter(|s| s.len() > 0)
        .map(|s| {
            let c = re.captures(s).unwrap();
            (c[1].to_string(), c[2].parse::<i32>().unwrap())
        })
        .collect::<Vec<(String, i32)>>();

    let mut part1_sol: i32 = 0;
    let mut cur_instr: usize = 0;
    let mut n_visits = vec![0; instructions.len()];

    loop {
        let (inst, val) = &instructions[cur_instr];
        n_visits[cur_instr] += 1;
        if n_visits.iter().any(|&n| n > 1) {
            break;
        }
        match inst.as_str() {
            "acc" => {
                part1_sol += val;
                cur_instr += 1
            }
            "jmp" => cur_instr = (cur_instr as i32 + val) as usize,
            _ => cur_instr += 1,
        }
    }

    println!("Part 1 solution: {:?}", part1_sol);

    let mut can_flip = instructions
        .iter()
        .map(|(instr, _)| instr == "jmp" || instr == "nop")
        .collect::<Vec<bool>>();
    let mut part2_sol: i32;

    loop {
        let mut visited = vec![false; instructions.len()];
        let mut has_flipped = false;
        cur_instr = 0;
        part2_sol = 0;
        loop {
            if cur_instr >= instructions.len() || visited[cur_instr] {
                break;
            }
            visited[cur_instr] = true;
            let (inst, val) = &instructions[cur_instr];
            let mut instruction: String = inst.to_owned();
            if !has_flipped {
                let (inst2, flip) = maybe_flip(inst, &can_flip[cur_instr]);
                has_flipped = flip;
                if flip {
                    can_flip[cur_instr] = false;
                };
                instruction = inst2;
            }
            match instruction.as_str() {
                "acc" => {
                    part2_sol += val;
                    cur_instr += 1
                }
                "jmp" => cur_instr = (cur_instr as i32 + val) as usize,
                _ => cur_instr += 1,
            }
        }
        if *visited.last().unwrap() {
            break;
        }
    }
    println!("Part 2: {:?}", part2_sol);
}

fn maybe_flip(inst: &String, can_flip: &bool) -> (String, bool) {
    if !can_flip {
        return (inst.to_owned(), false);
    }
    match inst.as_str() {
        "jmp" => return ("nop".to_string(), true),
        "nop" => return ("jmp".to_string(), true),
        _ => return (inst.to_owned(), false),
    }
}
