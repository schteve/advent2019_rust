/*
    --- Day 20: Donut Maze ---
    You notice a strange pattern on the surface of Pluto and land nearby to get a closer look. Upon closer inspection, you realize you've come across one of the famous space-warping mazes of the long-lost Pluto civilization!

    Because there isn't much space on Pluto, the civilization that used to live here thrived by inventing a method for folding spacetime. Although the technology is no longer understood, mazes like this one provide a small glimpse into the daily life of an ancient Pluto citizen.

    This maze is shaped like a donut. Portals along the inner and outer edge of the donut can instantly teleport you from one side to the other. For example:

             A
             A
      #######.#########
      #######.........#
      #######.#######.#
      #######.#######.#
      #######.#######.#
      #####  B    ###.#
    BC...##  C    ###.#
      ##.##       ###.#
      ##...DE  F  ###.#
      #####    G  ###.#
      #########.#####.#
    DE..#######...###.#
      #.#########.###.#
    FG..#########.....#
      ###########.#####
                 Z
                 Z
    This map of the maze shows solid walls (#) and open passages (.). Every maze on Pluto has a start (the open tile next to AA) and an end (the open tile next to ZZ). Mazes on Pluto also have portals; this maze has three pairs of portals: BC, DE, and FG. When on an open tile next to one of these labels, a single step can take you to the other tile with the same label. (You can only walk on . tiles; labels and empty space are not traversable.)

    One path through the maze doesn't require any portals. Starting at AA, you could go down 1, right 8, down 12, left 4, and down 1 to reach ZZ, a total of 26 steps.

    However, there is a shorter path: You could walk from AA to the inner BC portal (4 steps), warp to the outer BC portal (1 step), walk to the inner DE (6 steps), warp to the outer DE (1 step), walk to the outer FG (4 steps), warp to the inner FG (1 step), and finally walk to ZZ (6 steps). In total, this is only 23 steps.

    Here is a larger example:

                       A
                       A
      #################.#############
      #.#...#...................#.#.#
      #.#.#.###.###.###.#########.#.#
      #.#.#.......#...#.....#.#.#...#
      #.#########.###.#####.#.#.###.#
      #.............#.#.....#.......#
      ###.###########.###.#####.#.#.#
      #.....#        A   C    #.#.#.#
      #######        S   P    #####.#
      #.#...#                 #......VT
      #.#.#.#                 #.#####
      #...#.#               YN....#.#
      #.###.#                 #####.#
    DI....#.#                 #.....#
      #####.#                 #.###.#
    ZZ......#               QG....#..AS
      ###.###                 #######
    JO..#.#.#                 #.....#
      #.#.#.#                 ###.#.#
      #...#..DI             BU....#..LF
      #####.#                 #.#####
    YN......#               VT..#....QG
      #.###.#                 #.###.#
      #.#...#                 #.....#
      ###.###    J L     J    #.#.###
      #.....#    O F     P    #.#...#
      #.###.#####.#.#####.#####.###.#
      #...#.#.#...#.....#.....#.#...#
      #.#####.###.###.#.#.#########.#
      #...#.#.....#...#.#.#.#.....#.#
      #.###.#####.###.###.#.#.#######
      #.#.........#...#.............#
      #########.###.###.#############
               B   J   C
               U   P   P
    Here, AA has no direct path to ZZ, but it does connect to AS and CP. By passing through AS, QG, BU, and JO, you can reach ZZ in 58 steps.

    In your maze, how many steps does it take to get from the open tile marked AA to the open tile marked ZZ?
*/

use std::collections::HashMap;
use std::collections::HashSet;
use std::fmt;

#[derive(Clone, Copy, Debug, PartialEq)]
enum Cardinal {
    North,
    South,
    West,
    East,
}

impl Cardinal {
    fn step_from(&self, coord: Point) -> Point {
        let delta = match *self {
            Self::North => (0, -1),
            Self::South => (0, 1),
            Self::West => (-1, 0),
            Self::East => (1, 0),
        };

        Point {
            x: coord.x + delta.0,
            y: coord.y + delta.1,
        }
    }

    fn opposite(&self) -> Self {
        match *self {
            Self::North => Self::South,
            Self::South => Self::North,
            Self::West => Self::East,
            Self::East => Self::West,
        }
    }
}

