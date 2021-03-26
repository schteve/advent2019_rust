/*
    --- Part Two ---
    After careful analysis, one thing is certain: you have no idea where all these bugs are coming from.

    Then, you remember: Eris is an old Plutonian settlement! Clearly, the bugs are coming from recursively-folded space.

    This 5x5 grid is only one level in an infinite number of recursion levels. The tile in the middle of the grid is actually another 5x5 grid, the grid in your scan is contained as the middle tile of a larger 5x5 grid, and so on. Two levels of grids look like this:

        |     |         |     |
        |     |         |     |
        |     |         |     |
    -----+-----+---------+-----+-----
        |     |         |     |
        |     |         |     |
        |     |         |     |
    -----+-----+---------+-----+-----
        |     | | | | | |     |
        |     |-+-+-+-+-|     |
        |     | | | | | |     |
        |     |-+-+-+-+-|     |
        |     | | |?| | |     |
        |     |-+-+-+-+-|     |
        |     | | | | | |     |
        |     |-+-+-+-+-|     |
        |     | | | | | |     |
    -----+-----+---------+-----+-----
        |     |         |     |
        |     |         |     |
        |     |         |     |
    -----+-----+---------+-----+-----
        |     |         |     |
        |     |         |     |
        |     |         |     |
    (To save space, some of the tiles are not drawn to scale.) Remember, this is only a small part of the infinitely recursive grid; there is a 5x5 grid that contains this diagram, and a 5x5 grid that contains that one, and so on. Also, the ? in the diagram contains another 5x5 grid, which itself contains another 5x5 grid, and so on.

    The scan you took (your puzzle input) shows where the bugs are on a single level of this structure. The middle tile of your scan is empty to accommodate the recursive grids within it. Initially, no other levels contain bugs.

    Tiles still count as adjacent if they are directly up, down, left, or right of a given tile. Some tiles have adjacent tiles at a recursion level above or below its own level. For example:

        |     |         |     |
    1  |  2  |    3    |  4  |  5
        |     |         |     |
    -----+-----+---------+-----+-----
        |     |         |     |
    6  |  7  |    8    |  9  |  10
        |     |         |     |
    -----+-----+---------+-----+-----
        |     |A|B|C|D|E|     |
        |     |-+-+-+-+-|     |
        |     |F|G|H|I|J|     |
        |     |-+-+-+-+-|     |
    11  | 12  |K|L|?|N|O|  14 |  15
        |     |-+-+-+-+-|     |
        |     |P|Q|R|S|T|     |
        |     |-+-+-+-+-|     |
        |     |U|V|W|X|Y|     |
    -----+-----+---------+-----+-----
        |     |         |     |
    16  | 17  |    18   |  19 |  20
        |     |         |     |
    -----+-----+---------+-----+-----
        |     |         |     |
    21  | 22  |    23   |  24 |  25
        |     |         |     |
    Tile 19 has four adjacent tiles: 14, 18, 20, and 24.
    Tile G has four adjacent tiles: B, F, H, and L.
    Tile D has four adjacent tiles: 8, C, E, and I.
    Tile E has four adjacent tiles: 8, D, 14, and J.
    Tile 14 has eight adjacent tiles: 9, E, J, O, T, Y, 15, and 19.
    Tile N has eight adjacent tiles: I, O, S, and five tiles within the sub-grid marked ?.
    The rules about bugs living and dying are the same as before.

    For example, consider the same initial state as above:

    ....#
    #..#.
    #.?##
    ..#..
    #....
    The center tile is drawn as ? to indicate the next recursive grid. Call this level 0; the grid within this one is level 1, and the grid that contains this one is level -1. Then, after ten minutes, the grid at each level would look like this:

    Depth -5:
    ..#..
    .#.#.
    ..?.#
    .#.#.
    ..#..

    Depth -4:
    ...#.
    ...##
    ..?..
    ...##
    ...#.

    Depth -3:
    #.#..
    .#...
    ..?..
    .#...
    #.#..

    Depth -2:
    .#.##
    ....#
    ..?.#
    ...##
    .###.

    Depth -1:
    #..##
    ...##
    ..?..
    ...#.
    .####

    Depth 0:
    .#...
    .#.##
    .#?..
    .....
    .....

    Depth 1:
    .##..
    #..##
    ..?.#
    ##.##
    #####

    Depth 2:
    ###..
    ##.#.
    #.?..
    .#.##
    #.#..

    Depth 3:
    ..###
    .....
    #.?..
    #....
    #...#

    Depth 4:
    .###.
    #..#.
    #.?..
    ##.#.
    .....

    Depth 5:
    ####.
    #..#.
    #.?#.
    ####.
    .....
    In this example, after 10 minutes, a total of 99 bugs are present.

    Starting with your scan, how many bugs are present after 200 minutes?
*/

