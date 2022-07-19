use std::fs;
use regex::Regex;

use std::collections::HashMap;


lazy_static! {
    static ref RE_HGT: Regex = Regex::new(r"(\d+)(cm|in)$").unwrap();
    static ref RE_HCL: Regex = Regex::new(r"^#[a-z0-9]{6}$").unwrap();
    static ref RE_ECL: Regex = Regex::new(r"^(amb|blu|brn|gry|grn|hzl|oth)$").unwrap();
    static ref RE_PID: Regex = Regex::new(r"^[0-9]{9}$").unwrap();
}


#[allow(dead_code)]
pub fn day04() {
    let buffer = fs::read_to_string("inputs/day04").unwrap();
    let mut passport_info = Vec::<HashMap<String, String>>::new();
    let re = Regex::new(r"(\S+):(\S+)").unwrap();
    for entry in buffer.split("\n\n") {
        let mut pass = HashMap::<String, String>::new();
        for c in re.captures_iter(entry){
            pass.insert(String::from(&c[1]), String::from(&c[2]));
        }
        passport_info.push(pass);
    }
    println!("Number of passports: {}", passport_info.len());

    let part1_sol = passport_info.iter()
            .filter(|&p| is_valid(p))
            .count();
    println!("part1 {:?}", part1_sol);
    let part2_sol = passport_info.iter()
            .filter(|&p| is_valid2(p))
            .count();
    println!("part1 {:?}", part2_sol);


}

fn is_valid(pass: &HashMap<String, String>) -> bool {
    if pass.len() == 8 { return true };
    if (pass.len() == 7) & !pass.contains_key(&String::from("cid")) { return true };
    return false
}

fn is_valid2(pass: &HashMap<String, String>) -> bool {
    let mut valid = true;
    valid &= match pass.get(&String::from("byr")) {
        Some(byr) => {
            let y = byr.parse::<u32>().unwrap();
            y>=1920 && y<=2002
        },
        None => false
    };
    valid &= match pass.get(&String::from("iyr")) {
        Some(iyr) => {
            let y = iyr.parse::<u32>().unwrap();
            y>=2010 && y<=2020
        },
        None => false
    };
    valid &= match pass.get(&String::from("eyr")) {
        Some(eyr) => {
            let y = eyr.parse::<u32>().unwrap();
            y>=2020 && y<=2030
        },
        None => false
    };
    valid &= match pass.get(&String::from("hgt")) {
        Some(hgt) => {
            match RE_HGT.captures(hgt.as_str()) {
                Some(c) => {
                    let val = c[1].parse::<u32>().unwrap();
                    (&c[2] == "cm" && val >= 150 && val <= 193) ||
                    (&c[2] == "in" && val >= 59 && val <= 76)
                }
                None => false
            }
        },
        None => false
    };
    valid &= match pass.get(&String::from("hcl")) {
       Some(hcl) => RE_HCL.is_match(hcl.as_str()),
       None => false
    };

    valid &= match pass.get(&String::from("ecl")) {
       Some(ecl) => RE_ECL.is_match(ecl.as_str()),
       None => false
    };

    valid &= match pass.get(&String::from("pid")) {
       Some(pid) => RE_PID.is_match(pid.as_str()),
       None => false
    };
    return valid
}