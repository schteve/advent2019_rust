/*
    --- Part Two ---
    It's no good - in this configuration, the amplifiers can't generate a large enough output signal to produce the thrust you'll need. The Elves quickly talk you through rewiring the amplifiers into a feedback loop:

          O-------O  O-------O  O-------O  O-------O  O-------O
    0 -+->| Amp A |->| Amp B |->| Amp C |->| Amp D |->| Amp E |-.
       |  O-------O  O-------O  O-------O  O-------O  O-------O |
       |                                                        |
       '--------------------------------------------------------+
                                                                |
                                                                v
                                                         (to thrusters)
    Most of the amplifiers are connected as they were before; amplifier A's output is connected to amplifier B's input, and so on. However, the output from amplifier E is now connected into amplifier A's input. This creates the feedback loop: the signal will be sent through the amplifiers many times.

    In feedback loop mode, the amplifiers need totally different phase settings: integers from 5 to 9, again each used exactly once. These settings will cause the Amplifier Controller Software to repeatedly take input and produce output many times before halting. Provide each amplifier its phase setting at its first input instruction; all further input/output instructions are for signals.

    Don't restart the Amplifier Controller Software on any amplifier during this process. Each one should continue receiving and sending signals until it halts.

    All signals sent or received in this process will be between pairs of amplifiers except the very first signal and the very last signal. To start the process, a 0 signal is sent to amplifier A's input exactly once.

    Eventually, the software on the amplifiers will halt after they have processed the final loop. When this happens, the last output signal from amplifier E is sent to the thrusters. Your job is to find the largest output signal that can be sent to the thrusters using the new phase settings and feedback loop arrangement.

    Here are some example programs:

    Max thruster signal 139629729 (from phase setting sequence 9,8,7,6,5):

    3,26,1001,26,-4,26,3,27,1002,27,2,27,1,27,26,
    27,4,27,1001,28,-1,28,1005,28,6,99,0,0,5
    Max thruster signal 18216 (from phase setting sequence 9,7,8,5,6):

    3,52,1001,52,-5,52,3,53,1,52,56,54,1007,54,5,55,1005,55,26,1001,54,
    -5,54,1105,1,12,1,53,54,53,1008,54,0,55,1001,55,1,55,2,53,55,53,4,
    53,1001,56,-1,56,1005,56,6,99,0,0,0,0,10
    Try every combination of the new phase settings on the amplifier feedback loop. What is the highest signal that can be sent to the thrusters?
*/

use std::cmp;

struct Program {
    code: Vec<i32>,
    pc: u32,
    running: bool, // Should run or pause
    halted: bool, // Hit a halt instruction; completely done.
    input: Vec<i32>,
    input_idx: u32,
    output: Vec<i32>,
}

impl Program {
    fn new(code: &[i32], input: &[i32]) -> Program {
        Program {
            code: code.to_vec(),
            pc: 0,
            running: false,
            halted: false,
            input: input.to_vec(),
            input_idx: 0,
            output: Vec::new(),
        }
    }

    fn run(&mut self) {
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
                99 => self.opcode_halt(),
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
        (modes % 10i32.pow(digit)) / 10i32.pow(digit - 1)
    }

    fn get_param_addr(&self, param_idx: u32) -> u32 {
        let mode = self.get_mode_curr(param_idx);
        match mode {
            0 => self.get_value(self.pc + param_idx) as u32,
            1 => self.pc + param_idx,
            _ => panic!(),
        }
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

    // Set halted flag and stop running
    fn opcode_halt(&mut self) {
        self.running = false;
        self.halted = true;
        // Don't increment PC -- this lets us re-run the program where we left off and it will just halt immediately!

        // println!("Program complete!");
    }
}

fn check_signal(code: &[i32], phase: &[i32]) -> i32 {
    let mut amp_programs: Vec<Program> = phase.iter()
                                                .map(|&i| Program::new(&code, &[i])) // Set initial input to phase settings
                                                .collect();
    let mut next_input = Some(0);
    let mut last_output = 0;
    while amp_programs.last().unwrap().halted == false {
        for p in &mut amp_programs {
            if let Some(i) = next_input {
                p.input.push(i);
            }

            p.run();

            next_input = p.output.pop();
            if let Some(i) = next_input {
                last_output = i;
            }
        }
    }

    last_output
}

fn generate_permutations(outputs: &mut Vec<Vec<i32>>, sequence: &mut [i32], seq_idx: usize) {
    if sequence.len() == seq_idx {
        outputs.push(sequence.to_vec());
    } else {
        for i in seq_idx..sequence.len() {
            sequence.swap(seq_idx, i);                                      // Swap elements
            generate_permutations(outputs, sequence, seq_idx + 1 as usize); // Descend
            sequence.swap(seq_idx, i);                                      // Undo swapping
        }
    }
}

fn max_thruster_signal(code: &[i32], phases: &[i32]) -> u32 {
    let mut phase_permutations: Vec<Vec<i32>> = Vec::new();
    let mut phase_options = phases.to_vec();
    generate_permutations(&mut phase_permutations, &mut phase_options, 0);

    let max_signal = phase_permutations
                        .iter()
                        .map(|p| check_signal(&code, &p))
                        .fold(0, cmp::max);
    max_signal as u32
}

#[aoc(day7, part2)]
pub fn solve(input: &str) -> u32 {
    let code: Vec<i32> = input
                            .trim()
                            .split(',')
                            .map(|s| s.parse::<i32>().unwrap())
                            .collect();

    let max_signal = max_thruster_signal(&code, &[5,6,7,8,9]);
    println!("Max signal: {}", max_signal);
    max_signal
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_max_thruster_signal() {
        let code = [3,26,1001,26,-4,26,3,27,1002,27,2,27,1,27,26,27,4,27,1001,28,-1,28,1005,28,6,99,0,0,5];
        assert_eq!(max_thruster_signal(&code, &[5,6,7,8,9]), 139629729);

        let code = [3,52,1001,52,-5,52,3,53,1,52,56,54,1007,54,5,55,1005,55,26,1001,54,-5,54,1105,1,12,1,53,54,53,1008,54,0,55,1001,55,1,55,2,53,55,53,4,53,1001,56,-1,56,1005,56,6,99,0,0,0,0,10];
        assert_eq!(max_thruster_signal(&code, &[5,6,7,8,9]), 18216);
    }

    #[test]
    fn test_check_signal() {
        let code = [3,26,1001,26,-4,26,3,27,1002,27,2,27,1,27,26,27,4,27,1001,28,-1,28,1005,28,6,99,0,0,5];
        assert_eq!(check_signal(&code, &[9,8,7,6,5]), 139629729);

        let code = [3,52,1001,52,-5,52,3,53,1,52,56,54,1007,54,5,55,1005,55,26,1001,54,-5,54,1105,1,12,1,53,54,53,1008,54,0,55,1001,55,1,55,2,53,55,53,4,53,1001,56,-1,56,1005,56,6,99,0,0,0,0,10];
        assert_eq!(check_signal(&code, &[9,7,8,5,6]), 18216);
    }
}