use std::collections::HashMap;

#[derive(Debug, Eq, PartialEq)]
struct BugSim {
    state: HashMap<i32, i32>,
}

impl BugSim {
    fn from_string(input: &str) -> Self {
        let mut state: i32 = 0;
        let mut idx = 0;
        for c in input.chars() {
            if c == '.' || c == '?' {
                // Empty space. Do nothing as the value is already set to 0.
                idx += 1;
            } else if c == '#' {
                // Bug. Set to 1.
                state |= 1 << idx;
                idx += 1;
            } else if c == '\r' || c == '\n' {
                // Line break, just consume
            } else {
                panic!("Unknown input: 0x{:02x}", c as u8);
            }
        }

        let mut state_map = HashMap::new();
        state_map.insert(0, state);

        Self { state: state_map }
    }

    fn get_tile(&self, depth: i32, row: i32, col: i32) -> Option<bool> {
        if (0..5).contains(&row) == false
            || (0..5).contains(&col) == false
            || (row == 2 && col == 2)
        {
            return None; // Invalid tiles may result from looking beyond the grid boundaries, or from the recursive tile
        }

        if let Some(state) = self.state.get(&depth) {
            let idx = row * 5 + col;
            let tile_is_bug = (state & (1 << idx)) != 0;
            Some(tile_is_bug)
        } else {
            Some(false) // If depth isn't in map, then it hasn't been touched before and is empty
        }
    }

    fn display(&self) {
        let mut keys: Vec<i32> = self.state.keys().cloned().collect();
        keys.sort_unstable();
        for k in keys {
            println!("Depth: {}", k);
            for row in 0..5 {
                for col in 0..5 {
                    if let Some(tile) = self.get_tile(k, row, col) {
                        if tile == false {
                            print!(".");
                        } else {
                            print!("#");
                        }
                    } else {
                        print!("?");
                    }
                }
                println!();
            }
            println!();
        }
        println!();
    }

    fn count_adjacent(&self, depth: i32, row: i32, col: i32) -> i32 {
        let mut tiles_to_check: Vec<(i32, i32, i32)> = Vec::new(); // (depth, row, col)

        // Up
        if row - 1 < 0 {
            // Adjacent to tile 8 on the diagram
            tiles_to_check.push((depth - 1, 1, 2));
        } else if row - 1 == 2 && col == 2 {
            // Adjacent to bottom row of center tile
            for up_col in 0..5 {
                tiles_to_check.push((depth + 1, 4, up_col));
            }
        } else {
            // Nothing special, just the tile above
            tiles_to_check.push((depth, row - 1, col));
        }

        // Down
        if row + 1 >= 5 {
            // Adjacent to tile 18 on the diagram
            tiles_to_check.push((depth - 1, 3, 2));
        } else if row + 1 == 2 && col == 2 {
            // Adjacent to top row of center tile
            for down_col in 0..5 {
                tiles_to_check.push((depth + 1, 0, down_col));
            }
        } else {
            // Nothing special, just the tile below
            tiles_to_check.push((depth, row + 1, col));
        }

        // Left
        if col - 1 < 0 {
            // Adjacent to tile 12 on the diagram
            tiles_to_check.push((depth - 1, 2, 1));
        } else if row == 2 && col - 1 == 2 {
            // Adjacent to rightmost row of center tile
            for left_row in 0..5 {
                tiles_to_check.push((depth + 1, left_row, 4));
            }
        } else {
            // Nothing special, just the tile to the left
            tiles_to_check.push((depth, row, col - 1));
        }

        // Right
        if col + 1 >= 5 {
            // Adjacent to tile 14 on the diagram
            tiles_to_check.push((depth - 1, 2, 3));
        } else if row == 2 && col + 1 == 2 {
            // Adjacent to leftmost row of center tile
            for right_row in 0..5 {
                tiles_to_check.push((depth + 1, right_row, 0));
            }
        } else {
            // Nothing special, just the tile to the right
            tiles_to_check.push((depth, row, col + 1));
        }

        let count = tiles_to_check
            .iter()
            .filter(|(d, r, c)| self.get_tile(*d, *r, *c) == Some(true))
            .count();
        count as i32
    }

