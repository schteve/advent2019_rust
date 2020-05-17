/*
    --- Part Two ---
    You quickly repair the oxygen system; oxygen gradually fills the area.

    Oxygen starts in the location containing the repaired oxygen system. It takes one minute for oxygen to spread to all open locations that are adjacent to a location that already contains oxygen. Diagonal locations are not adjacent.

    In the example above, suppose you've used the droid to explore the area fully and have the following map (where locations that currently contain oxygen are marked O):

     ##
    #..##
    #.#..#
    #.O.#
     ###
    Initially, the only location which contains oxygen is the location of the repaired oxygen system. However, after one minute, the oxygen spreads to all open (.) locations that are adjacent to a location containing oxygen:

     ##
    #..##
    #.#..#
    #OOO#
     ###
    After a total of two minutes, the map looks like this:

     ##
    #..##
    #O#O.#
    #OOO#
     ###
    After a total of three minutes:

     ##
    #O.##
    #O#OO#
    #OOO#
     ###
    And finally, the whole region is full of oxygen after a total of four minutes:

     ##
    #OO##
    #O#OO#
    #OOO#
     ###
    So, in this example, all locations contain oxygen after 4 minutes.

    Use the repair droid to get a complete map of the area. How many minutes will it take to fill with oxygen?
*/

use std::collections::HashMap;

struct Program {
    code: Vec<i64>,
    mem: HashMap<usize, i64>,
    pc: usize,
    running: bool, // Should run or pause
    halted: bool, // Hit a halt instruction; completely done.
    relative_base_offset: i64,

    input: Vec<i64>,
    input_needed: bool,

    output: Vec<i64>,
}

impl Program {
    fn new(code: &[i64], input: &[i64]) -> Program {
        Program {
            code: code.to_vec(),
            mem: HashMap::new(),
            pc: 0,
            running: false,
            halted: false,
            relative_base_offset: 0,
            input: input.to_vec(),
            input_needed: false,
            output: Vec::new(),
        }
    }

    fn run(&mut self) {
        while self.halted == false {
            self.execute_next_opcode();
        }
    }

    fn run_with_pause(&mut self) {
        self.running = true;
        while self.running == true {
            self.execute_next_opcode();
        }
    }

    fn execute_next_opcode(&mut self) {
        // println!("Code: {:?}", self.code);
        // println!("PC: {}", self.pc);
        // println!();

        let opcode = self.get_opcode_curr();
        // println!("Opcode: {}", opcode);
        match opcode {
            1  => self.opcode_add(),
            2  => self.opcode_mul(),
            3  => self.opcode_in(),
            4  => self.opcode_out(),
            5  => self.opcode_jmp(),
            6  => self.opcode_jmpn(),
            7  => self.opcode_lt(),
            8  => self.opcode_eq(),
            9  => self.opcode_rel(),
            99 => self.opcode_halt(),
            _  => {
                // println!("FAIL");
                self.running = false;
                self.halted = true;
            }
        }
    }

    fn get_opcode_curr(&self) -> i64 {
        Program::get_opcode(self.code[self.pc])
    }

    fn get_opcode(code_word: i64) -> i64 {
        code_word % 100
    }

    fn get_mode_curr(&self, param_idx: u32) -> i64 {
        Program::get_mode(self.code[self.pc], param_idx)
    }

    fn get_mode(code_word: i64, digit: u32) -> i64 {
        let modes = code_word / 100;
        let mode = (modes % 10i64.pow(digit)) / 10i64.pow(digit - 1);
        mode
    }

    fn get_param_addr(&self, param_idx: u32) -> usize {
        let mode = self.get_mode_curr(param_idx);
        let addr = match mode {
            0 => self.get_value(self.pc + param_idx as usize) as usize,
            1 => self.pc + param_idx as usize,
            2 => (self.relative_base_offset + self.get_value(self.pc + param_idx as usize)) as usize,
            _ => panic!("Invalid param address mode: {}", mode),
        };
        addr
    }

    fn get_value(&self, addr: usize) -> i64 {
        let code_len = self.code.len();
        let value = match addr {
            a if a < code_len => {
                self.code[addr]
            },
            a if a >= code_len => {
                match self.mem.get(&addr) {
                    Some(value) => *value,
                    None => 0i64,
                }
            }
            _ => panic!("Invalid address: {}", addr),
        };
        value
    }

