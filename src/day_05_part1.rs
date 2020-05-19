/*
    --- Day 5: Sunny with a Chance of Asteroids ---
    You're starting to sweat as the ship makes its way toward Mercury. The Elves suggest that you get the air conditioner working by upgrading your ship computer to support the Thermal Environment Supervision Terminal.

    The Thermal Environment Supervision Terminal (TEST) starts by running a diagnostic program (your puzzle input). The TEST diagnostic program will run on your existing Intcode computer after a few modifications:

    First, you'll need to add two new instructions:

    Opcode 3 takes a single integer as input and saves it to the position given by its only parameter. For example, the instruction 3,50 would take an input value and store it at address 50.
    Opcode 4 outputs the value of its only parameter. For example, the instruction 4,50 would output the value at address 50.
    Programs that use these instructions will come with documentation that explains what should be connected to the input and output. The program 3,0,4,0,99 outputs whatever it gets as input, then halts.

    Second, you'll need to add support for parameter modes:

    Each parameter of an instruction is handled based on its parameter mode. Right now, your ship computer already understands parameter mode 0, position mode, which causes the parameter to be interpreted as a position - if the parameter is 50, its value is the value stored at address 50 in memory. Until now, all parameters have been in position mode.

    Now, your ship computer will also need to handle parameters in mode 1, immediate mode. In immediate mode, a parameter is interpreted as a value - if the parameter is 50, its value is simply 50.

    Parameter modes are stored in the same value as the instruction's opcode. The opcode is a two-digit number based only on the ones and tens digit of the value, that is, the opcode is the rightmost two digits of the first value in an instruction. Parameter modes are single digits, one per parameter, read right-to-left from the opcode: the first parameter's mode is in the hundreds digit, the second parameter's mode is in the thousands digit, the third parameter's mode is in the ten-thousands digit, and so on. Any missing modes are 0.

    For example, consider the program 1002,4,3,4,33.

    The first instruction, 1002,4,3,4, is a multiply instruction - the rightmost two digits of the first value, 02, indicate opcode 2, multiplication. Then, going right to left, the parameter modes are 0 (hundreds digit), 1 (thousands digit), and 0 (ten-thousands digit, not present and therefore zero):

    ABCDE
     1002

    DE - two-digit opcode,      02 == opcode 2
     C - mode of 1st parameter,  0 == position mode
     B - mode of 2nd parameter,  1 == immediate mode
     A - mode of 3rd parameter,  0 == position mode,
                                      omitted due to being a leading zero
    This instruction multiplies its first two parameters. The first parameter, 4 in position mode, works like it did before - its value is the value stored at address 4 (33). The second parameter, 3 in immediate mode, simply has value 3. The result of this operation, 33 * 3 = 99, is written according to the third parameter, 4 in position mode, which also works like it did before - 99 is written to address 4.

    Parameters that an instruction writes to will never be in immediate mode.

    Finally, some notes:

    It is important to remember that the instruction pointer should increase by the number of values in the instruction after the instruction finishes. Because of the new instructions, this amount is no longer always 4.
    Integers can be negative: 1101,100,-1,4,0 is a valid program (find 100 + -1, store the result in position 4).
    The TEST diagnostic program will start by requesting from the user the ID of the system to test by running an input instruction - provide it 1, the ID for the ship's air conditioner unit.

    It will then perform a series of diagnostic tests confirming that various parts of the Intcode computer, like parameter modes, function correctly. For each test, it will run an output instruction indicating how far the result of the test was from the expected value, where 0 means the test was successful. Non-zero outputs mean that a function is not working correctly; check the instructions that were run before the output instruction to see which one failed.

    Finally, the program will output a diagnostic code and immediately halt. This final output isn't an error; an output followed immediately by a halt means the program finished. If all outputs were zero except the diagnostic code, the diagnostic program ran successfully.

    After providing 1 to the only input instruction and passing all the tests, what diagnostic code does the program produce?
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
                99 => {
                    // println!("Program complete!");
                    break;
                }
                _  => panic!("Invalid opcode"),
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

        // println!("In {} => [{}]", input, param1_addr);

        self.set_value(param1_addr, input);
    }

    // Get 1 and output it to user
    fn opcode_out(&mut self) {
        let param1_addr = self.get_param_addr(1);
        self.pc += 2;

        let param1 = self.get_value(param1_addr);

        self.output.push(param1);

        // println!("Out {}", param1);
    }
}

#[aoc(day5, part1)]
pub fn solve(input: &str) -> i32 {
    let code: Vec<i32> = input.trim().split(",").map(|s| s.parse::<i32>().unwrap()).collect();
    let input = [1];

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
    }
}
