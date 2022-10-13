use std::fs;

fn evaluate_expression<'a>(expression: &mut impl Iterator<Item = &'a char>) -> u64 {
    let radix = 10u32;
    let mut result = match expression.next() {
        Some(&'(') => evaluate_expression(expression),
        Some(&c) => c.to_digit(radix).unwrap() as u64,
        None => panic!("unrecognized value")
    };
    loop {
        let operation = expression.next();
        match operation {
            Some(&')') => return result,
            Some(_) => (),
            None => return result,
        }

        let value_right = match expression.next() {
            Some(&'(') => evaluate_expression(expression),
            Some(&c) => c.to_digit(radix).unwrap() as u64,
            None => 0,
        };
        match operation {
            Some(&'+') => result += value_right,
            Some(&'*') => result *= value_right,
            Some(_) => panic!("unrecognized value"),
            None => return result,
        }
    }
}

#[allow(dead_code)]
pub fn day18() {
    let file_contents = fs::read_to_string("inputs/day18").unwrap();
    let part1_sol: u64 = file_contents
        .replace(" ", "")
        .split("\n")
        .map(|x| evaluate_expression(&mut x.chars().collect::<Vec<_>>().iter()))
        .sum();

    println!("Part 1: {:?}", part1_sol);

    let part2_sol: u64 = ("(".to_owned() + &file_contents + ")")
        .replace(" ", "")
        .replace("(", "((")
        .replace(")", "))")
        .replace("*", ")*(")
        .replace("\n", ")\n(")
        .split("\n")
        .map(|x| evaluate_expression(&mut x.chars().collect::<Vec<_>>().iter()))
        .sum();

    println!("Part 2: {:?}", part2_sol);

}
