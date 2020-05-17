/*
    --- Part Two ---
    There are many areas the springdroid can't reach. You flip through the manual and discover a way to increase its sensor range.

    Instead of ending your springcode program with WALK, use RUN. Doing this will enable extended sensor mode, capable of sensing ground up to nine tiles away. This data is available in five new read-only registers:

    Register E indicates whether there is ground five tiles away.
    Register F indicates whether there is ground six tiles away.
    Register G indicates whether there is ground seven tiles away.
    Register H indicates whether there is ground eight tiles away.
    Register I indicates whether there is ground nine tiles away.
    All other functions remain the same.

    Successfully survey the rest of the hull by ending your program with RUN. What amount of hull damage does the springdroid now report?
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

struct Script {
    lines: Vec<String>,
}

impl Script {
    fn new() -> Self {
        Self {
            lines: Vec::new(),
        }
    }

    fn add_line(&mut self, line: &str) {
        println!("{}", line);
        self.lines.push(line.to_owned());
    }
}

struct Droid<'a> {
    program: &'a mut Program,
    script: Script,
}

impl<'a> Droid<'a> {
    fn new(program: &'a mut Program) -> Self {
        Self {
            program: program,
            script: Script::new(),
        }
    }

    fn give_springscript(&mut self) {
        for line in &self.script.lines {
            for c in line.bytes() {
                self.program.input.push(c as i64);
            }
            self.program.input.push(0x0A as i64); // Always end with newline
        }
    }

    fn print_output(&mut self) -> Option<i64> {
        for i in self.program.output.drain(..) {
            if i < 128 { // If it's ASCII, print it as a character
                print!("{}", (i as u8) as char);
            } else { // If it's not ASCII, this is the final program result and can be returned immediately
                return Some(i);
            }
        }
        None
    }

    fn run(&mut self) -> i64 {
        // Get intial prompt
        self.program.run_with_pause();
        self.print_output();

        // Give script to program
        self.give_springscript();

        // Run the script
        while self.program.halted == false {
            self.program.run_with_pause();
            if let Some(result) = self.print_output() {
                return result;
            }

            if self.program.input_needed == true {
                println!("Input needed!"); // Shouldn't happen
            }
        }

        panic!("Program halted without completing");
    }
}

#[aoc(day21, part2)]
pub fn solve(input: &str) -> i64 {
    let code: Vec<i64> = input
                            .trim()
                            .split(",")
                            .map(|s| s.parse::<i64>().unwrap())
                            .collect();
    let mut program = Program::new(&code, &[]);
    let mut droid = Droid::new(&mut program);

    // Check if I need to jump
    droid.script.add_line("NOT A J"); // J: ~A
    droid.script.add_line("NOT B T"); // T: ~B
    droid.script.add_line("OR T J");  // J: ~A | ~B
    droid.script.add_line("NOT C T"); // T: ~C
    droid.script.add_line("OR T J");  // J: ~A | ~B | ~C
    // Check if I can jump
    droid.script.add_line("AND D J"); // J: (~A | ~B | ~C) & D
    // Check if I would need to jump again immediately, and if so can I
    droid.script.add_line("NOT E T"); // T: ~E
    droid.script.add_line("NOT T T"); // T: E
    droid.script.add_line("OR H T"); // T: E | H
    droid.script.add_line("AND T J"); // J: (~A | ~B | ~C) & D & (E | H)
    droid.script.add_line("RUN");

    let damage = droid.run();
    println!("Hull damage: {}", damage);
    damage
}

#[cfg(test)]
mod test {
    #[allow(unused_imports)] // No tests for this module
    use super::*;

    #[test]
    fn test_() {

    }
}