    fn set_value(&mut self, addr: usize, value: i64) {
        let code_len = self.code.len();
        match addr {
            a if a < code_len => {
                self.code[addr] = value;
            },
            a if a >= code_len => {
                self.mem.insert(addr, value);
            },
            _ => panic!("Invalid address: {}", addr),
        }
    }

    // 1 + 2 => 3
    fn opcode_add(&mut self) {
        let param1_addr = self.get_param_addr(1);
        let param2_addr = self.get_param_addr(2);
        let param3_addr = self.get_param_addr(3); // Note: this is output so must ALWAYS be positional
        self.pc += 4;

        let param1 = self.get_value(param1_addr);
        let param2 = self.get_value(param2_addr);

        // println!("Add {} + {} => [{}]", param1, param2, param3_addr);

        self.set_value(param3_addr, param1 + param2);
    }

    // 1 * 2 => 3
    fn opcode_mul(&mut self) {
        let param1_addr = self.get_param_addr(1);
        let param2_addr = self.get_param_addr(2);
        let param3_addr = self.get_param_addr(3); // Note: this is output so must ALWAYS be positional
        self.pc += 4;

        let param1 = self.get_value(param1_addr);
        let param2 = self.get_value(param2_addr);

        // println!("Mul {} * {} => [{}]", param1, param2, param3_addr);

        self.set_value(param3_addr, param1 * param2);
    }

    // Get input and store in target
    fn opcode_in(&mut self) {
        let param1_addr = self.get_param_addr(1);

        if self.input.len() > 0 {
            let input = self.input.remove(0);
            self.input_needed = false;
            self.pc += 2;

            // println!("In {} => [{}]", input, param1_addr);

            self.set_value(param1_addr, input);
        } else {
            self.running = false;
            self.input_needed = true;
            // Don't increment PC so running again is not an error
            // println!("Input required!");
        }
    }

    // Get 1 and output it to user
    fn opcode_out(&mut self) {
        let param1_addr = self.get_param_addr(1);
        self.pc += 2;

        let param1 = self.get_value(param1_addr);

        self.output.push(param1);
        self.running = false; // Pause so output can be processed

        // println!("Out {}", param1);
    }

    // If 1 is non-zero, jump to 2
    fn opcode_jmp(&mut self) {
        let param1_addr = self.get_param_addr(1);
        let param2_addr = self.get_param_addr(2);

        let param1 = self.get_value(param1_addr);
        let param2 = self.get_value(param2_addr);

        if param1 != 0 {
            // println!("Jmp {} => PC", param2);
            self.pc = param2 as usize;
        } else {
            // println!("Jmp {}", param1);
            self.pc += 3;
        }
    }

    // If 1 is zero, jump to 2
    fn opcode_jmpn(&mut self) {
        let param1_addr = self.get_param_addr(1);
        let param2_addr = self.get_param_addr(2);

        let param1 = self.get_value(param1_addr);
        let param2 = self.get_value(param2_addr);

        if param1 == 0 {
            // println!("JmpN {} => PC", param2);
            self.pc = param2 as usize;
        } else {
            // println!("JmpN {}", param1);
            self.pc += 3;
        }
    }

    // If 1 < 2, #1 => 3, else #0 => 3
    fn opcode_lt(&mut self) {
        let param1_addr = self.get_param_addr(1);
        let param2_addr = self.get_param_addr(2);
        let param3_addr = self.get_param_addr(3); // Note: this is output so must ALWAYS be positional
        self.pc += 4;

        let param1 = self.get_value(param1_addr);
        let param2 = self.get_value(param2_addr);

        let mut value = 0;
        if param1 < param2 {
            value = 1;
        }

        // println!("LT {} < {}, {} => [{}]", param1, param2, value, param3_addr);

        self.set_value(param3_addr, value);
    }

