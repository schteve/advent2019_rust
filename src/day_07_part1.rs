/*
    --- Day 7: Amplification Circuit ---
    Based on the navigational maps, you're going to need to send more power to your ship's thrusters to reach Santa in time. To do this, you'll need to configure a series of amplifiers already installed on the ship.

    There are five amplifiers connected in series; each one receives an input signal and produces an output signal. They are connected such that the first amplifier's output leads to the second amplifier's input, the second amplifier's output leads to the third amplifier's input, and so on. The first amplifier's input value is 0, and the last amplifier's output leads to your ship's thrusters.

        O-------O  O-------O  O-------O  O-------O  O-------O
    0 ->| Amp A |->| Amp B |->| Amp C |->| Amp D |->| Amp E |-> (to thrusters)
        O-------O  O-------O  O-------O  O-------O  O-------O
    The Elves have sent you some Amplifier Controller Software (your puzzle input), a program that should run on your existing Intcode computer. Each amplifier will need to run a copy of the program.

    When a copy of the program starts running on an amplifier, it will first use an input instruction to ask the amplifier for its current phase setting (an integer from 0 to 4). Each phase setting is used exactly once, but the Elves can't remember which amplifier needs which phase setting.

    The program will then call another input instruction to get the amplifier's input signal, compute the correct output signal, and supply it back to the amplifier with an output instruction. (If the amplifier has not yet received an input signal, it waits until one arrives.)

    Your job is to find the largest output signal that can be sent to the thrusters by trying every possible combination of phase settings on the amplifiers. Make sure that memory is not shared or reused between copies of the program.

    For example, suppose you want to try the phase setting sequence 3,1,2,4,0, which would mean setting amplifier A to phase setting 3, amplifier B to setting 1, C to 2, D to 4, and E to 0. Then, you could determine the output signal that gets sent from amplifier E to the thrusters with the following steps:

    Start the copy of the amplifier controller software that will run on amplifier A. At its first input instruction, provide it the amplifier's phase setting, 3. At its second input instruction, provide it the input signal, 0. After some calculations, it will use an output instruction to indicate the amplifier's output signal.
    Start the software for amplifier B. Provide it the phase setting (1) and then whatever output signal was produced from amplifier A. It will then produce a new output signal destined for amplifier C.
    Start the software for amplifier C, provide the phase setting (2) and the value from amplifier B, then collect its output signal.
    Run amplifier D's software, provide the phase setting (4) and input value, and collect its output signal.
    Run amplifier E's software, provide the phase setting (0) and input value, and collect its output signal.
    The final output signal from amplifier E would be sent to the thrusters. However, this phase setting sequence may not have been the best one; another sequence might have sent a higher signal to the thrusters.

    Here are some example programs:

    Max thruster signal 43210 (from phase setting sequence 4,3,2,1,0):

    3,15,3,16,1002,16,10,16,1,16,15,15,4,15,99,0,0
    Max thruster signal 54321 (from phase setting sequence 0,1,2,3,4):

    3,23,3,24,1002,24,10,24,1002,23,-1,23,
    101,5,23,23,1,24,23,23,4,23,99,0,0
    Max thruster signal 65210 (from phase setting sequence 1,0,4,3,2):

    3,31,3,32,1002,32,10,32,1001,31,-2,31,1007,31,0,33,
    1002,33,7,33,1,33,31,31,1,32,31,31,4,31,99,0,0,0
    Try every combination of phase settings on the amplifiers. What is the highest signal that can be sent to the thrusters?
*/

use std::cmp;
use std::fs;

struct Program {
    code: Vec<i32>,
    pc: u32,
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
            halted: false,
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
                99 => self.opcode_halt(),
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

    // Set halted flag
    fn opcode_halt(&mut self) {
        self.halted = true;
        // Don't increment PC -- this lets us re-run the program where we left off and it will just halt immediately!

        // println!("Program complete!");
    }
}

fn check_signal(code: &[i32], phase: &[i32]) -> i32 {
    let mut amp_programs = Vec::new();
    for &i in phase {
        amp_programs.push(Program::new(&code, &[i])); // Set initial input to phase settings
    }

    let mut next_input = Some(0);
    let mut last_output = 0;
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
                        .fold(0, |max, signal| cmp::max(max, signal));
    max_signal as u32
}

pub fn solve() {
    let input = fs::read_to_string("src/day_07_input.txt")
                    .expect("Something went wrong reading the file");
    let code: Vec<i32> = input
                            .trim()
                            .split(",")
                            .map(|s| s.parse::<i32>().unwrap())
                            .collect();

    let max_signal = max_thruster_signal(&code, &[0,1,2,3,4]);
    println!("Max signal: {}", max_signal);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_check_signal() {
        let code = [3,15,3,16,1002,16,10,16,1,16,15,15,4,15,99,0,0];
        assert_eq!(max_thruster_signal(&code, &[4,3,2,1,0]), 43210);

        let code = [3,23,3,24,1002,24,10,24,1002,23,-1,23,101,5,23,23,1,24,23,23,4,23,99,0,0];
        assert_eq!(max_thruster_signal(&code, &[0,1,2,3,4]), 54321);

        let code = [3,31,3,32,1002,32,10,32,1001,31,-2,31,1007,31,0,33,1002,33,7,33,1,33,31,31,1,32,31,31,4,31,99,0,0,0];
        assert_eq!(max_thruster_signal(&code, &[1,0,4,3,2]), 65210);
    }

    #[test]
    fn test_max_thruster_signal() {
        let code = [3,15,3,16,1002,16,10,16,1,16,15,15,4,15,99,0,0];
        assert_eq!(max_thruster_signal(&code, &[0,1,2,3,4]), 43210);

        let code = [3,23,3,24,1002,24,10,24,1002,23,-1,23,101,5,23,23,1,24,23,23,4,23,99,0,0];
        assert_eq!(max_thruster_signal(&code, &[0,1,2,3,4]), 54321);

        let code = [3,31,3,32,1002,32,10,32,1001,31,-2,31,1007,31,0,33,1002,33,7,33,1,33,31,31,1,32,31,31,4,31,99,0,0,0];
        assert_eq!(max_thruster_signal(&code, &[0,1,2,3,4]), 65210);
    }
}
