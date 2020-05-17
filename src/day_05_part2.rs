/*
    --- Part Two ---
    The air conditioner comes online! Its cold air feels good for a while, but then the TEST alarms start to go off. Since the air conditioner can't vent its heat anywhere but back into the spacecraft, it's actually making the air inside the ship warmer.

    Instead, you'll need to use the TEST to extend the thermal radiators. Fortunately, the diagnostic program (your puzzle input) is already equipped for this. Unfortunately, your Intcode computer is not.

    Your computer is only missing a few opcodes:

    Opcode 5 is jump-if-true: if the first parameter is non-zero, it sets the instruction pointer to the value from the second parameter. Otherwise, it does nothing.
    Opcode 6 is jump-if-false: if the first parameter is zero, it sets the instruction pointer to the value from the second parameter. Otherwise, it does nothing.
    Opcode 7 is less than: if the first parameter is less than the second parameter, it stores 1 in the position given by the third parameter. Otherwise, it stores 0.
    Opcode 8 is equals: if the first parameter is equal to the second parameter, it stores 1 in the position given by the third parameter. Otherwise, it stores 0.
    Like all instructions, these instructions need to support parameter modes as described above.

    Normally, after an instruction is finished, the instruction pointer increases by the number of values in that instruction. However, if the instruction modifies the instruction pointer, that value is used and the instruction pointer is not automatically increased.

    For example, here are several programs that take one input, compare it to the value 8, and then produce one output:

    3,9,8,9,10,9,4,9,99,-1,8 - Using position mode, consider whether the input is equal to 8; output 1 (if it is) or 0 (if it is not).
    3,9,7,9,10,9,4,9,99,-1,8 - Using position mode, consider whether the input is less than 8; output 1 (if it is) or 0 (if it is not).
    3,3,1108,-1,8,3,4,3,99 - Using immediate mode, consider whether the input is equal to 8; output 1 (if it is) or 0 (if it is not).
    3,3,1107,-1,8,3,4,3,99 - Using immediate mode, consider whether the input is less than 8; output 1 (if it is) or 0 (if it is not).
    Here are some jump tests that take an input, then output 0 if the input was zero or 1 if the input was non-zero:

    3,12,6,12,15,1,13,14,13,4,13,99,-1,0,1,9 (using position mode)
    3,3,1105,-1,9,1101,0,0,12,4,12,99,1 (using immediate mode)
    Here's a larger example:

    3,21,1008,21,8,20,1005,20,22,107,8,21,20,1006,20,31,
    1106,0,36,98,0,0,1002,21,125,20,4,20,1105,1,46,104,
    999,1105,1,46,1101,1000,1,20,4,20,1105,1,46,98,99
    The above example program uses an input instruction to ask for a single number. The program will then output 999 if the input value is below 8, output 1000 if the input value is equal to 8, or output 1001 if the input value is greater than 8.

    This time, when the TEST diagnostic program runs its input instruction to get the ID of the system to test, provide it 5, the ID for the ship's thermal radiator controller. This diagnostic test suite only outputs one number, the diagnostic code.

    What is the diagnostic code for system ID 5?
*/

struct Program {
    code: Vec<i32>,
    pc: u32,
    input: Vec<i32>,
    input_idx: u32,
    output: Vec<i32>,
}

impl Program {
    fn new(code: &[i32], input: &[i32]) -> Program {
        Program {
            code: code.to_vec(),
            pc: 0,
            input: input.to_vec(),
            input_idx: 0,
            output: Vec::new(),
        }
    }

