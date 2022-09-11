use std::fs;
use gcd::Gcd;
use itertools::Itertools;


fn find_bus_at_time(buses: &Vec<Option<u64>>, time: u64) -> Vec<u64> {
    buses.iter()
    .filter(|&x| x.is_some() && time % x.unwrap() == 0)
    .map(|x| x.unwrap())
    .collect_vec()
}


fn update_t_init_and_step_size(t_init: u64, step_size: u64, period: u64, offest: usize) -> (u64, u64) {
    let mut t = t_init;
    while (t + offest as u64) % period != 0 {
        t += step_size;
    }
    (t, (step_size * period)/step_size.gcd(period))
}

#[allow(dead_code)]
pub fn day13() {
    let t_init = 1001938u64;
    let buses = fs::read_to_string("inputs/day13").unwrap().split('\n')
        .nth(1).unwrap().split(',').map(|x| x.parse::<u64>().ok()).collect::<Vec<Option<u64>>>();
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
    let mut step_size: u64 = 1;
    let mut t_init: u64 = 0;
    for (offset, bus) in buses.iter().enumerate() {
        match bus {
            Some(period) => { (t_init, step_size) = update_t_init_and_step_size(t_init, step_size, *period, offset); },
            None => ()
        }
    }
    println!("Part 2: {:?}", t_init);
}