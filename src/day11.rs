use std::cmp::min;
use std::fs;
use std::str::FromStr;

#[derive(PartialEq, Eq, Clone)]
enum PositionState {
    Empty,
    Occupied,
    Floor,
}

impl FromStr for PositionState {

    type Err = ();

    fn from_str(input: &str) -> Result<PositionState, Self::Err> {
        match input {
            "L"  => Ok(PositionState::Empty),
            "#"  => Ok(PositionState::Occupied),
            "."  => Ok(PositionState::Floor),
            _    => {println!("{:?}", input); Err(())},
        }
    }
}

type Seats = Vec<Vec<PositionState>>;


fn evolve_seats(seats_in: &Seats) -> Seats {
    let mut result = seats_in.clone().to_vec();
    for (i, row) in seats_in.iter().enumerate() {
        for (j, position) in row.iter().enumerate(){
            let count_occupied = neighbours_in_state(i, j, seats_in, PositionState::Occupied);
            match position {
                &PositionState::Empty => if count_occupied == 0 { result[i][j] = PositionState::Occupied },
                &PositionState::Occupied => if count_occupied >= 4 { result[i][j] = PositionState::Empty },
                &PositionState::Floor => (),
            }
        }
    }
    result
}

fn neighbours_in_state(i: usize, j: usize, seats: &Seats, state: PositionState) -> usize {
    let mut result: usize = 0;
    for ii in i.saturating_sub(1)..min(i+2, seats.len()) {
        for jj in j.saturating_sub(1)..min(j+2, seats[i].len()) {
            if !(ii == i && jj == j) {
                result += (seats[ii][jj] == state) as usize;
            }
        }
    }
    result
}

fn evolve_seats2(seats_in: &Seats) -> Seats {
    let mut result = seats_in.clone().to_vec();
    for (i, row) in seats_in.iter().enumerate() {
        for (j, position) in row.iter().enumerate(){
            let count_occupied = neighbours_in_state2(i, j, seats_in, PositionState::Occupied);
            match position {
                &PositionState::Empty => if count_occupied == 0 { result[i][j] = PositionState::Occupied },
                &PositionState::Occupied => if count_occupied >= 5 { result[i][j] = PositionState::Empty },
                &PositionState::Floor => (),
            }
        }
    }
    result
}

fn neighbours_in_state2(i: usize, j: usize, seats: &Seats, state: PositionState) -> usize {
    let mut result: usize = 0;
    for dir_i in -1i32..=1i32 {
        for dir_j in -1i32..=1i32 {
            if !(dir_i == 0 && dir_j == 0) {
                let mut t = 1;
                loop {
                    let ii = i as i32 + dir_i * t;
                    let jj = j as i32 + dir_j * t;
                    if ii < 0 || ii >= seats.len() as i32 {break;} ;
                    if jj < 0 || jj >= seats.len() as i32 {break;} ;
                    match seats[ii as usize][jj as usize] {
                       PositionState::Floor => t += 1,
                       _ => {result += (seats[ii as usize][jj as usize] == state) as usize; break;}
                    }
                }
            }
        }
    }
    result
}

fn seats_equal (seats1: &Seats, seats2: &Seats) -> bool {
    seats1.iter().
    zip(seats2.iter()).
    all(|(r1, r2)| {
        r1.iter().zip(r2.iter()).all(|(p1, p2)| p1 == p2)
    })
}

fn seats_occupied(seats: &Seats) -> u32 {
    seats
    .iter()
    .map(|row| row.iter().fold(0u32, |acc, p| acc + (p == &PositionState::Occupied) as u32))
    .sum()
}

#[allow(dead_code)]
pub fn day11(){
    let input_contents = fs::read_to_string("inputs/day11").unwrap();
    let seats = input_contents
        .split('\n')
        .filter(|x| x.len() > 0)
        .into_iter()
        .map(|x| {
            x.chars()
            .map(|x| PositionState::from_str(&x.to_string()).unwrap())
            .collect::<Vec<PositionState>>()
        })
        .collect::<Seats>();
    let mut p1_seats = seats.clone();
    loop {
       let new_seats = evolve_seats(&p1_seats);
       if seats_equal(&p1_seats, &new_seats) {
        break;
       } else {
        p1_seats = new_seats;
       }
    }
    let part1_sol = seats_occupied(&p1_seats);
    println!("Part 1: {:?}", part1_sol);
    let mut p2_seats = seats.clone();
    loop {
       let new_seats = evolve_seats2(&p2_seats);
       if seats_equal(&p2_seats, &new_seats) {
        break;
       } else {
        p2_seats = new_seats;
       }
    }
    let part2_sol = seats_occupied(&p2_seats);
    println!("Part 2: {:?}", part2_sol);

}