    // If 1 == 2, #1 => 3, else #0 => 3
    fn opcode_eq(&mut self) {
        let param1_addr = self.get_param_addr(1);
        let param2_addr = self.get_param_addr(2);
        let param3_addr = self.get_param_addr(3); // Note: this is output so must ALWAYS be positional
        self.pc += 4;

        let param1 = self.get_value(param1_addr);
        let param2 = self.get_value(param2_addr);

        let mut value = 0;
        if param1 == param2 {
            value = 1;
        }

        // println!("EQ {} == {}, {} => [{}]", param1, param2, value, param3_addr);

        self.set_value(param3_addr, value);
    }

    fn opcode_rel(&mut self) {
        let param1_addr = self.get_param_addr(1);
        self.pc += 2;

        let param1 = self.get_value(param1_addr);

        // println!("Rel {}", param1);

        self.relative_base_offset += param1;
    }

    // Set halted flag and stop running
    fn opcode_halt(&mut self) {
        self.running = false;
        self.halted = true;
        // Don't increment PC -- this lets us re-run the program where we left off and it will just halt immediately!

        // println!("Program complete!");
    }
}

#[derive(Clone, Copy)]
enum Direction {
    North,
    South,
    West,
    East,
}

impl Direction {
    fn from_value(value: i64) -> Direction {
        match value {
            1 => Direction::North,
            2 => Direction::South,
            3 => Direction::West,
            4 => Direction::East,
            _ => panic!("Invalid Direction value {}", value),
        }
    }

    fn value(&self) -> i64 {
        match *self {
            Direction::North => 1,
            Direction::South => 2,
            Direction::West => 3,
            Direction::East => 4,
        }
    }

    fn to_string(&self) -> String {
        match *self {
            Direction::North => "North".to_string(),
            Direction::South => "South".to_string(),
            Direction::West => "West".to_string(),
            Direction::East => "East".to_string(),
        }
    }

    fn step_from(&self, from: (i32, i32)) -> (i32, i32) {
        match *self {
            Direction::North => (from.0, from.1 - 1),
            Direction::South => (from.0, from.1 + 1),
            Direction::West =>  (from.0 - 1, from.1),
            Direction::East =>  (from.0 + 1, from.1),
        }
    }

    fn undo(&self) -> Direction {
        match *self {
            Direction::North => Direction::South,
            Direction::South => Direction::North,
            Direction::West =>  Direction::East,
            Direction::East =>  Direction::West,
        }
    }
}

enum Status {
    Wall,
    Moved,
    Oxygen,
}

impl Status {
    fn from_value(value: i64) -> Status {
        match value {
            0 => Status::Wall,
            1 => Status::Moved,
            2 => Status::Oxygen,
            _ => panic!("Invalid Status value {}", value),
        }
    }

    fn value(&self) -> i64 {
        match *self {
            Status::Wall => 0,
            Status::Moved => 1,
            Status::Oxygen => 2,
        }
    }

    fn to_string(&self) -> String {
        match *self {
            Status::Wall => "Wall".to_string(),
            Status::Moved => "Moved".to_string(),
            Status::Oxygen => "Oxygen".to_string(),
        }
    }
}

#[derive(PartialEq)]
enum Space {
    Unknown,
    Empty,
    Wall,
    Oxygen,
}

impl Space {
    fn char(&self) -> char {
        match *self {
            Space::Unknown => '?',
            Space::Empty => ' ',
            Space::Wall => '#',
            Space::Oxygen => 'O',
        }
    }
}

struct Drone<'a> {
    controller: &'a mut Program,
    area: HashMap<(i32, i32), Space>,
    location: (i32, i32),
}

impl<'a> Drone<'a> {
    fn new(controller: &'a mut Program) -> Drone {
        let mut area = HashMap::new();
        area.insert((0, 0), Space::Empty);
        Drone {
            controller: controller,
            area: area,
            location: (0, 0),
        }
    }

