use std::{fs, collections::HashSet};

#[derive(Debug)]
struct SeatRange {
    row_from: u32,
    row_to: u32,
    col_from: u32,
    col_to: u32,
}

impl SeatRange {
   fn appliy_f(&self) -> SeatRange {
        let middle = (self.row_from + self.row_to) / 2;
        SeatRange {
            row_from: self.row_from,
            row_to:middle,
            col_from: self.col_from,
            col_to: self.col_to
        }
   } 

   fn apply_b(&self) -> SeatRange {
        let middle = (self.row_from + self.row_to) / 2;
        SeatRange {
            row_from: middle + 1,
            row_to:self.row_to,
            col_from: self.col_from,
            col_to: self.col_to
        }
   } 

   fn apply_l(&self) -> SeatRange {
        let middle = (self.col_from + self.col_to) / 2;
        SeatRange {
            row_from: self.row_from,
            row_to: self.row_to,
            col_from: self.col_from,
            col_to: middle
        }
   } 

   fn apply_r(&self) -> SeatRange {
        let middle = (self.col_from + self.col_to) / 2;
        SeatRange {
            row_from: self.row_from,
            row_to: self.row_to,
            col_from: middle + 1,
            col_to: self.col_to
        }
   } 

   fn unique_seat(&self) -> Option<(u32, u32)> {
        if (self.row_from == self.row_to) && (self.col_from == self.col_to) {
            return Some((self.row_from, self.col_from));
        }
        None
   }
}

fn get_seat_position(seat: &String) -> Result<(u32, u32), String> {
    let mut range = SeatRange{
        row_from: 0,
        row_to: 127,
        col_from: 0,
        col_to: 7
    };
    for s in seat.chars() {
        match s {
           'F' => range = range.appliy_f(),
           'B' => range = range.apply_b(),
           'R' => range = range.apply_r(),
           'L' => range = range.apply_l(),
           _ => return Err(String::from("Invalid instruction"))
        }
    };
    match range.unique_seat() {
       Some((col, row)) => return Ok((col, row)),
       None => Err(String::from("Could not find seat"))
    }

}

fn get_seat_id(seat: &String) -> Result<u32, String> {
    let position = get_seat_position(seat)?;
    Ok(position.0 * 8 + position.1)
}

#[allow(dead_code)]
pub fn day05() {
    let buffer = fs::read_to_string("inputs/day05").unwrap();
    let all_ids: HashSet<u32> = HashSet::from_iter({
            buffer
            .split("\n").filter(|&s| s.len() > 0)
            .map(|s| get_seat_id(&s.to_string()).unwrap())
    });
    let part1_sol = all_ids.iter().max().unwrap();
    println!("part 1: {}", part1_sol);

    let part2_sol = (8..126*8)
            .filter(|id| !all_ids.contains(id) &&
                                all_ids.contains(&(id+1))&&
                                all_ids.contains(&(id-1)))
            .collect::<Vec<u32>>();
    println!("part 2: {:?}", part2_sol);
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_seat() {
        let seat =  String::from("FBFBBFFRLR");
        assert_eq!((44, 5), get_seat_position(&seat).unwrap());
        assert_eq!(357, get_seat_id(&seat).unwrap());
    }

}