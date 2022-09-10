use std::fs;


#[derive(PartialEq, Eq, Clone, Copy)]
enum Direction {
    North,
    South,
    East,
    West
}

enum TurnDirection {
    Left,
    Right,
}

#[derive(Clone)]
struct ShipState {
    direction: Direction,
    position_ns: i32,
    position_ew: i32
}

impl ShipState {
    pub fn apply_command(&self, command: &str) -> ShipState {
        let instruction = command.chars().nth(0).unwrap();
        let value = &command[1..].parse::<i32>().unwrap();
        match instruction {
            'N' => self.move_ship(Direction::North, value),
            'S' => self.move_ship(Direction::South, value),
            'E' => self.move_ship(Direction::East, value),
            'W' => self.move_ship(Direction::West, value),
            'F' => self.move_ship(self.direction, value),
            'L' => self.turn_ship(TurnDirection::Left, value),
            'R' => self.turn_ship(TurnDirection::Right, value),
            _ => panic!("Invalid instruction!")
        }
    }

    fn move_ship(&self, direction: Direction, value: &i32) -> ShipState {
        let mut result = self.clone();
        match direction {
            Direction::North => result.position_ns += value,
            Direction::South => result.position_ns -= value,
            Direction::East => result.position_ew += value,
            Direction::West => result.position_ew -= value,
        }
        result
    }

    fn turn_ship(&self, turn_direction: TurnDirection, value: &i32) -> ShipState {
        let directions = vec![Direction::East, Direction::North, Direction::West, Direction::South];
        let current_angle = directions.iter().position(|&x| x == self.direction).unwrap() * 90usize;
        let turn_angle = match turn_direction {
            TurnDirection::Left => *value as usize, 
            TurnDirection::Right => (360 - value) as usize, 
        };
        let new_direction = directions[(((current_angle + turn_angle)/90) % 4)];
        let mut result = self.clone();
        result.direction = new_direction;
        result
    }
    
}

#[allow(dead_code)]
pub fn day12() {
    let input_contents = fs::read_to_string("inputs/day12").unwrap();
    let instructions = input_contents
        .split('\n')
        .filter(|x| x.len() > 0)
        .into_iter();
    let mut ship = ShipState{direction: Direction::East, position_ns:0, position_ew:0};
    for command in instructions {
        ship = ship.apply_command(command);
    }
    let part1_sol = ship.position_ns.abs() + ship.position_ew.abs();
    println!("Part 1: {:?}", part1_sol);
}
