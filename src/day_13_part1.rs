/*
    --- Day 13: Care Package ---
    As you ponder the solitude of space and the ever-increasing three-hour roundtrip for messages between you and Earth, you notice that the Space Mail Indicator Light is blinking. To help keep you sane, the Elves have sent you a care package.

    It's a new game for the ship's arcade cabinet! Unfortunately, the arcade is all the way on the other end of the ship. Surely, it won't be hard to build your own - the care package even comes with schematics.

    The arcade cabinet runs Intcode software like the game the Elves sent (your puzzle input). It has a primitive screen capable of drawing square tiles on a grid. The software draws tiles to the screen with output instructions: every three output instructions specify the x position (distance from the left), y position (distance from the top), and tile id. The tile id is interpreted as follows:

    0 is an empty tile. No game object appears in this tile.
    1 is a wall tile. Walls are indestructible barriers.
    2 is a block tile. Blocks can be broken by the ball.
    3 is a horizontal paddle tile. The paddle is indestructible.
    4 is a ball tile. The ball moves diagonally and bounces off objects.
    For example, a sequence of output values like 1,2,3,6,5,4 would draw a horizontal paddle tile (1 tile from the left and 2 tiles from the top) and a ball tile (6 tiles from the left and 5 tiles from the top).

    Start the game. How many block tiles are on the screen when the game exits?
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
            _ => panic!("Invalid param address mode: {}", mode),
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

#[derive(Clone, Copy, Eq, PartialEq)]
enum Tile {
    Empty,
    Wall,
    Block,
    Paddle,
    Ball,
}

impl Tile {
    fn from_value(value: i64) -> Tile {
        match value {
            0 => Tile::Empty,
            1 => Tile::Wall,
            2 => Tile::Block,
            3 => Tile::Paddle,
            4 => Tile::Ball,
            _ => panic!("Invalid Tile value: {}", value),
        }
    }

    fn value(&self) -> i64 {
        match *self {
            Tile::Empty => 0,
            Tile::Wall => 1,
            Tile::Block => 2,
            Tile::Paddle => 3,
            Tile::Ball => 4,
        }
    }

    fn char(&self) -> char {
        match *self {
            Tile::Empty => ' ',
            Tile::Wall => '|',
            Tile::Block => 'X',
            Tile::Paddle => '-',
            Tile::Ball => '*',
        }
    }
}

struct Game {
    tiles: HashMap<(i64, i64), Tile>,
}

impl Game {
    fn new() -> Game {
        Game {
            tiles: HashMap::new(),
        }
    }

    fn blocks_left(&self) -> usize {
        let count = self.tiles.values()
                                .filter(|&&v| v == Tile::Block)
                                .count();
        count
    }

    fn display(&self) {
        let mut x_range = (0, 0);
        let mut y_range = (0, 0);

        for k in self.tiles.keys() {
            if k.0 < x_range.0 {
                x_range = (k.0, x_range.1);
            } else if k.0 > x_range.1 {
                x_range = (x_range.0, k.0);
            }

            if k.1 < y_range.0 {
                y_range = (k.1, y_range.1);
            } else if k.1 > y_range.1 {
                y_range = (y_range.0, k.1);
            }
        }
        // println!("x_range: {:?}", x_range);
        // println!("y_range: {:?}", y_range);

        for y in y_range.0 ..= y_range.1 {
            for x in x_range.0 ..= x_range.1 {
                if let Some(t) = self.tiles.get(&(x, y)) {
                    print!("{}", t.char());
                } else {
                    print!(".");
                }
            }
            println!();
        }
        println!();
    }
}

fn run_program_with_game(program: &mut Program, game: &mut Game) {
    while program.halted == false {
        program.run_with_pause();

        if program.input_needed == true {
            panic!("Game input required but none available!");
        } else if program.output.len() >= 3 {
            let x_value = program.output.remove(0);
            let y_value = program.output.remove(0);
            let tile_value = program.output.remove(0);
            let tile = Tile::from_value(tile_value);
            game.tiles.insert((x_value, y_value), tile);
        }
    }
}

#[aoc(day13, part1)]
pub fn solve(input: &str) -> usize {
    let code: Vec<i64> = input
                            .trim()
                            .split(',')
                            .map(|s| s.parse::<i64>().unwrap())
                            .collect();
    let mut program = Program::new(&code, &[]);

    let mut game = Game::new();
    run_program_with_game(&mut program, &mut game);
    game.display();

    let blocks = game.blocks_left();
    println!("Blocks on screen: {}", blocks);
    blocks
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
