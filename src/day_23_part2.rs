/*
    --- Part Two ---
    Packets sent to address 255 are handled by a device called a NAT (Not Always Transmitting). The NAT is responsible for managing power consumption of the network by blocking certain packets and watching for idle periods in the computers.

    If a packet would be sent to address 255, the NAT receives it instead. The NAT remembers only the last packet it receives; that is, the data in each packet it receives overwrites the NAT's packet memory with the new packet's X and Y values.

    The NAT also monitors all computers on the network. If all computers have empty incoming packet queues and are continuously trying to receive packets without sending packets, the network is considered idle.

    Once the network is idle, the NAT sends only the last packet it received to address 0; this will cause the computers on the network to resume activity. In this way, the NAT can throttle power consumption of the network when the ship needs power in other areas.

    Monitor packets released to the computer at address 0 by the NAT. What is the first Y value delivered by the NAT to the computer at address 0 twice in a row?
*/

use std::collections::HashMap;

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

#[derive(Clone, Copy)]
struct Packet {
    address: i64,
    x: i64,
    y: i64,
}

impl Packet {
    fn from_slice(vec: Vec<i64>) -> Self {
        if vec.len() != 3 {
            panic!("Expected packet from vec of size 3");
        }

        Packet {
            address: vec[0],
            x: vec[1],
            y: vec[2],
        }
    }
}

struct Computer {
    program: Program,
    address: i64,
    idle_count: i64,
}

impl Computer {
    fn new(mut program: Program, address: i64) -> Self {
        program.run_with_pause();
        if program.input_needed == true {
            program.input.push(address);
        } else {
            panic!("Computer startup failure!");
        }

        Self {
            program,
            address,
            idle_count: 0,
        }
    }

    fn run(&mut self) -> Option<Packet> {
        if self.program.halted == true {
            return None;
        }

        self.program.run_with_pause();
        if self.program.input_needed == true {
            // Nothing to rx, but inform the program
            self.program.input.push(-1);

            // If the program is waiting for input and has no data in the output then it is idle
            if self.program.output.is_empty() == true {
                self.idle_count += 1;
            }
        }

        if self.program.output.len() >= 3 {
            // Full packet received, return it
            let output_data = self.program.output.drain(0..3).collect::<Vec<i64>>();
            let packet = Packet::from_slice(output_data);
            return Some(packet);
        } else {
            // Nothing of interest happened (maybe received part of the packet)
            return None;
        }
    }

    fn rx(&mut self, packet: Packet) {
        if packet.address == self.address {
            self.program.input.push(packet.x);
            self.program.input.push(packet.y);
        } else {
            panic!("Packet sent to wrong address");
        }

        // Packet was received, no longer idle
        self.idle_count = 0;
    }
}

struct Router {
    computers: Vec<Computer>,
}

impl Router {
    fn new(program: Program) -> Self {
        Self {
            computers: (0..50).map(|i| Computer::new(program.clone(), i)).collect(),
        }
    }

    fn run(&mut self) -> i64 {
        let mut nat: Option<Packet> = None;
        let mut previous_nat_delivered: Option<Packet> = None;
        loop {
            for i in 0..self.computers.len() {
                // Run the computer. If it sends a packet it will be returned.
                if let Some(packet) = self.computers[i].run() {
                    // Packet sent, route it to the correct computer.
                    if (packet.address as usize) < self.computers.len() {
                        self.computers[packet.address as usize].rx(packet);
                    } else if packet.address == 255 {
                        // This packet goes to the NAT
                        nat = Some(packet);
                    } else {
                        panic!("Packet with invalid address received");
                    }
                }
            }

            let is_idle = self.computers.iter().all(|c| c.idle_count > 5); // Arbitrary idle count limit
            if is_idle == true {
                let packet = nat.expect("Router is idle but NAT has no value");
                let modified_packet = Packet {
                    address: 0,
                    ..packet
                };
                self.computers[0].rx(modified_packet);

                // Check stop condition: if two NAT packets in a row match Y values
                if let Some(prev) = previous_nat_delivered {
                    if prev.y == packet.y {
                        return packet.y;
                    }
                }
                previous_nat_delivered = nat;
            }
        }
    }
}

#[aoc(day23, part2)]
pub fn solve(input: &str) -> i64 {
    let code: Vec<i64> = input
        .trim()
        .split(',')
        .map(|s| s.parse::<i64>().unwrap())
        .collect();
    let program = Program::new(&code, &[]);
    let mut router = Router::new(program);
    let result = router.run();
    println!(
        "First packet sent repeatedly by NAT has Y value: {}",
        result
    );
    result
}

#[cfg(test)]
mod test {
    #[allow(unused_imports)] // No tests for this module
    use super::*;

    #[test]
    fn test_() {}
}
