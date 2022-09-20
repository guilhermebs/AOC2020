use regex::Regex;
use std::fs;

#[allow(dead_code)]
#[derive(Copy, Clone, Debug)]
struct Interval {
    lb: usize,
    ub: usize,
}

impl Interval {
    pub fn is_in(self: &Interval, nr: &usize) -> bool {
        return self.lb <= *nr && *nr <= self.ub;
    }
}

#[allow(dead_code)]
#[derive(Debug)]
struct TicketField {
    name: String,
    intervals: [Interval; 2],
}

impl TicketField {
    pub fn from_string(ln: &str) -> TicketField {
        let re = Regex::new(r"([\w\s]+): (\d+)-(\d+) or (\d+)-(\d+)$").unwrap();
        match re.captures(ln) {
            Some(c) => TicketField {
                name: c[1].to_string(),
                intervals: [
                    Interval {
                        lb: c[2].parse::<usize>().unwrap(),
                        ub: c[3].parse::<usize>().unwrap(),
                    },
                    Interval {
                        lb: c[4].parse::<usize>().unwrap(),
                        ub: c[5].parse::<usize>().unwrap(),
                    },
                ],
            },

            None => panic!("invalid string: {:?}", ln),
        }
    }

    pub fn evaluate(self: &TicketField, nr: &usize) -> bool {
        self.intervals.iter().any(|interval| interval.is_in(nr))
    }
}

#[allow(dead_code)]
pub fn day16() {
    let file_contents = fs::read_to_string("inputs/day16").unwrap();
    let mut blocks = file_contents.split("\n\n");

    let rules_str = blocks.next().unwrap();
    let my_ticket_str = blocks.next().unwrap();
    let nearby_tickets_str = blocks.next().unwrap();

    let rules = rules_str
        .split("\n")
        .map(|ln| TicketField::from_string(ln))
        .collect::<Vec<TicketField>>();

    let mut nearby_tickets = nearby_tickets_str.split("\n");
    nearby_tickets.next();
    let mut tickets: Vec<Vec<usize>> = vec![];
    let mut part1_sol = 0;
    for ticket in nearby_tickets.filter(|t| t.len() > 0) {
        let ticket = ticket
            .split(",")
            .map(|x| x.parse::<usize>().unwrap())
            .collect::<Vec<usize>>();
        let total_invalid = ticket
            .iter()
            .filter(|x| !rules.iter().any(|r| r.evaluate(&x)))
            .sum();
        match total_invalid {
            0 => tickets.push(ticket),
            _ => part1_sol += total_invalid,
        }
    }

    println!("Part 1: {:?}", part1_sol);

    let my_ticket = my_ticket_str
        .split("\n")
        .last()
        .unwrap()
        .split(",")
        .map(|x| x.parse::<usize>().unwrap())
        .collect::<Vec<usize>>();
    tickets.push(my_ticket);

}
