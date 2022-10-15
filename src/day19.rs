use std::{
    collections::{hash_map::RandomState, HashMap},
    fs,
};

use itertools::Itertools;
use regex::Regex;

fn create_expression(rules_str: &HashMap<usize, &str>, i: usize) -> String {
    let rule = rules_str[&i]
        .replace("\"", "")
        .split(" ")
        .map(|s| match s.parse::<usize>() {
            Ok(j) => create_expression(rules_str, j),
            Err(_) => s.to_owned(),
        })
        .join("");
    return "(?:".to_owned() + &rule + ")";
}

#[allow(dead_code)]
pub fn day19() {
    let file_contents = fs::read_to_string("inputs/day19").unwrap();
    let rule_re = Regex::new(r"^(\d+): (.*)$").unwrap();
    let rules_str =
        HashMap::<usize, &str, RandomState>::from_iter(file_contents.split("\n").filter_map(|s| {
            match rule_re.captures(s) {
                Some(c) => Some((
                    c.get(1).unwrap().as_str().parse::<usize>().unwrap(),
                    c.get(2).unwrap().as_str(),
                )),
                None => None,
            }
        }));
    let rule0_regex = Regex::new(&("^".to_owned() + &create_expression(&rules_str, 0) + "$")).unwrap();
    let part1_sol = file_contents
        .split("\n")
        .filter_map(|s| match rule0_regex.is_match(s) {
            true => Some(s),
            false => None,
        })
        .count();
    println!("{:?}", part1_sol);
}
