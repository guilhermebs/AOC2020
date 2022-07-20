use std::{
    collections::{BTreeSet, HashMap},
    fs,
};

use regex::Regex;

pub fn day07() {
    let mut with_bag = HashMap::<String, Vec<(u32, String)>>::new();
    let mut bags_in = HashMap::<String, Vec<(u32, String)>>::new();
    let buffer = fs::read_to_string("inputs/day07").unwrap();
    let re_name: Regex = Regex::new(r"([ \w]+) bags contain").unwrap();
    let re_contains: Regex = Regex::new(r"(\d) ([ \w]+) bags?").unwrap();
    for rule in buffer.split("\n").filter(|s| s.len() > 0) {
        match re_name.captures(rule) {
            Some(c1) => {
                for c2 in re_contains.captures_iter(rule) {
                    with_bag
                        .entry(c2[2].to_string())
                        .or_default()
                        .push((c2[1].parse::<u32>().unwrap(), c1[1].to_string()));
                    bags_in
                        .entry(c1[1].to_string())
                        .or_default()
                        .push((c2[1].parse::<u32>().unwrap(), c2[2].to_string()));
                }
            }
            None => println!("Could not find bag name"),
        }
    }

    let mut part1_sol = 0;
    let mut searched = BTreeSet::<&String>::new();
    let mut search_stack = Vec::<&String>::from_iter(
        with_bag
            .get("shiny gold")
            .unwrap()
            .iter()
            .map(|(_n, bag)| bag),
    );

    while search_stack.len() > 0 {
        part1_sol += 1;
        let bag = search_stack.pop().unwrap();
        match with_bag.get(bag) {
            Some(iter) => {
                for (_n, b) in iter {
                    if !searched.contains(b) && !search_stack.contains(&b) {
                        search_stack.push(b)
                    }
                }
            }
            None => (),
        }
        searched.insert(bag);
    }

    println!("Part 1: {:?}", part1_sol);

    let mut part2_sol = 0;
    let mut ammount_and_bags = bags_in.get("shiny gold").unwrap().clone();
    while ammount_and_bags.len() > 0 {
        let (n, b) = ammount_and_bags.pop().unwrap();
        part2_sol += n;
        ammount_and_bags.extend(
            bags_in
                .entry(b)
                .or_default()
                .iter()
                .map(|(n_in, b_in)| (n * n_in, b_in.to_string())),
        );
    }

    println!("Part 2: {:?}", part2_sol);
}
