/*
    --- Day 17: Set and Forget ---
    An early warning system detects an incoming solar flare and automatically activates the ship's electromagnetic shield. Unfortunately, this has cut off the Wi-Fi for many small robots that, unaware of the impending danger, are now trapped on exterior scaffolding on the unsafe side of the shield. To rescue them, you'll have to act quickly!

    The only tools at your disposal are some wired cameras and a small vacuum robot currently asleep at its charging station. The video quality is poor, but the vacuum robot has a needlessly bright LED that makes it easy to spot no matter where it is.

    An Intcode program, the Aft Scaffolding Control and Information Interface (ASCII, your puzzle input), provides access to the cameras and the vacuum robot. Currently, because the vacuum robot is asleep, you can only access the cameras.

    Running the ASCII program on your Intcode computer will provide the current view of the scaffolds. This is output, purely coincidentally, as ASCII code: 35 means #, 46 means ., 10 starts a new line of output below the current one, and so on. (Within a line, characters are drawn left-to-right.)

    In the camera output, # represents a scaffold and . represents open space. The vacuum robot is visible as ^, v, <, or > depending on whether it is facing up, down, left, or right respectively. When drawn like this, the vacuum robot is always on a scaffold; if the vacuum robot ever walks off of a scaffold and begins tumbling through space uncontrollably, it will instead be visible as X.

    In general, the scaffold forms a path, but it sometimes loops back onto itself. For example, suppose you can see the following view from the cameras:

    ..#..........
    ..#..........
    #######...###
    #.#...#...#.#
    #############
    ..#...#...#..
    ..#####...^..
    Here, the vacuum robot, ^ is facing up and sitting at one end of the scaffold near the bottom-right of the image. The scaffold continues up, loops across itself several times, and ends at the top-left of the image.

    The first step is to calibrate the cameras by getting the alignment parameters of some well-defined points. Locate all scaffold intersections; for each, its alignment parameter is the distance between its left edge and the left edge of the view multiplied by the distance between its top edge and the top edge of the view. Here, the intersections from the above image are marked O:

    ..#..........
    ..#..........
    ##O####...###
    #.#...#...#.#
    ##O###O###O##
    ..#...#...#..
    ..#####...^..
    For these intersections:

    The top-left intersection is 2 units from the left of the image and 2 units from the top of the image, so its alignment parameter is 2 * 2 = 4.
    The bottom-left intersection is 2 units from the left and 4 units from the top, so its alignment parameter is 2 * 4 = 8.
    The bottom-middle intersection is 6 from the left and 4 from the top, so its alignment parameter is 24.
    The bottom-right intersection's alignment parameter is 40.
    To calibrate the cameras, you need the sum of the alignment parameters. In the above example, this is 76.

    Run your ASCII program. What is the sum of the alignment parameters for the scaffold intersections?
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

#[derive(Clone, Copy, Debug, PartialEq)]
enum Turn {
    Left,
    Right,
}

impl Turn {
    fn to_string(&self) -> String {
        match *self {
            Self::Left => "L".to_string(),
            Self::Right => "R".to_string(),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum Cardinal {
    North,
    South,
    West,
    East,
}

impl Cardinal {
    fn to_string(&self) -> String {
        match *self {
            Self::North => "North".to_string(),
            Self::South => "South".to_string(),
            Self::West => "West".to_string(),
            Self::East => "East".to_string(),
        }
    }

    fn step_from(&self, coord: (i32, i32)) -> (i32, i32) {
        let delta = match *self {
            Self::North => (0, -1),
            Self::South => (0, 1),
            Self::West => (-1, 0),
            Self::East => (1, 0),
        };

        (coord.0 + delta.0, coord.1 + delta.1)
    }

    fn opposite(&self) -> Self {
        match *self {
            Self::North => Self::South,
            Self::South => Self::North,
            Self::West =>  Self::East,
            Self::East =>  Self::West,
        }
    }

    fn turn(&self, dir: Turn) -> Self {
        match dir {
            Turn::Left => {
                match *self {
                    Self::North => Self::West,
                    Self::West => Self::South,
                    Self::South => Self::East,
                    Self::East => Self::North,
                }
            },

            Turn::Right => {
                match *self {
                    Self::North => Self::East,
                    Self::East => Self::South,
                    Self::South => Self::West,
                    Self::West => Self::North,
                }
            },
        }
    }
}

#[derive(Clone, Copy, PartialEq)]
enum Space {
    Unknown,
    Empty,
    Scaffold,
    Intersection,
    Robot(Cardinal),
}

impl Space {
    fn from_value(value: i64) -> Self {
        match value {
            0x2E => Self::Empty,
            0x23 => Self::Scaffold,
            0x4F => Self::Intersection,
            0x5E => Self::Robot(Cardinal::North),
            0x76 => Self::Robot(Cardinal::South),
            0x3C => Self::Robot(Cardinal::West),
            0x3E => Self::Robot(Cardinal::East),
            _ => Self::Unknown,
        }
    }

    fn char(&self) -> char {
        match *self {
            Self::Unknown => ' ',
            Self::Empty => '.',
            Self::Scaffold => '#',
            Self::Intersection => 'O',
            Self::Robot(Cardinal::North) => '^',
            Self::Robot(Cardinal::South) => 'v',
            Self::Robot(Cardinal::West) => '<',
            Self::Robot(Cardinal::East) => '>',
        }
    }
}

#[derive(Debug)]
struct Segment {
    turn: Turn,
    distance: i32,
}

struct Camera<'a> {
    program: &'a mut Program,
    area: HashMap<(i32, i32), Space>
}

impl<'a> Camera<'a> {
    fn new(program: &'a mut Program) -> Camera {
        Camera {
            program: program,
            area: HashMap::new(),
        }
    }

    fn snap(&mut self) {
        self.program.run();

        let mut x_coord = 0;
        let mut y_coord = 0;
        for &o in &self.program.output {
            let space = Space::from_value(o);
            if space != Space::Unknown {
                self.area.insert((x_coord, y_coord), space);
                x_coord += 1;
            } else {
                if o == 0x0A {
                    x_coord = 0;
                    y_coord += 1;
                } else {
                    println!("Unknown output: {}", o);
                }
            }
        }
    }

    fn display(&self) {
        let mut x_range = (0, 0);
        let mut y_range = (0, 0);

        for k in self.area.keys() {
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

        for y in (y_range.0)..(y_range.1 + 1) {
            for x in (x_range.0)..(x_range.1 + 1) {
                if let Some(t) = self.area.get(&(x, y)) {
                    print!("{}", t.char());
                } else {
                    print!(" ");
                }
            }
            println!("");
        }
        println!("");
        println!("");
    }

    fn find_path(&self) -> Vec<Segment> {
        // First find starting point
        let mut starting_coord = None;
        let mut starting_dir = None;
        for (&k, &v) in self.area.iter() {
            match v {
                Space::Robot(dir) => {
                    starting_coord = Some(k);
                    starting_dir = Some(dir);
                },
                _ => (),
            }
        }

        if starting_coord == None || starting_dir == None {
            panic!("Error: could not find starting point");
        }

        // Calculate path using naieve / greedy movement
        let mut path: Vec<Segment> = Vec::new();
        let mut current_coord = starting_coord.unwrap();
        let mut current_dir = starting_dir.unwrap();
        loop {
            // Get new direction
            let mut next_turn = None;
            for &t in [Turn::Left, Turn::Right].iter() {
                // Turn in the candidate direction, then take a step
                let step_dir = current_dir.turn(t);
                let step_coord = step_dir.step_from(current_coord);
                if let Some(&s) = self.area.get(&step_coord) {
                    if s == Space::Scaffold {
                        next_turn = Some(t);
                    }
                }
            }

            if next_turn == None {
                // No more segments to follow
                break;
            }
            let next_turn = next_turn.unwrap();
            current_dir = current_dir.turn(next_turn);

            // Get distance
            let mut distance = 0;
            loop {
                let step_coord = current_dir.step_from(current_coord);
                if let Some(&s) = self.area.get(&step_coord) {
                    if s == Space::Scaffold {
                        current_coord = step_coord;
                        distance += 1;
                    } else {
                        break;
                    }
                } else {
                    break;
                }
            }

            // Segment is complete
            let segment = Segment {
                turn: next_turn,
                distance: distance,
            };
            path.push(segment);
        }

        path
    }

    fn give_string(&mut self, string: &str) {
        for c in string.bytes() {
            self.program.input.push(c as i64);
        }
        self.program.input.push(0x0A as i64); // Always end with newline
    }

    fn give_main_routine(&mut self, main_routine: &Vec<usize>) {
        let main_routine_str = main_routine.iter()
                                        .map(|x| match x {
                                                0 => "A".to_string(),
                                                1 => "B".to_string(),
                                                2 => "C".to_string(),
                                                _ => "X".to_string(),
                                            })
                                        .collect::<Vec<String>>()
                                        .join(",");
        println!("Main routine: {}", main_routine_str);

        self.give_string(&main_routine_str);
    }

    fn give_sub_routines(&mut self, sub_routines: &Vec<String>) {
        for routine in sub_routines {
            // Routines are in a packed string format. Need to split and insert commas.
            let mut expect_alpha = true;
            let mut sub_routine_vec: Vec<String> = Vec::new();
            let mut tmp_vec: Vec<char> = Vec::new();
            for c in routine.chars() {
                let complete = expect_alpha == c.is_digit(10);
                if complete == true {
                    let value_string = tmp_vec.iter().collect::<String>();
                    tmp_vec.clear();

                    sub_routine_vec.push(value_string);

                    expect_alpha = !expect_alpha;
                }
                tmp_vec.push(c);
            }

            // If there's anything leftover at the end (should be), push it too.
            if tmp_vec.len() > 0 {
                let value_string = tmp_vec.iter().collect::<String>();
                tmp_vec.clear();

                sub_routine_vec.push(value_string);
            }

            // Make string and give to robot
            let sub_routine_str = sub_routine_vec.join(",");
            println!("Sub routine: {}", sub_routine_str);
            self.give_string(&sub_routine_str);
        }
    }

    fn enable_video(&mut self, yes: bool) {
        let yes_str = if yes == true { "y".to_string() } else { "n".to_string() };
        self.give_string(&yes_str);
    }

    fn feed(&mut self) -> i64 {
        while self.program.halted == false {
            self.program.run_with_pause();

            for i in 0..self.program.output.len() {
                let output_value = self.program.output.remove(0);
                if output_value < 128 { // If it's ASCII, print it as a character
                    print!("{}", (output_value as u8) as char);
                } else {
                    return output_value;
                }
            }

            if self.program.input_needed == true {
                println!("Input needed!"); // Shouldn't happen
            }
        }

        panic!("Program halted without completing");
    }
}

fn segment_to_string(segment: &Segment, with_comma: bool) -> String {
    let parts = [segment.turn.to_string(),
                 segment.distance.to_string()];
    let segment_string = parts.join(if with_comma == true { "," } else { "" });
    // println!("Segment: {}", segment_string);
    segment_string
}

fn path_to_string(path: &[Segment], with_comma: bool) -> String {
    let path_string = path.iter()
                        .map(|seg| segment_to_string(seg, with_comma))
                        .collect::<Vec<String>>()
                        .join(if with_comma == true { "," } else { "" });
    // println!("Path: {}", path_string);
    path_string
}

fn split_sub_routines(path: &[Segment]) -> Vec<String> {
    let mut histogram: HashMap<String, i32> = HashMap::new();
    for size in 2..6 { // Impossible to have more than 5 segments in 20 characters ("X,N," * 5)
        for i in 0..(path.len() - size) {
            let sub_path = &path[i..(i+size)];
            let sub_path_str = path_to_string(sub_path, false);
            if sub_path_str.len() <= 20 { // Each sub routine is limited on length
                let entry = histogram.entry(sub_path_str).or_insert(0);
                *entry += 1;
            }
        }
    }
    // println!("Histogram: {:#?}", histogram);

    // Convert the hashmap to a vector
    let mut histogram_vec = histogram.iter()
                                    .map(|(k, v)| (k.clone(), *v))
                                    .collect::<Vec<(String, i32)>>();
    histogram_vec.sort_by(|a, b| a.1.cmp(&b.1).reverse());
    // println!("Histogram vec: {:#?}", histogram_vec);
    let paths_vec = histogram_vec.iter()
                                .map(|(string, _)| string.clone())
                                .collect::<Vec<String>>();
    // println!("Paths vec: {:#?}", paths_vec);
    paths_vec
}

fn match_path(path_str: &str, candidates: &Vec<String>, main_routine: &mut Vec<usize>, sub_routines: &mut Vec<String>) -> bool {
    for c in candidates {
        if c.len() <= path_str.len() {
            if c == &path_str[0..c.len()] {
                let mut routine_pushed = false;
                if sub_routines.contains(c) == false {
                    if sub_routines.len() < 3 {
                        sub_routines.push(c.clone());
                        routine_pushed = true;
                    } else {
                        // We already had 3 sub routines in use and this candidate wasn't one of them. Move along.
                        continue;
                    }
                }

                let position = sub_routines.iter().position(|i| i == c).unwrap();
                main_routine.push(position);

                // Check if the full path has been matched.
                if c.len() == path_str.len() {
                    return true;
                }

                let result = match_path(&path_str[c.len()..], candidates, main_routine, sub_routines);
                if result == true {
                    return true; // Propagate a full match
                }
                // It didn't work out. That's fine, we probably have other candidates to try.

                main_routine.pop();
                if routine_pushed == true {
                    sub_routines.pop();
                }
            }
        }
    }

    false // If we made it this far, the match was not successful
}

fn find_3_sub_routines(path: &[Segment]) -> (Vec<usize>, Vec<String>) {
    let path_str = path_to_string(path, false);
    let candidates = split_sub_routines(path);

    let mut main_routine: Vec<usize> = Vec::new();
    let mut sub_routines: Vec<String> = Vec::new();
    let result = match_path(&path_str, &candidates, &mut main_routine, &mut sub_routines);
    if result != true {
        println!("Error! Could not find valid path with only 3 sub routines.");
    }

    (main_routine, sub_routines)
}

#[aoc(day17, part2)]
pub fn solve(input: &str) -> i64 {
    let code: Vec<i64> = input
                            .trim()
                            .split(",")
                            .map(|s| s.parse::<i64>().unwrap())
                            .collect();
    let mut program = Program::new(&code, &[]);
    let mut camera = Camera::new(&mut program);
    camera.snap();
    camera.display();
    let path = camera.find_path();
    let (main_routine, sub_routines) = find_3_sub_routines(&path);

    let mut control_program = Program::new(&code, &[]);
    control_program.code[0] = 2; // Wake up robot
    let mut camera = Camera::new(&mut control_program);
    camera.give_main_routine(&main_routine);
    camera.give_sub_routines(&sub_routines);
    camera.enable_video(false);
    let dust = camera.feed();
    println!("Space dust: {}", dust);
    dust

}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_find_3_sub_routines() {
        let path: Vec<Segment> = vec![
            Segment { turn: Turn::Right, distance: 8 },
            Segment { turn: Turn::Right, distance: 8 },
            Segment { turn: Turn::Right, distance: 4 },
            Segment { turn: Turn::Right, distance: 4 },
            Segment { turn: Turn::Right, distance: 8 },
            Segment { turn: Turn::Left,  distance: 6 },
            Segment { turn: Turn::Left,  distance: 2 },
            Segment { turn: Turn::Right, distance: 4 },
            Segment { turn: Turn::Right, distance: 4 },
            Segment { turn: Turn::Right, distance: 8 },
            Segment { turn: Turn::Right, distance: 8 },
            Segment { turn: Turn::Right, distance: 8 },
            Segment { turn: Turn::Left,  distance: 6 },
            Segment { turn: Turn::Left,  distance: 2 },
            ];
        let (main_routine, sub_routines) = find_3_sub_routines(&path);
        assert_eq!(main_routine, [0, 1, 2, 1, 0, 2]);
        assert_eq!(sub_routines[0], "R8R8");
        assert!((sub_routines[1] == "R4R4R8") || (sub_routines[1] == "R4R4")); // Note: there are multiple valid answers, the example is just one!
        assert!((sub_routines[2] == "L6L2") || (sub_routines[2] == "R8L6L2"));
    }
}
