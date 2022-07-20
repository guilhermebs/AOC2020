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
        println!("{:?}, {:?}, {:?}", cur_instr, inst, val);
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

    println!("{:?}", part1_sol);
}
