/*
    --- Part Two ---
    "Good, the new computer seems to be working correctly! Keep it nearby during this mission - you'll probably use it again. Real Intcode computers support many more features than your new one, but we'll let you know what they are as you need them."

    "However, your current priority should be to complete your gravity assist around the Moon. For this mission to succeed, we should settle on some terminology for the parts you've already built."

    Intcode programs are given as a list of integers; these values are used as the initial state for the computer's memory. When you run an Intcode program, make sure to start by initializing memory to the program's values. A position in memory is called an address (for example, the first value in memory is at "address 0").

    Opcodes (like 1, 2, or 99) mark the beginning of an instruction. The values used immediately after an opcode, if any, are called the instruction's parameters. For example, in the instruction 1,2,3,4, 1 is the opcode; 2, 3, and 4 are the parameters. The instruction 99 contains only an opcode and has no parameters.

    The address of the current instruction is called the instruction pointer; it starts at 0. After an instruction finishes, the instruction pointer increases by the number of values in the instruction; until you add more instructions to the computer, this is always 4 (1 opcode + 3 parameters) for the add and multiply instructions. (The halt instruction would increase the instruction pointer by 1, but it halts the program instead.)

    "With terminology out of the way, we're ready to proceed. To complete the gravity assist, you need to determine what pair of inputs produces the output 19690720."

    The inputs should still be provided to the program by replacing the values at addresses 1 and 2, just like before. In this program, the value placed in address 1 is called the noun, and the value placed in address 2 is called the verb. Each of the two input values will be between 0 and 99, inclusive.

    Once the program has halted, its output is available at address 0, also just like before. Each time you try a pair of inputs, make sure you first reset the computer's memory to the values in the program (your puzzle input) - in other words, don't reuse memory from a previous attempt.

    Find the input noun and verb that cause the program to produce the output 19690720. What is 100 * noun + verb? (For example, if noun=12 and verb=2, the answer would be 1202.)
*/

// use std::process::Command;

fn run_program(program: &mut [u32]) {
    let mut pc = 0u32;
    loop {
        // println!("Program: {:?}", program);
        // println!("PC: {}", pc);
        // println!();

        let opcode = program[pc as usize];
        match opcode {
            1  => opcode_add(program, &mut pc),
            2  => opcode_mul(program, &mut pc),
            99 => {
                // println!("Program complete!");
                break;
            }
            _  => panic!("Invalid opcode"),
        }
    }
}

fn opcode_add(program: &mut [u32], pc: &mut u32) {
    let param1_addr = program[(*pc + 1) as usize];
    let param2_addr = program[(*pc + 2) as usize];
    let target_addr = program[(*pc + 3) as usize];
    *pc += 4;

    let param1 = program[param1_addr as usize];
    let param2 = program[param2_addr as usize];

    // println!("Add {} + {} => {}", param1, param2, target_addr);

    program[target_addr as usize] = param1 + param2;
}

fn opcode_mul(program: &mut [u32], pc: &mut u32)  {
    let param1_addr = program[(*pc + 1) as usize];
    let param2_addr = program[(*pc + 2) as usize];
    let target_addr = program[(*pc + 3) as usize];
    *pc += 4;

    let param1 = program[param1_addr as usize];
    let param2 = program[param2_addr as usize];

    // println!("Add {} * {} => {}", param1, param2, target_addr);

    program[target_addr as usize] = param1 * param2;
}

#[aoc(day2, part2)]
pub fn solve(input: &str) -> u32 {
    let mut program: Vec<u32> = input.trim().split(",").map(|s| s.parse::<u32>().unwrap()).collect();
    // Position 1 changed to 12 and position 2 changed to 2 per instructions
    program[1] = 12;
    program[2] = 2;

    for noun in 0..100 {
        for verb in 0..100 {
            let mut p = program.clone();
            p[1] = noun;
            p[2] = verb;
            run_program(&mut p);

            let output = p[0];
            if output == 19690720 {
                println!("Noun, verb = {}, {}", noun, verb);
                println!("Answer = {}", 100 * noun + verb);
                return 100 * noun + verb;
            }
        }
    }

    panic!("No solution found");
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_program() {
        let mut program1 = [1,0,0,0,99];
        run_program(&mut program1);
        assert_eq!(program1, [2,0,0,0,99]);

        let mut program2 = [2,3,0,3,99];
        run_program(&mut program2);
        assert_eq!(program2, [2,3,0,6,99]);

        let mut program3 = [2,4,4,5,99,0];
        run_program(&mut program3);
        assert_eq!(program3, [2,4,4,5,99,9801]);

        let mut program4 = [1,1,1,4,99,5,6,0,99];
        run_program(&mut program4);
        assert_eq!(program4, [30,1,1,4,2,5,6,0,99]);
    }
}