impl fmt::Display for Cardinal {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let disp_str = match *self {
            Self::North => "North",
            Self::South => "South",
            Self::West => "West",
            Self::East => "East",
        };
        write!(f, "{}", disp_str)
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
enum Space {
    Unknown,
    Empty,
    Wall,
    PortalPiece(char),
}

impl Space {
    fn from_value(value: char) -> Self {
        match value {
            '.' => Self::Empty,
            '#' => Self::Wall,
            'A'..='Z' => Self::PortalPiece(value),
            _ => Self::Unknown,
        }
    }

    fn char(&self) -> char {
        match *self {
            Self::Unknown => '?',
            Self::Empty => ' ',
            Self::Wall => '#',
            Self::PortalPiece(c) => c,
        }
    }
}

#[derive(Clone, Copy, Debug)]
struct Portal {
    value: [char; 2],
}

#[derive(Clone)]
struct Map {
    area: HashMap<Point, Space>,
    portals: HashMap<Point, Portal>,
}

impl Map {
    fn from_string(input: &str) -> Map {
        let mut area = HashMap::new();

        let mut p = Point { x: 0, y: 0 };
        for line in input.lines() {
            for c in line.chars() {
                let space = Space::from_value(c);
                if space != Space::Unknown {
                    area.insert(p, space);
                    p.x += 1;
                } else if c == ' ' {
                    p.x += 1;
                } else {
                    panic!("Unknown input: 0x{:02x}", c as u8);
                }
            }
            p.x = 0;
            p.y += 1;
        }

        let mut map = Map {
            area,
            portals: HashMap::new(),
        };
        map.detect_portals();
        //println!("Portals: {:#?}", map.portals);

        map
    }

    fn detect_portals(&mut self) {
        let mut x_range = (0, 0);
        let mut y_range = (0, 0);

        for k in self.area.keys() {
            if k.x < x_range.0 {
                x_range = (k.x, x_range.1);
            } else if k.x > x_range.1 {
                x_range = (x_range.0, k.x);
            }

            if k.y < y_range.0 {
                y_range = (k.y, y_range.1);
            } else if k.y > y_range.1 {
                y_range = (y_range.0, k.y);
            }
        }
        // println!("x_range: {:?}", x_range);
        // println!("y_range: {:?}", y_range);

        for y in y_range.0..=y_range.1 {
            for x in x_range.0..=x_range.1 {
                if let Some(Space::PortalPiece(c0)) = self.area.get(&Point { x, y }) {
                    let portal_piece0 = *c0;

                    // We are scanning from left to right, top to bottom. So we only need to look to the right and down.
                    // If there's an adjacent letter in any other position, this is part of an existing portal.
                    let portal_piece1;
                    if let Some(Space::PortalPiece(c1)) = self.area.get(&Point { x: x + 1, y }) {
                        portal_piece1 = *c1;
                    } else if let Some(Space::PortalPiece(c1)) =
                        self.area.get(&Point { x, y: y + 1 })
                    {
                        portal_piece1 = *c1;
                    } else {
                        continue; // Not a valid portal
                    }

                    // Find the walkable space. Relative to the first piece it is either one left, two right, one up, or two down.
                    let portal_walkable;
                    if let Some(Space::Empty) = self.area.get(&Point { x: x - 1, y }) {
                        portal_walkable = Point { x: x - 1, y };
                    } else if let Some(Space::Empty) = self.area.get(&Point { x: x + 2, y }) {
                        portal_walkable = Point { x: x + 2, y };
                    } else if let Some(Space::Empty) = self.area.get(&Point { x, y: y - 1 }) {
                        portal_walkable = Point { x, y: y - 1 };
                    } else if let Some(Space::Empty) = self.area.get(&Point { x, y: y + 2 }) {
                        portal_walkable = Point { x, y: y + 2 };
                    } else {
                        continue; // Not a valid portal
                    }

                    let portal = Portal {
                        value: [portal_piece0, portal_piece1],
                    };
                    self.portals.insert(portal_walkable, portal);
                }
            }
        }
    }

    fn display(&self) {
        let mut x_range = (0, 0);
        let mut y_range = (0, 0);

        for k in self.area.keys() {
            if k.x < x_range.0 {
                x_range = (k.x, x_range.1);
            } else if k.x > x_range.1 {
                x_range = (x_range.0, k.x);
            }

            if k.y < y_range.0 {
                y_range = (k.y, y_range.1);
            } else if k.y > y_range.1 {
                y_range = (y_range.0, k.y);
            }
        }
        // println!("x_range: {:?}", x_range);
        // println!("y_range: {:?}", y_range);

        for y in y_range.0..=y_range.1 {
            for x in x_range.0..=x_range.1 {
                if let Some(t) = self.area.get(&Point { x, y }) {
                    print!("{}", t.char());
                } else {
                    print!(" ");
                }
            }
            println!();
        }
        println!();
        println!();
    }

