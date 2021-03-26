/*
    --- Day 25: Cryostasis ---
    As you approach Santa's ship, your sensors report two important details:

    First, that you might be too late: the internal temperature is -40 degrees.

    Second, that one faint life signature is somewhere on the ship.

    The airlock door is locked with a code; your best option is to send in a small droid to investigate the situation. You attach your ship to Santa's, break a small hole in the hull, and let the droid run in before you seal it up again. Before your ship starts freezing, you detach your ship and set it to automatically stay within range of Santa's ship.

    This droid can follow basic instructions and report on its surroundings; you can communicate with it through an Intcode program (your puzzle input) running on an ASCII-capable computer.

    As the droid moves through its environment, it will describe what it encounters. When it says Command?, you can give it a single instruction terminated with a newline (ASCII code 10). Possible instructions are:

    Movement via north, south, east, or west.
    To take an item the droid sees in the environment, use the command take <name of item>. For example, if the droid reports seeing a red ball, you can pick it up with take red ball.
    To drop an item the droid is carrying, use the command drop <name of item>. For example, if the droid is carrying a green ball, you can drop it with drop green ball.
    To get a list of all of the items the droid is currently carrying, use the command inv (for "inventory").
    Extra spaces or other characters aren't allowed - instructions must be provided precisely.

    Santa's ship is a Reindeer-class starship; these ships use pressure-sensitive floors to determine the identity of droids and crew members. The standard configuration for these starships is for all droids to weigh exactly the same amount to make them easier to detect. If you need to get past such a sensor, you might be able to reach the correct weight by carrying items from the environment.

    Look around the ship and see if you can find the password for the main airlock.
*/

use std::collections::HashMap;
use std::fmt;
use std::io::{self, BufRead};

#[derive(Clone)]
struct Program {
    code: Vec<i64>,
    mem: HashMap<usize, i64>,
    pc: usize,
    running: bool, // Should run or pause
    halted: bool,  // Hit a halt instruction; completely done.
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
            _ => panic!("Invalid param address mode: {}", mode),
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
            _ => panic!("Invalid address: {}", addr),
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

        if self.input.is_empty() == false {
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

enum Command {
    North,
    South,
    East,
    West,
    Take(String),
    Drop(String),
    List,
    Unknown,
}

impl Command {
    fn from_string(input: &str) -> Self {
        let pieces: Vec<&str> = input.trim().split(' ').collect();
        match pieces[0] {
            "north" => Self::North,
            "south" => Self::South,
            "east" => Self::East,
            "west" => Self::West,
            "take" => Self::Take(pieces[1..].join(" ")),
            "drop" => Self::Drop(pieces[1..].join(" ")),
            "inv" => Self::List,
            _ => Self::Unknown,
        }
    }
}

impl fmt::Display for Command {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::North => write!(f, "north"),
            Self::South => write!(f, "south"),
            Self::East => write!(f, "east"),
            Self::West => write!(f, "west"),
            Self::Take(s) => write!(f, "take {}", s),
            Self::Drop(s) => write!(f, "drop {}", s),
            Self::List => write!(f, "inv"),
            Self::Unknown => write!(f, ""),
        }
    }
}

struct Droid {
    program: Program,
    commands: Vec<Command>,
}

impl Droid {
    fn new(program: Program) -> Self {
        Self {
            program,
            commands: Vec::new(),
        }
    }

    fn give_command(&mut self, command: Command) {
        for c in command.to_string().bytes() {
            self.program.input.push(c as i64);
        }
        self.program.input.push(0x0A_i64); // Always end with newline
    }

    fn print_output(&mut self) {
        for i in self.program.output.drain(..) {
            if i < 128 {
                // If it's ASCII, print it as a character
                print!("{}", (i as u8) as char);
            } else {
                panic!("Non-ASCII character received");
            }
        }
    }

    fn run(&mut self) {
        while self.program.halted == false {
            self.program.run_with_pause();
            self.print_output();

            if self.program.input_needed == true {
                if self.commands.is_empty() == false {
                    // Get command from queue
                    let command = self.commands.remove(0);
                    self.give_command(command);
                } else {
                    // Get command from user
                    let mut line = String::new();
                    let stdin = io::stdin();
                    stdin.lock().read_line(&mut line).unwrap();

                    match Command::from_string(&line) {
                        Command::Unknown => println!("Unknown command; try again:"),
                        c => self.give_command(c),
                    }
                }
            }
        }
    }
}

#[aoc(day25, part1)]
pub fn solve(input: &str) -> String {
    let code: Vec<i64> = input
        .trim()
        .split(',')
        .map(|s| s.parse::<i64>().unwrap())
        .collect();
    let program = Program::new(&code, &[]);
    let mut droid = Droid::new(program);

    // I manually explored and experimented with the items. More fun that way :-)
    let commands = vec![
        Command::North,
        Command::North,
        Command::Take(String::from("monolith")),
        Command::North,
        Command::Take(String::from("hypercube")),
        Command::South,
        Command::South,
        Command::East,
        Command::East,
        Command::Take(String::from("easter egg")),
        Command::East,
        Command::South,
        Command::Take(String::from("ornament")),
        Command::West,
        Command::South,
        Command::West,
    ];
    droid.commands.extend(commands);

    droid.run();
    String::from("See output above")
}

#[cfg(test)]
mod test {
    #[allow(unused_imports)] // No tests for this module
    use super::*;

    #[test]
    fn test_() {}
}
