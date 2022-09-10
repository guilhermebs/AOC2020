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

struct ShipState2 {
    position_ns: i32,
    position_ew: i32,
    w_position_ns: i32,
    w_position_ew: i32

}

impl ShipState2 {
    pub fn apply_command(&mut self, command: &str){
        let instruction = command.chars().nth(0).unwrap();
        let value = &command[1..].parse::<i32>().unwrap();
        match instruction {
            'N' => self.move_waypoint(Direction::North, value),
            'S' => self.move_waypoint(Direction::South, value),
            'E' => self.move_waypoint(Direction::East, value),
            'W' => self.move_waypoint(Direction::West, value),
            'F' => self.move_ship(value),
            'L' => self.turn_waypoint(TurnDirection::Left, value),
            'R' => self.turn_waypoint(TurnDirection::Right, value),
            _ => panic!("Invalid instruction!")
        }
    }

    fn move_waypoint(&mut self, direction: Direction, value: &i32) {
        match direction {
            Direction::North => self.w_position_ns += value,
            Direction::South => self.w_position_ns -= value,
            Direction::East => self.w_position_ew += value,
            Direction::West => self.w_position_ew -= value,
        }
    }

    fn move_ship(&mut self, value: &i32) {
        self.position_ns += value * self.w_position_ns;
        self.position_ew += value * self.w_position_ew;
    }

    fn turn_waypoint(&mut self, turn_direction: TurnDirection, value: &i32) {
        let turn_angle = match turn_direction {
            TurnDirection::Left => *value, 
            TurnDirection::Right => 360 - value
        };
        match turn_angle % 360 {
            0 => (),
            90 => (self.w_position_ns, self.w_position_ew) = (self.w_position_ew, -self.w_position_ns),
            180 => (self.w_position_ns, self.w_position_ew) = (-self.w_position_ns, -self.w_position_ew),
            270 => (self.w_position_ns, self.w_position_ew) = (-self.w_position_ew, self.w_position_ns),
            _ => panic!("Invalid value")
        };
    }
    
}

#[allow(dead_code)]
pub fn day12() {
    let input_contents = fs::read_to_string("inputs/day12").unwrap();
    let instructions = input_contents
        .split('\n')
        .filter(|x| x.len() > 0)
        .collect::<Vec<&str>>();
    let mut ship = ShipState{direction: Direction::East, position_ns:0, position_ew:0};
    for command in instructions.iter() {
        ship.apply_command(command);
    }
    let part1_sol = ship.position_ns.abs() + ship.position_ew.abs();
    println!("Part 1: {:?}", part1_sol);

    let mut ship2 = ShipState2{position_ns:0, position_ew:0, w_position_ns:1, w_position_ew:10};
    for command in instructions.iter() {
        ship2.apply_command(command);
    }
    let part2_sol = ship2.position_ns.abs() + ship2.position_ew.abs();
    println!("Part 2: {:?}", part2_sol);
}