    fn get_entrance(&self) -> Point {
        for (&k, &v) in self.portals.iter() {
            if let ['A', 'A'] = v.value {
                return k;
            }
        }
        panic!("No entrance found!");
    }

    fn a_to_z(&self) -> u32 {
        let entrance = self.get_entrance();
        //println!("Entrance: {:?}", entrance);

        let mut frontier: Vec<Point> = Vec::new();
        let mut walked: HashSet<Point> = HashSet::new();

        frontier.push(entrance);
        walked.insert(entrance);

        let mut steps = 0;
        loop {
            steps += 1;
            for location in frontier.drain(..).collect::<Vec<Point>>() {
                let candidates = [
                    Cardinal::North,
                    Cardinal::South,
                    Cardinal::West,
                    Cardinal::East,
                ];
                for direction in candidates.iter() {
                    let step_in_direction = direction.step_from(location);
                    //println!("Step: {:?}", step_in_direction);
                    if walked.get(&step_in_direction) == None {
                        // Step into any adjacent empty space
                        if self.area.get(&step_in_direction) == Some(&Space::Empty) {
                            // If this is the goal space, return now
                            if let Some(&portal) = self.portals.get(&step_in_direction) {
                                if portal.value == ['Z', 'Z'] {
                                    return steps;
                                }
                            }

                            // Otherwise, add it to the frontier
                            frontier.push(step_in_direction);
                            walked.insert(step_in_direction);
                        }
                    }
                }

                // Also step into any portals connected to the current space
                if let Some(&portal) = self.portals.get(&location) {
                    // Find matching portal
                    for (&k, &v) in self.portals.iter() {
                        if k != location && v.value == portal.value && walked.get(&k) == None {
                            frontier.push(k);
                            walked.insert(k);
                        }
                    }
                }
            }

            //println!("frontier: {:?}", frontier);
            if frontier.is_empty() == true {
                break;
            }
        }

        panic!("Could not find portal ZZ");
    }
}

#[aoc(day20, part1)]
pub fn solve(input: &str) -> u32 {
    let map = Map::from_string(&input);
    //map.display();

    let steps = map.a_to_z();
    println!("Steps: {}", steps);
    steps
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_a_to_z() {
        let input = "
         A
         A
  #######.#########
  #######.........#
  #######.#######.#
  #######.#######.#
  #######.#######.#
  #####  B    ###.#
BC...##  C    ###.#
  ##.##       ###.#
  ##...DE  F  ###.#
  #####    G  ###.#
  #########.#####.#
DE..#######...###.#
  #.#########.###.#
FG..#########.....#
  ###########.#####
             Z
             Z
";
        let map = Map::from_string(&input);
        let steps = map.a_to_z();
        assert_eq!(steps, 23);

        let input = "
                   A
                   A
  #################.#############
  #.#...#...................#.#.#
  #.#.#.###.###.###.#########.#.#
  #.#.#.......#...#.....#.#.#...#
  #.#########.###.#####.#.#.###.#
  #.............#.#.....#.......#
  ###.###########.###.#####.#.#.#
  #.....#        A   C    #.#.#.#
  #######        S   P    #####.#
  #.#...#                 #......VT
  #.#.#.#                 #.#####
  #...#.#               YN....#.#
  #.###.#                 #####.#
DI....#.#                 #.....#
  #####.#                 #.###.#
ZZ......#               QG....#..AS
  ###.###                 #######
JO..#.#.#                 #.....#
  #.#.#.#                 ###.#.#
  #...#..DI             BU....#..LF
  #####.#                 #.#####
YN......#               VT..#....QG
  #.###.#                 #.###.#
  #.#...#                 #.....#
  ###.###    J L     J    #.#.###
  #.....#    O F     P    #.#...#
  #.###.#####.#.#####.#####.###.#
  #...#.#.#...#.....#.....#.#...#
  #.#####.###.###.#.#.#########.#
  #...#.#.....#...#.#.#.#.....#.#
  #.###.#####.###.###.#.#.#######
  #.#.........#...#.............#
  #########.###.###.#############
           B   J   C
           U   P   P
";
        let map = Map::from_string(&input);
        let steps = map.a_to_z();
        assert_eq!(steps, 58);
    }
}
