use std::fs;

use itertools::Itertools;


fn find_bus_at_time(buses: &Vec<Option<u32>>, time: u32) -> Vec<u32> {
    buses.iter()
    .filter(|&x| x.is_some() && time % x.unwrap() == 0)
    .map(|x| x.unwrap())
    .collect_vec()
}

#[allow(dead_code)]
pub fn day13() {
    let t_init = 1001938u32;
    let buses = fs::read_to_string("inputs/day13").unwrap().split('\n')
        .nth(1).unwrap().split(',').map(|x| x.parse::<u32>().ok()).collect::<Vec<Option<u32>>>();
    println!("{:?}", buses);
    let mut t = t_init;
    let mut bus_t: Vec<u32>;
    loop {
        bus_t = find_bus_at_time(&buses, t);
        if bus_t.len() > 0 { break; }
        t += 1 
    }
    let part1_sol = (t - t_init) * bus_t.first().unwrap();
    println!("Part 1: {:?}", part1_sol);


}