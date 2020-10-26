/*
    --- Day 9: Sensor Boost ---
    You've just said goodbye to the rebooted rover and left Mars when you receive a faint distress signal coming from the asteroid belt. It must be the Ceres monitoring station!

    In order to lock on to the signal, you'll need to boost your sensors. The Elves send up the latest BOOST program - Basic Operation Of System Test.

    While BOOST (your puzzle input) is capable of boosting your sensors, for tenuous safety reasons, it refuses to do so until the computer it runs on passes some checks to demonstrate it is a complete Intcode computer.

    Your existing Intcode computer is missing one key feature: it needs support for parameters in relative mode.

    Parameters in mode 2, relative mode, behave very similarly to parameters in position mode: the parameter is interpreted as a position. Like position mode, parameters in relative mode can be read from or written to.

    The important difference is that relative mode parameters don't count from address 0. Instead, they count from a value called the relative base. The relative base starts at 0.

    The address a relative mode parameter refers to is itself plus the current relative base. When the relative base is 0, relative mode parameters and position mode parameters with the same value refer to the same address.

    For example, given a relative base of 50, a relative mode parameter of -7 refers to memory address 50 + -7 = 43.

    The relative base is modified with the relative base offset instruction:

    Opcode 9 adjusts the relative base by the value of its only parameter. The relative base increases (or decreases, if the value is negative) by the value of the parameter.
    For example, if the relative base is 2000, then after the instruction 109,19, the relative base would be 2019. If the next instruction were 204,-34, then the value at address 1985 would be output.

    Your Intcode computer will also need a few other capabilities:

    The computer's available memory should be much larger than the initial program. Memory beyond the initial program starts with the value 0 and can be read or written like any other memory. (It is invalid to try to access memory at a negative address, though.)
    The computer should have support for large numbers. Some instructions near the beginning of the BOOST program will verify this capability.
    Here are some example programs that use these features:

    109,1,204,-1,1001,100,1,100,1008,100,16,101,1006,101,0,99 takes no input and produces a copy of itself as output.
    1102,34915192,34915192,7,4,7,99,0 should output a 16-digit number.
    104,1125899906842624,99 should output the large number in the middle.
    The BOOST program will ask for a single input; run it in test mode by providing it the value 1. It will perform a series of checks on each opcode, output any opcodes (and the associated parameter modes) that seem to be functioning incorrectly, and finally output a BOOST keycode.

    Once your Intcode computer is fully functional, the BOOST program should report no malfunctioning opcodes when run in test mode; it should only output a single value, the BOOST keycode. What BOOST keycode does it produce?
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
    input_idx: usize,
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
            input_idx: 0,
            output: Vec::new(),
        }
    }

    fn run(&mut self) {
        while self.halted == false {
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
                _  => panic!("Invalid opcode"),
            }
        }
    }

    fn run_with_pause(&mut self) {
        self.running = true;
        while self.running == true {
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
                _  => panic!("Invalid opcode"),
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
        (modes % 10i64.pow(digit)) / 10i64.pow(digit - 1)
    }

    fn get_param_addr(&self, param_idx: u32) -> usize {
        let mode = self.get_mode_curr(param_idx);
        match mode {
            0 => self.get_value(self.pc + param_idx as usize) as usize,
            1 => self.pc + param_idx as usize,
            2 => (self.relative_base_offset + self.get_value(self.pc + param_idx as usize)) as usize,
            _ => panic!(),
        }
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
        self.pc += 2;

        let input = self.input[self.input_idx];
        self.input_idx += 1;

        // println!("In {} => [{}]", input, param1_addr);

        self.set_value(param1_addr, input);
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

#[aoc(day9, part1)]
pub fn solve(input: &str) -> i64 {
    let code: Vec<i64> = input
                            .trim()
                            .split(',')
                            .map(|s| s.parse::<i64>().unwrap())
                            .collect();
    let mut program = Program::new(&code, &[1]);
    program.run();
    println!("BOOST keycode: {}", program.output[0]);
    program.output[0]
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
