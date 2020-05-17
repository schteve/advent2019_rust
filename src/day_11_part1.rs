/*
    --- Day 11: Space Police ---
    On the way to Jupiter, you're pulled over by the Space Police.

    "Attention, unmarked spacecraft! You are in violation of Space Law! All spacecraft must have a clearly visible registration identifier! You have 24 hours to comply or be sent to Space Jail!"

    Not wanting to be sent to Space Jail, you radio back to the Elves on Earth for help. Although it takes almost three hours for their reply signal to reach you, they send instructions for how to power up the emergency hull painting robot and even provide a small Intcode program (your puzzle input) that will cause it to paint your ship appropriately.

    There's just one problem: you don't have an emergency hull painting robot.

    You'll need to build a new emergency hull painting robot. The robot needs to be able to move around on the grid of square panels on the side of your ship, detect the color of its current panel, and paint its current panel black or white. (All of the panels are currently black.)

    The Intcode program will serve as the brain of the robot. The program uses input instructions to access the robot's camera: provide 0 if the robot is over a black panel or 1 if the robot is over a white panel. Then, the program will output two values:

    First, it will output a value indicating the color to paint the panel the robot is over: 0 means to paint the panel black, and 1 means to paint the panel white.
    Second, it will output a value indicating the direction the robot should turn: 0 means it should turn left 90 degrees, and 1 means it should turn right 90 degrees.
    After the robot turns, it should always move forward exactly one panel. The robot starts facing up.

    The robot will continue running for a while like this and halt when it is finished drawing. Do not restart the Intcode computer inside the robot during this process.

    For example, suppose the robot is about to start running. Drawing black panels as ., white panels as #, and the robot pointing the direction it is facing (< ^ > v), the initial state and region near the robot looks like this:

    .....
    .....
    ..^..
    .....
    .....
    The panel under the robot (not visible here because a ^ is shown instead) is also black, and so any input instructions at this point should be provided 0. Suppose the robot eventually outputs 1 (paint white) and then 0 (turn left). After taking these actions and moving forward one panel, the region now looks like this:

    .....
    .....
    .<#..
    .....
    .....
    Input instructions should still be provided 0. Next, the robot might output 0 (paint black) and then 0 (turn left):

    .....
    .....
    ..#..
    .v...
    .....
    After more outputs (1,0, 1,0):

    .....
    .....
    ..^..
    .##..
    .....
    The robot is now back where it started, but because it is now on a white panel, input instructions should be provided 1. After several more outputs (0,1, 1,0, 1,0), the area looks like this:

    .....
    ..<#.
    ...#.
    .##..
    .....
    Before you deploy the robot, you should probably have an estimate of the area it will cover: specifically, you need to know the number of panels it paints at least once, regardless of color. In the example above, the robot painted 6 panels at least once. (It painted its starting panel twice, but that panel is still only counted once; it also never painted the panel it ended on.)

    Build a new emergency hull painting robot and run the Intcode program on it. How many panels does it paint at least once?
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
        // println!("Program: {:?}", program);
        // println!("PC: {}", pc);
        // println!();

        let opcode = self.get_opcode_curr();
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
            _ => panic!(),
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
            _ => panic!(),
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
            _ => panic!(),
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

enum Color {
    Black,
    White,
}

impl Color {
    fn from_value(value: i64) -> Color {
        match value {
            0 => Color::Black,
            1 => Color::White,
            _ => panic!(),
        }
    }

    fn value(&self) -> i64 {
        match *self {
            Color::Black => 0,
            Color::White => 1,
        }
    }

    fn char(&self) -> char {
        match *self {
            Color::Black => '.',
            Color::White => '#',
        }
    }
}

enum AbsDirection {
    North,
    South,
    East,
    West,
}

enum RelDirection {
    Left,
    Right,
}

impl RelDirection {
    fn from_value(value: i64) -> RelDirection {
        match value {
            0 => RelDirection::Left,
            1 => RelDirection::Right,
            _ => panic!(),
        }
    }

    fn value(&self) -> i64 {
        match *self {
            RelDirection::Left => 0,
            RelDirection::Right => 1,
        }
    }
}

struct Robot {
    panels: HashMap<(i64, i64), Color>,
    abs_direction: AbsDirection,
    location: (i64, i64),
}

impl Robot {
    fn new() -> Robot {
        Robot {
            panels: HashMap::new(),
            abs_direction: AbsDirection::North,
            location: (0, 0),
        }
    }

    fn turn(&mut self, rel_direction: RelDirection) {
        match self.abs_direction {
            AbsDirection::North => {
                match rel_direction {
                    RelDirection::Left => self.abs_direction = AbsDirection::West,
                    RelDirection::Right => self.abs_direction = AbsDirection::East,
                }
            }
            AbsDirection::South => {
                match rel_direction {
                    RelDirection::Left => self.abs_direction = AbsDirection::East,
                    RelDirection::Right => self.abs_direction = AbsDirection::West,
                }
            }
            AbsDirection::East => {
                match rel_direction {
                    RelDirection::Left => self.abs_direction = AbsDirection::North,
                    RelDirection::Right => self.abs_direction = AbsDirection::South,
                }
            }
            AbsDirection::West => {
                match rel_direction {
                    RelDirection::Left => self.abs_direction = AbsDirection::South,
                    RelDirection::Right => self.abs_direction = AbsDirection::North,
                }
            }
        }
    }

    fn move_forward(&mut self, spaces: i64) {
        match self.abs_direction {
            AbsDirection::North => self.location = (self.location.0,          self.location.1 - spaces),
            AbsDirection::South => self.location = (self.location.0,          self.location.1 + spaces),
            AbsDirection::East  => self.location = (self.location.0 + spaces, self.location.1),
            AbsDirection::West  => self.location = (self.location.0 - spaces, self.location.1),
        }
    }

    fn set_color(&mut self, color: Color) {
        self.panels.insert(self.location, color);
    }

    fn get_color_curr(&self) -> Color {
        self.get_color(self.location)
    }

    fn get_color(&self, location: (i64, i64)) -> Color {
        let option = self.panels.get(&location);
        match option {
            Some(color) => Color::from_value(color.value()), // HashMap.get() returns a reference; we need to copy or reconstruct the value
            None        => Color::Black,
        }
    }

    fn display(&self) {
        let mut x_range = (0, 0);
        let mut y_range = (0, 0);

        for k in self.panels.keys() {
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
                    match self.abs_direction {
                        AbsDirection::North => print!("{}", '^'),
                        AbsDirection::South => print!("{}", 'v'),
                        AbsDirection::East  => print!("{}", '>'),
                        AbsDirection::West  => print!("{}", '<'),
                        _ => panic!(),
                    }
                } else {
                    print!("{}", self.get_color((x, y)).char());
                }
            }
            println!("");
        }
    }
}

fn run_program_with_robot(program: &mut Program, robot: &mut Robot) {
    let mut count = 0u32;

    while program.halted == false {
        program.run_with_pause();

        if program.input_needed == true {
            let current_color = robot.get_color_curr();
            program.input.push(current_color.value() as i64);
        } else if program.output.len() >= 2 {
            let color_value = program.output.remove(0);
            let color = Color::from_value(color_value);
            robot.set_color(color);

            let rel_dir_value = program.output.remove(0);
            let rel_dir = RelDirection::from_value(rel_dir_value);
            robot.turn(rel_dir);
            robot.move_forward(1);
        }
    }
}

#[aoc(day11, part1)]
pub fn solve(input: &str) -> usize {
    let code: Vec<i64> = input
                            .trim()
                            .split(",")
                            .map(|s| s.parse::<i64>().unwrap())
                            .collect();
    let mut program = Program::new(&code, &[]);

    let mut robot = Robot::new();
    /*robot.panels.insert((-10, -5), Color::White);
    robot.panels.insert((0, 5), Color::White);
    robot.panels.insert((7, 3), Color::White);
    robot.display();*/

    run_program_with_robot(&mut program, &mut robot);
    robot.display();
    println!("Painted panels: {}", robot.panels.len());
    robot.panels.len()
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
