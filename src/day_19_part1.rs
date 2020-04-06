/*
    --- Day 19: Tractor Beam ---
    Unsure of the state of Santa's ship, you borrowed the tractor beam technology from Triton. Time to test it out.

    When you're safely away from anything else, you activate the tractor beam, but nothing happens. It's hard to tell whether it's working if there's nothing to use it on. Fortunately, your ship's drone system can be configured to deploy a drone to specific coordinates and then check whether it's being pulled. There's even an Intcode program (your puzzle input) that gives you access to the drone system.

    The program uses two input instructions to request the X and Y position to which the drone should be deployed. Negative numbers are invalid and will confuse the drone; all numbers should be zero or positive.

    Then, the program will output whether the drone is stationary (0) or being pulled by something (1). For example, the coordinate X=0, Y=0 is directly in front of the tractor beam emitter, so the drone control program will always report 1 at that location.

    To better understand the tractor beam, it is important to get a good picture of the beam itself. For example, suppose you scan the 10x10 grid of points closest to the emitter:

           X
      0->      9
     0#.........
     |.#........
     v..##......
      ...###....
      ....###...
    Y .....####.
      ......####
      ......####
      .......###
     9........##
    In this example, the number of points affected by the tractor beam in the 10x10 area closest to the emitter is 27.

    However, you'll need to scan a larger area to understand the shape of the beam. How many points are affected by the tractor beam in the 50x50 area closest to the emitter? (For each of X and Y, this will be 0 through 49.)
*/

use std::collections::HashMap;
use std::fs;

#[derive(Clone)]
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

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Space {
    Stationary,
    Pulled,
}

impl Space {
    fn from_value(value: i64) -> Self {
        match value {
            0 => Self::Stationary,
            1 => Self::Pulled,
            _ => panic!("Invalid value: {}", value),
        }
    }

    fn char(&self) -> char {
        match *self {
            Self::Stationary => '.',
            Self::Pulled => '#',
        }
    }
}

struct TractorBeam<'a> {
    program: &'a mut Program,
    area: HashMap<Point, Space>
}

impl<'a> TractorBeam<'a> {
    fn new(program: &'a mut Program) -> TractorBeam {
        TractorBeam {
            program: program,
            area: HashMap::new(),
        }
    }

    fn check_point(&mut self, point: Point) -> Space {
        if let Some(&space) = self.area.get(&point) {
            // The value is already cached
            return space;
        } else {
            // We need to ask the oracle what the value is
            let mut oracle_program = self.program.clone(); // This is needed because the program only runs once and then exits

            oracle_program.input.push(point.x as i64);
            oracle_program.input.push(point.y as i64);
            oracle_program.run_with_pause();

            if oracle_program.output.len() > 0 {
                let result = oracle_program.output.remove(0);
                let space = Space::from_value(result);

                // Cache the value for later
                self.area.insert(point, space);

                return space;
            } else {
                panic!("Something went wrong when trying a point");
            }
        }
    }

    fn scan(&mut self, x_range: u16, y_range: u16) {
        for x in 0..x_range {
            for y in 0..y_range {
                let p = Point {
                    x: x as i32,
                    y: y as i32,
                };
                self.check_point(p);
            }
        }
    }

    fn display(&self) {
        let mut x_range = (0, 0);
        let mut y_range = (0, 0);

        for k in self.area.keys() {
            if k.x < x_range.0 {
                x_range = (k.x, x_range.1);
            } else if k.x > x_range.1 {
                x_range = (x_range.0, k.x);
            }

            if k.y < y_range.0 {
                y_range = (k.y, y_range.1);
            } else if k.y > y_range.1 {
                y_range = (y_range.0, k.y);
            }
        }
        // println!("x_range: {:?}", x_range);
        // println!("y_range: {:?}", y_range);

        for y in y_range.0 ..= y_range.1 {
            for x in x_range.0 ..= x_range.1 {
                if let Some(t) = self.area.get(&Point { x, y }) {
                    print!("{}", t.char());
                } else {
                    print!(" ");
                }
            }
            println!("");
        }
        println!("");
        println!("");
    }