    fn step_layer(&mut self, depth: i32) -> i32 {
        let mut next_state = 0;
        for row in 0..5 {
            for col in 0..5 {
                if let Some(tile) = self.get_tile(depth, row, col) {
                    if tile == true {
                        // It's a bug. Check if it lives or dies.
                        let count = self.count_adjacent(depth, row, col);
                        if count == 1 {
                            // Lives
                            let idx = row * 5 + col;
                            next_state |= 1 << idx;
                        } else {
                            // Dies. Nothing to modify in the next state.
                        }
                    } else {
                        // It's empty. Check if it becomes infested.
                        let count = self.count_adjacent(depth, row, col);
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
        }

        next_state
    }

    fn step_single(&mut self) {
        let mut next_state_map = HashMap::new();

        // For each existing layer, calculate the next state
        let keys = self.state.keys().cloned().collect::<Vec<i32>>();
        for &k in &keys {
            let next_state = self.step_layer(k);
            next_state_map.insert(k, next_state);
        }

        // Also calculate the next state for the layer below the minimum and above the max.
        // These are empty layers adjacent to non-empty ones which may become non-empty.
        let min_depth = *keys.iter().min().unwrap();
        let next_state = self.step_layer(min_depth - 1);
        if next_state != 0 {
            next_state_map.insert(min_depth - 1, next_state);
        }

        let max_depth = *keys.iter().max().unwrap();
        let next_state = self.step_layer(max_depth + 1);
        if next_state != 0 {
            next_state_map.insert(max_depth + 1, next_state);
        }

        self.state = next_state_map;
    }

    fn step(&mut self, num_steps: i32) {
        (0..num_steps).for_each(|_| self.step_single());
    }

    fn count_bugs(&self) -> i32 {
        let mut count = 0;
        for &depth in self.state.keys() {
            for row in 0..5 {
                for col in 0..5 {
                    if self.get_tile(depth, row, col) == Some(true) {
                        count += 1;
                    }
                }
            }
        }

        count
    }
}

#[aoc(day24, part2)]
pub fn solve(input: &str) -> i32 {
    let mut sim = BugSim::from_string(&input);
    sim.step(200);
    //sim.display();
    let total_bugs = sim.count_bugs();
    println!("Total bugs: {}", total_bugs);
    total_bugs
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

        // Depth -5
        let input = "
..#..
.#.#.
..?.#
.#.#.
..#..
";
        let sim_check_n5 = BugSim::from_string(&input);

        // Depth -4
        let input = "
...#.
...##
..?..
...##
...#.
";
        let sim_check_n4 = BugSim::from_string(&input);

        // Depth -3
        let input = "
#.#..
.#...
..?..
.#...
#.#..
";
        let sim_check_n3 = BugSim::from_string(&input);

        // Depth -2
        let input = "
.#.##
....#
..?.#
...##
.###.
";
        let sim_check_n2 = BugSim::from_string(&input);

        // Depth -1
        let input = "
#..##
...##
..?..
...#.
.####
";
        let sim_check_n1 = BugSim::from_string(&input);

        // Depth 0
        let input = "
.#...
.#.##
.#?..
.....
.....
";
        let sim_check_0 = BugSim::from_string(&input);

        // Depth 1
        let input = "
.##..
#..##
..?.#
##.##
#####
";
        let sim_check_1 = BugSim::from_string(&input);

        // Depth 2
        let input = "
###..
##.#.
#.?..
.#.##
#.#..
";
        let sim_check_2 = BugSim::from_string(&input);

        // Depth 3
        let input = "
..###
.....
#.?..
#....
#...#
";
        let sim_check_3 = BugSim::from_string(&input);

        // Depth -4
        let input = "
.###.
#..#.
#.?..
##.#.
.....
";
        let sim_check_4 = BugSim::from_string(&input);

        // Depth 5
        let input = "
####.
#..#.
#.?#.
####.
.....
";
        let sim_check_5 = BugSim::from_string(&input);

        let mut sim_check = BugSim {
            state: HashMap::new(),
        };
        let layers = [
            (-5, sim_check_n5),
            (-4, sim_check_n4),
            (-3, sim_check_n3),
            (-2, sim_check_n2),
            (-1, sim_check_n1),
            (0, sim_check_0),
            (1, sim_check_1),
            (2, sim_check_2),
            (3, sim_check_3),
            (4, sim_check_4),
            (5, sim_check_5),
        ];
        for (i, l) in &layers {
            sim_check.state.insert(*i, *l.state.get(&0).unwrap());
        }

        sim.step(10);
        assert_eq!(sim, sim_check);
    }

    #[test]
    fn test_count_bugs() {
        let input = "
....#
#..#.
#..##
..#..
#....
";
        let mut sim = BugSim::from_string(&input);
        sim.step(10);
        assert_eq!(sim.count_bugs(), 99);
    }
}
