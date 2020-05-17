/*
    --- Day 24: Planet of Discord ---
    You land on Eris, your last stop before reaching Santa. As soon as you do, your sensors start picking up strange life forms moving around: Eris is infested with bugs! With an over 24-hour roundtrip for messages between you and Earth, you'll have to deal with this problem on your own.

    Eris isn't a very large place; a scan of the entire area fits into a 5x5 grid (your puzzle input). The scan shows bugs (#) and empty spaces (.).

    Each minute, The bugs live and die based on the number of bugs in the four adjacent tiles:

    A bug dies (becoming an empty space) unless there is exactly one bug adjacent to it.
    An empty space becomes infested with a bug if exactly one or two bugs are adjacent to it.
    Otherwise, a bug or empty space remains the same. (Tiles on the edges of the grid have fewer than four adjacent tiles; the missing tiles count as empty space.) This process happens in every location simultaneously; that is, within the same minute, the number of adjacent bugs is counted for every tile first, and then the tiles are updated.

    Here are the first few minutes of an example scenario:

    Initial state:
    ....#
    #..#.
    #..##
    ..#..
    #....

    After 1 minute:
    #..#.
    ####.
    ###.#
    ##.##
    .##..

    After 2 minutes:
    #####
    ....#
    ....#
    ...#.
    #.###

    After 3 minutes:
    #....
    ####.
    ...##
    #.##.
    .##.#

    After 4 minutes:
    ####.
    ....#
    ##..#
    .....
    ##...
    To understand the nature of the bugs, watch for the first time a layout of bugs and empty spaces matches any previous layout. In the example above, the first layout to appear twice is:

    .....
    .....
    .....
    #....
    .#...
    To calculate the biodiversity rating for this layout, consider each tile left-to-right in the top row, then left-to-right in the second row, and so on. Each of these tiles is worth biodiversity points equal to increasing powers of two: 1, 2, 4, 8, 16, 32, and so on. Add up the biodiversity points for tiles with bugs; in this example, the 16th tile (32768 points) and 22nd tile (2097152 points) have bugs, a total biodiversity rating of 2129920.

    What is the biodiversity rating for the first layout that appears twice?
*/

use std::collections::HashSet;

#[derive(Debug, Eq, PartialEq)]
struct BugSim {
    state: i32,
}

impl BugSim {
    fn from_string(input: &str) -> Self {
        let mut state: i32 = 0;
        let mut idx = 0;
        for c in input.chars() {
            if c == '.' {
                // Empty space. Do nothing as the value is already set to 0.
                idx += 1;
            } else if c == '#' {
                // Bug. Set to 1.
                state |= 1 << idx;
                idx += 1;
            } else if c == '\n' {
                // Line break, just consume
            } else {
                panic!("Unknown input: 0x{:02x}", c as u8);
            }
        }

        Self {
            state
        }
    }

    fn get_tile(&self, row: i32, col: i32) -> bool {
        if row < 0 || row >= 5 || col < 0 || col >= 5 {
            return false; // Invalid tiles may result from looking beyond the grid boundaries, these should always count as empty
        }
        let idx = row * 5 + col;
        let tile_is_bug = (self.state & (1 << idx)) != 0;
        tile_is_bug
    }

    fn display(&self) {
        for row in 0..5 {
            for col in 0..5 {
                if self.get_tile(row, col) == false {
                    print!(".");
                } else {
                    print!("#");
                }
            }
            println!("");
        }
        println!("");
    }

    fn count_adjacent(&self, row: i32, col: i32) -> i32 {
        let mut count = 0;
        if self.get_tile(row - 1, col + 0) == true {
            count += 1;
        }
        if self.get_tile(row + 1, col + 0) == true {
            count += 1;
        }
        if self.get_tile(row + 0, col - 1) == true {
            count += 1;
        }
        if self.get_tile(row + 0, col + 1) == true {
            count += 1;
        }
        count
    }

    fn step(&mut self) {
        let mut next_state = 0;
        for row in 0..5 {
            for col in 0..5 {
                if self.get_tile(row, col) == true {
                    // It's a bug. Check if it lives or dies.
                    let count = self.count_adjacent(row, col);
                    if count == 1 {
                        // Lives
                        let idx = row * 5 + col;
                        next_state |= 1 << idx;
                    } else {
                        // Dies. Nothing to modify in the next state.
                    }
                } else {
                    // It's empty. Check if it becomes infested.
                    let count = self.count_adjacent(row, col);
                    if count == 1 || count == 2 {
                        // Infested
                        let idx = row * 5 + col;
                        next_state |= 1 << idx;
                    } else {
                        // Nothing happens.
                    }
                }
            }
        }

        self.state = next_state;
    }

    fn run_until_repeat(&mut self) {
        let mut state_set: HashSet<i32> = HashSet::new();
        state_set.insert(self.state);
        loop {
            self.step();
            if state_set.insert(self.state) == false {
                // The set already had the value. We have found a repeat, the end condition.
                return;
            }
        }
    }

    fn biodiversity(&self) -> i32 {
        self.state
    }
}

#[aoc(day24, part1)]
pub fn solve(input: &str) -> i32 {
    let mut sim = BugSim::from_string(&input);
    sim.run_until_repeat();
    //sim.display();
    let biodiversity = sim.biodiversity();
    println!("Biodiversity: {}", biodiversity);
    biodiversity
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_step() {
        let input = "
....#
#..#.
#..##
..#..
#....
";
        let mut sim = BugSim::from_string(&input);

        let input = "
#..#.
####.
###.#
##.##
.##..
";
        let sim_check = BugSim::from_string(&input);
        sim.step();
        assert_eq!(sim, sim_check);

        let input = "
#####
....#
....#
...#.
#.###
";
        let sim_check = BugSim::from_string(&input);
        sim.step();
        assert_eq!(sim, sim_check);

        let input = "
#....
####.
...##
#.##.
.##.#
";
        let sim_check = BugSim::from_string(&input);
        sim.step();
        assert_eq!(sim, sim_check);

        let input = "
####.
....#
##..#
.....
##...
";
        let sim_check = BugSim::from_string(&input);
        sim.step();
        assert_eq!(sim, sim_check);
    }

    #[test]
    fn test_run_until_repeat() {
        let input = "
....#
#..#.
#..##
..#..
#....
";
        let mut sim = BugSim::from_string(&input);
        let input = "
.....
.....
.....
#....
.#...
";
        let sim_check = BugSim::from_string(&input);
        sim.run_until_repeat();
        assert_eq!(sim, sim_check);
    }

    #[test]
    fn test_biodiversity() {
        let input = "
.....
.....
.....
#....
.#...
";
        let sim = BugSim::from_string(&input);
        assert_eq!(sim.biodiversity(), 2129920);
    }
}