    fn count_points_in_beam(&self) -> u32 {
        let count = self.area.iter()
                            .filter(|&(k, v)| *v == Space::Pulled)
                            .count();
        count as u32
    }
}

pub fn solve() {
    let input = fs::read_to_string("src/day_19_input.txt")
                    .expect("Something went wrong reading the file");
    let code: Vec<i64> = input
                            .trim()
                            .split(",")
                            .map(|s| s.parse::<i64>().unwrap())
                            .collect();
    let mut program = Program::new(&code, &[]);
    let mut tractor_beam = TractorBeam::new(&mut program);

    tractor_beam.scan(50, 50);
    tractor_beam.display();

    let num_points = tractor_beam.count_points_in_beam();
    println!("Points in beam: {}", num_points);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_scan_0_0() {
        let input = fs::read_to_string("src/day_19_input.txt")
                    .expect("Something went wrong reading the file");
        let code: Vec<i64> = input
                                .trim()
                                .split(",")
                                .map(|s| s.parse::<i64>().unwrap())
                                .collect();
        let mut program = Program::new(&code, &[]);
        let mut tractor_beam = TractorBeam::new(&mut program);
        let space = tractor_beam.check_point(Point { x: 0, y: 0 });
        assert_eq!(space, Space::Pulled);
    }

    #[test]
    fn test_count_points_in_beam() {
        let mut program = Program::new(&[], &[]);
        let mut tractor_beam = TractorBeam::new(&mut program);

        for x in 0..10 {
            for y in 0..10 {
                tractor_beam.area.insert(Point { x, y }, Space::Stationary);
            }
        }

        tractor_beam.area.insert(Point { x: 0, y: 0 }, Space::Pulled);
        tractor_beam.area.insert(Point { x: 1, y: 1 }, Space::Pulled);
        tractor_beam.area.insert(Point { x: 2, y: 2 }, Space::Pulled);
        tractor_beam.area.insert(Point { x: 3, y: 2 }, Space::Pulled);
        tractor_beam.area.insert(Point { x: 3, y: 3 }, Space::Pulled);
        tractor_beam.area.insert(Point { x: 4, y: 3 }, Space::Pulled);
        tractor_beam.area.insert(Point { x: 5, y: 3 }, Space::Pulled);
        tractor_beam.area.insert(Point { x: 4, y: 4 }, Space::Pulled);
        tractor_beam.area.insert(Point { x: 5, y: 4 }, Space::Pulled);
        tractor_beam.area.insert(Point { x: 6, y: 4 }, Space::Pulled);
        tractor_beam.area.insert(Point { x: 5, y: 5 }, Space::Pulled);
        tractor_beam.area.insert(Point { x: 6, y: 5 }, Space::Pulled);
        tractor_beam.area.insert(Point { x: 7, y: 5 }, Space::Pulled);
        tractor_beam.area.insert(Point { x: 8, y: 5 }, Space::Pulled);
        tractor_beam.area.insert(Point { x: 6, y: 6 }, Space::Pulled);
        tractor_beam.area.insert(Point { x: 7, y: 6 }, Space::Pulled);
        tractor_beam.area.insert(Point { x: 8, y: 6 }, Space::Pulled);
        tractor_beam.area.insert(Point { x: 9, y: 6 }, Space::Pulled);
        tractor_beam.area.insert(Point { x: 6, y: 7 }, Space::Pulled);
        tractor_beam.area.insert(Point { x: 7, y: 7 }, Space::Pulled);
        tractor_beam.area.insert(Point { x: 8, y: 7 }, Space::Pulled);
        tractor_beam.area.insert(Point { x: 9, y: 7 }, Space::Pulled);
        tractor_beam.area.insert(Point { x: 7, y: 8 }, Space::Pulled);
        tractor_beam.area.insert(Point { x: 8, y: 8 }, Space::Pulled);
        tractor_beam.area.insert(Point { x: 9, y: 8 }, Space::Pulled);
        tractor_beam.area.insert(Point { x: 8, y: 9 }, Space::Pulled);
        tractor_beam.area.insert(Point { x: 9, y: 9 }, Space::Pulled);

        assert_eq!(tractor_beam.count_points_in_beam(), 27);
    }
}
