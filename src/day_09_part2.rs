/*
    --- Part Two ---
    You now have a complete Intcode computer.

    Finally, you can lock on to the Ceres distress signal! You just need to boost your sensors using the BOOST program.

    The program runs in sensor boost mode by providing the input instruction the value 2. Once run, it will boost the sensors automatically, but it might take a few seconds to complete the operation on slower hardware. In sensor boost mode, the program will output a single value: the coordinates of the distress signal.

    Run the BOOST program in sensor boost mode. What are the coordinates of the distress signal?
*/

use std::collections::HashMap;

struct Program {
    code: Vec<i64>,
    mem: HashMap<usize, i64>,
    pc: usize,
    running: bool, // Should run or pause
    halted: bool,  // Hit a halt instruction; completely done.
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
                1 => self.opcode_add(),
                2 => self.opcode_mul(),
                3 => self.opcode_in(),
                4 => self.opcode_out(),
                5 => self.opcode_jmp(),
                6 => self.opcode_jmpn(),
                7 => self.opcode_lt(),
                8 => self.opcode_eq(),
                9 => self.opcode_rel(),
                99 => self.opcode_halt(),
                _ => panic!("Invalid opcode"),
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
                1 => self.opcode_add(),
                2 => self.opcode_mul(),
                3 => self.opcode_in(),
                4 => self.opcode_out(),
                5 => self.opcode_jmp(),
                6 => self.opcode_jmpn(),
                7 => self.opcode_lt(),
                8 => self.opcode_eq(),
                9 => self.opcode_rel(),
                99 => self.opcode_halt(),
                _ => panic!("Invalid opcode"),
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
            2 => {
                (self.relative_base_offset + self.get_value(self.pc + param_idx as usize)) as usize
            }
            _ => panic!(),
        }
    }

    fn get_value(&self, addr: usize) -> i64 {
        let code_len = self.code.len();
        let value = match addr {
            a if a < code_len => self.code[addr],
            a if a >= code_len => match self.mem.get(&addr) {
                Some(value) => *value,
                None => 0i64,
            },
            _ => panic!(),
        };
        value
    }

    fn set_value(&mut self, addr: usize, value: i64) {
        let code_len = self.code.len();
        match addr {
            a if a < code_len => {
                self.code[addr] = value;
            }
            a if a >= code_len => {
                self.mem.insert(addr, value);
            }
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

#[aoc(day9, part2)]
pub fn solve(input: &str) -> i64 {
    let code: Vec<i64> = input
        .trim()
        .split(',')
        .map(|s| s.parse::<i64>().unwrap())
        .collect();
    let mut program = Program::new(&code, &[2]);
    program.run();
    println!("Coordinates: {}", program.output[0]);
    program.output[0]
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_program() {
        let code = [
            109, 1, 204, -1, 1001, 100, 1, 100, 1008, 100, 16, 101, 1006, 101, 0, 99,
        ];
        let mut program = Program::new(&code, &[]);
        program.run();
        assert_eq!(program.output, code);

        let code = [1102, 34915192, 34915192, 7, 4, 7, 99, 0];
        let mut program = Program::new(&code, &[]);
        program.run();
        assert_eq!(program.output, [1219070632396864]);

        let code = [104, 1125899906842624, 99];
        let mut program = Program::new(&code, &[]);
        program.run();
        assert_eq!(program.output, [1125899906842624]);
    }
}