    fn display_area(&self) {
        let mut x_range = (0, 0);
        let mut y_range = (0, 0);

        for k in self.area.keys() {
            if k.0 < x_range.0 {
                x_range = (k.0, x_range.1);
            } else if k.0 > x_range.1 {
                x_range = (x_range.0, k.0);
            }

            if k.1 < y_range.0 {
                y_range = (k.1, y_range.1);
            } else if k.1 > y_range.1 {
                y_range = (y_range.0, k.1);
            }
        }
        // println!("x_range: {:?}", x_range);
        // println!("y_range: {:?}", y_range);

        for y in (y_range.0)..(y_range.1 + 1) {
            for x in (x_range.0)..(x_range.1 + 1) {
                if self.location == (x, y) {
                    print!("D");
                } else if let Some(t) = self.area.get(&(x, y)) {
                    print!("{}", t.char());
                } else {
                    print!(".");
                }
            }
            println!("");
        }
        println!("");
        println!("");
    }

    fn movement(&mut self, direction: Direction) -> bool {
        self.controller.input.push(direction.value() as i64);
        self.controller.run_with_pause();

        if self.controller.output.len() > 0 {
            let result = self.controller.output.remove(0);
            let status = Status::from_value(result);
            // println!("Move {}: {}", direction.to_string(), status.to_string());

            match status {
                Status::Wall => {
                    let wall_position = direction.step_from(self.location);
                    self.area.insert(wall_position, Space::Wall);
                    return false;
                },
                Status::Moved => {
                    let drone_position = direction.step_from(self.location);
                    self.area.insert(drone_position, Space::Empty);
                    self.location = drone_position;
                    return true;
                },
                Status::Oxygen => {
                    let drone_position = direction.step_from(self.location);
                    self.area.insert(drone_position, Space::Oxygen);
                    self.location = drone_position;
                    return true;
                },
            }
        } else {
            println!("No movement: program halted!");
            return false;
        }
    }

    fn map_area(&mut self) {
        self.search();
    }

    fn search(&mut self) {
        // self.display_area();

        let candidates = vec![Direction::North,
                              Direction::South,
                              Direction::West,
                              Direction::East];
        for direction in candidates {
            let step_in_direction = direction.step_from(self.location);
            match self.area.get(&step_in_direction) {
                None => {
                    if self.movement(direction) == true {
                        self.search();
                        self.movement(direction.undo());
                    }
                },
                _ => (),
            }
        }
    }

    fn fill_with_oxygen(&mut self) -> i32 {
        if let Some(oxygen) = self.area.iter().find(|(k, v)| **v == Space::Oxygen) {
            let mut counter = 0;
            let mut frontier: Vec<(i32, i32)> = Vec::new();
            frontier.push(*oxygen.0);

            loop {
                let mut empties = Vec::new();
                for location in frontier.drain(..).collect::<Vec<(i32, i32)>>() {
                    let candidates = vec![Direction::North,
                                          Direction::South,
                                          Direction::West,
                                          Direction::East];
                    for direction in candidates {
                        let step_in_direction = direction.step_from(location);
                        match self.area.get(&step_in_direction) {
                            Some(Space::Empty) => {
                                empties.push(step_in_direction);
                            },
                            _ => (),
                        }
                    }
                }

                for e in empties {
                    self.area.insert(e, Space::Oxygen);
                    frontier.push(e);
                }

                if frontier.len() == 0 {
                    return counter;
                } else {
                    counter += 1;
                }

                // self.display_area();
            }
        }

        0
    }
}

#[aoc(day15, part2)]
pub fn solve(input: &str) -> i32 {
    let code: Vec<i64> = input
                            .trim()
                            .split(",")
                            .map(|s| s.parse::<i64>().unwrap())
                            .collect();
    let mut program = Program::new(&code, &[]);

    let mut drone = Drone::new(&mut program);
    drone.map_area();
    drone.display_area();

    // Flood the room with oxygen
    let minutes = drone.fill_with_oxygen();
    drone.display_area();

    println!("Minutes to fill room with oxygen: {}", minutes);
    minutes
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_program() {
        let code = [109,1,204,-1,1001,100,1,100,1008,100,16,101,1006,101,0,99];
        let mut program = Program::new(&code, &[]);
        program.run();
        assert_eq!(program.output, code);

        let code = [1102,34915192,34915192,7,4,7,99,0];
        let mut program = Program::new(&code, &[]);
        program.run();
        assert_eq!(program.output, [1219070632396864]);

        let code = [104,1125899906842624,99];
        let mut program = Program::new(&code, &[]);
        program.run();
        assert_eq!(program.output, [1125899906842624]);
    }
}
