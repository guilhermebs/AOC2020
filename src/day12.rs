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

struct ShipState {
    direction: Direction,
    position_ns: i32,
    position_ew: i32
}

impl ShipState {
    pub fn apply_command(&mut self, command: &str){
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

    fn move_ship(&mut self, direction: Direction, value: &i32) {
        match direction {
            Direction::North => self.position_ns += value,
            Direction::South => self.position_ns -= value,
            Direction::East => self.position_ew += value,
            Direction::West => self.position_ew -= value,
        }
    }

    fn turn_ship(&mut self, turn_direction: TurnDirection, value: &i32) {
        let directions = vec![Direction::East, Direction::North, Direction::West, Direction::South];
        let current_angle = directions.iter().position(|&x| x == self.direction).unwrap() * 90usize;
        let turn_angle = match turn_direction {
            TurnDirection::Left => *value as usize, 
            TurnDirection::Right => (360 - value) as usize, 
        };
        self.direction = directions[(((current_angle + turn_angle)/90) % 4)];
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
        ship.apply_command(command);
    }
    let part1_sol = ship.position_ns.abs() + ship.position_ew.abs();
    println!("Part 1: {:?}", part1_sol);
}