    fn run(&mut self) {
        loop {
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
                99 => {
                    // println!("Program complete!");
                    break;
                }
                _  => {
                    // println!("FAIL");
                    break;
                }
            }
        }
    }

    fn get_opcode_curr(&self) -> i32 {
        Program::get_opcode(self.code[self.pc as usize])
    }

    fn get_opcode(code_word: i32) -> i32 {
        code_word % 100
    }

    fn get_mode_curr(&self, param_idx: u32) -> i32 {
        Program::get_mode(self.code[self.pc as usize], param_idx)
    }

    fn get_mode(code_word: i32, digit: u32) -> i32 {
        let modes = code_word / 100;
        let mode = (modes % 10i32.pow(digit)) / 10i32.pow(digit - 1);
        mode
    }

    fn get_param_addr(&self, param_idx: u32) -> u32 {
        let mode = self.get_mode_curr(param_idx);
        let addr = match mode {
            0 => self.get_value(self.pc + param_idx) as u32,
            1 => self.pc + param_idx,
            _ => panic!(),
        };
        addr
    }

    fn get_value(&self, addr: u32) -> i32 {
        self.code[addr as usize]
    }

    fn set_value(&mut self, addr: u32, value: i32) {
        self.code[addr as usize] = value;
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

        let input = self.input[self.input_idx as usize];
        self.input_idx += 1;

        println!("In {} => [{}]", input, param1_addr);

        self.set_value(param1_addr, input);
    }

    // Get 1 and output it to user
    fn opcode_out(&mut self) {
        let param1_addr = self.get_param_addr(1);
        self.pc += 2;

        let param1 = self.get_value(param1_addr);

        self.output.push(param1);

        println!("Out {}", param1);
    }

    // If 1 is non-zero, jump to 2
    fn opcode_jmp(&mut self) {
        let param1_addr = self.get_param_addr(1);
        let param2_addr = self.get_param_addr(2);

        let param1 = self.get_value(param1_addr);
        let param2 = self.get_value(param2_addr);

        if param1 != 0 {
            // println!("Jmp {} => PC", param2);
            self.pc = param2 as u32;
        } else {
            // println!("Jmp {}", param1);
            self.pc += 3
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
            self.pc = param2 as u32;
        } else {
            // println!("JmpN {}", param1);
            self.pc += 3
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
}

#[aoc(day5, part2)]
pub fn solve(input: &str) -> i32 {
    let code: Vec<i32> = input.split(",").map(|s| s.parse::<i32>().unwrap()).collect();
    let input = [5];

    let mut program = Program::new(&code, &input);
    program.run();

    let result = program.output.iter().cloned().max().unwrap();
    result
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_program() {
        // Add
        let mut program = Program::new(&[1,0,0,0,99], &[]);
        program.run();
        assert_eq!(program.code, [2,0,0,0,99]);
        assert_eq!(program.output, []);

        // Mul
        let mut program = Program::new(&[2,3,0,3,99], &[]);
        program.run();
        assert_eq!(program.code, [2,3,0,6,99]);
        assert_eq!(program.output, []);

        // Mul
        let mut program = Program::new(&[2,4,4,5,99,0], &[]);
        program.run();
        assert_eq!(program.code, [2,4,4,5,99,9801]);
        assert_eq!(program.output, []);

        // Add / Mul
        let mut program = Program::new(&[1,1,1,4,99,5,6,0,99], &[]);
        program.run();
        assert_eq!(program.code, [30,1,1,4,2,5,6,0,99]);
        assert_eq!(program.output, []);

        // Mode
        let mut program = Program::new(&[1002,4,3,4,33], &[]);
        program.run();
        assert_eq!(program.code, [1002,4,3,4,99]);
        assert_eq!(program.output, []);

        // Input / Output
        let mut program = Program::new(&[3,0,4,0,99], &[1]);
        program.run();
        assert_eq!(program.code, [1,0,4,0,99]);
        assert_eq!(program.output, [1]);

        // Negative
        let mut program = Program::new(&[1101,100,-1,4,0], &[]);
        program.run();
        assert_eq!(program.code, [1101,100,-1,4,99]);
        assert_eq!(program.output, []);

        // EQ, position mode
        let mut program = Program::new(&[3,9,8,9,10,9,4,9,99,-1,8], &[5]);
        program.run();
        assert_eq!(program.output, [0]);

        // EQ, position mode
        let mut program = Program::new(&[3,9,8,9,10,9,4,9,99,-1,8], &[8]);
        program.run();
        assert_eq!(program.output, [1]);

        // LT, position mode
        let mut program = Program::new(&[3,9,7,9,10,9,4,9,99,-1,8], &[5]);
        program.run();
        assert_eq!(program.output, [1]);

        // LT, position mode
        let mut program = Program::new(&[3,9,7,9,10,9,4,9,99,-1,8], &[8]);
        program.run();
        assert_eq!(program.output, [0]);

        // EQ, immediate mode
        let mut program = Program::new(&[3,3,1108,-1,8,3,4,3,99], &[5]);
        program.run();
        assert_eq!(program.output, [0]);

        // EQ, immediate mode
        let mut program = Program::new(&[3,3,1108,-1,8,3,4,3,99], &[8]);
        program.run();
        assert_eq!(program.output, [1]);

        // LT, immediate mode
        let mut program = Program::new(&[3,3,1107,-1,8,3,4,3,99], &[5]);
        program.run();
        assert_eq!(program.output, [1]);

        // LT, immediate mode
        let mut program = Program::new(&[3,3,1107,-1,8,3,4,3,99], &[8]);
        program.run();
        assert_eq!(program.output, [0]);

        // Jmp, position mode
        let mut program = Program::new(&[3,12,6,12,15,1,13,14,13,4,13,99,-1,0,1,9], &[0]);
        program.run();
        assert_eq!(program.output, [0]);

        // Jmp, position mode
        let mut program = Program::new(&[3,12,6,12,15,1,13,14,13,4,13,99,-1,0,1,9], &[1]);
        program.run();
        assert_eq!(program.output, [1]);

        // Jmp, immediate mode
        let mut program = Program::new(&[3,3,1105,-1,9,1101,0,0,12,4,12,99,1], &[0]);
        program.run();
        assert_eq!(program.output, [0]);

        // Jmp, immediate mode
        let mut program = Program::new(&[3,3,1105,-1,9,1101,0,0,12,4,12,99,1], &[1]);
        program.run();
        assert_eq!(program.output, [1]);

        // Everything
        let code = [3,21,1008,21,8,20,1005,20,22,107,8,21,20,1006,20,31,1106,0,36,98,0,0,1002,21,125,20,4,20,1105,1,46,104,999,1105,1,46,1101,1000,1,20,4,20,1105,1,46,98,99];
        let mut program = Program::new(&code, &[2]);
        program.run();
        assert_eq!(program.output, [999]);

        // Everything
        let code = [3,21,1008,21,8,20,1005,20,22,107,8,21,20,1006,20,31,1106,0,36,98,0,0,1002,21,125,20,4,20,1105,1,46,104,999,1105,1,46,1101,1000,1,20,4,20,1105,1,46,98,99];
        let mut program = Program::new(&code, &[8]);
        program.run();
        assert_eq!(program.output, [1000]);

        // Everything
        let code = [3,21,1008,21,8,20,1005,20,22,107,8,21,20,1006,20,31,1106,0,36,98,0,0,1002,21,125,20,4,20,1105,1,46,104,999,1105,1,46,1101,1000,1,20,4,20,1105,1,46,98,99];
        let mut program = Program::new(&code, &[10]);
        program.run();
        assert_eq!(program.output, [1001]);
    }
}
