use std::fs;

use itertools::Itertools;


fn find_bus_at_time(buses: &Vec<Option<u64>>, time: u64) -> Vec<u64> {
    buses.iter()
    .filter(|&x| x.is_some() && time % x.unwrap() == 0)
    .map(|x| x.unwrap())
    .collect_vec()
}


fn validate(buses: &Vec<Option<u64>>, time: u64) -> bool {
    for (i, bus) in buses.iter().enumerate() {
        match bus {
            Some(n) => {if (time + i as u64) % n != 0 { return false }},
            None => ()
        };
    }
    true
}

#[allow(dead_code)]
pub fn day13() {
    let t_init = 1001938u64;
    let buses = fs::read_to_string("inputs/day13").unwrap().split('\n')
        .nth(1).unwrap().split(',').map(|x| x.parse::<u64>().ok()).collect::<Vec<Option<u64>>>();
    println!("{:?}", buses);
    let mut t = t_init;
    let mut bus_t: Vec<u64>;
    loop {
        bus_t = find_bus_at_time(&buses, t);
        if bus_t.len() > 0 { break; }
        t += 1 
    }
    let part1_sol = (t - t_init) * bus_t.first().unwrap();
    println!("Part 1: {:?}", part1_sol);

    //let buses = vec![Some(67u64),Some(7u64),None,Some(59u64),Some(61u64)];
    let step_size: u64 = buses.iter().max().unwrap().unwrap() as u64;
    let mut t: u64 = step_size - buses.iter().position_max().unwrap() as u64;
    let mut i = 0;
    while !validate(&buses, t) {
        if i%1000000 == 0 { println!("t={:?}", t) };
        t += step_size;
        i += 1;
    }
    println!("Part 2: {:?}", t);
}