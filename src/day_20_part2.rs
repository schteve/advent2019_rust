/*
    --- Part Two ---
    Strangely, the exit isn't open when you reach it. Then, you remember: the ancient Plutonians were famous for building recursive spaces.

    The marked connections in the maze aren't portals: they physically connect to a larger or smaller copy of the maze. Specifically, the labeled tiles around the inside edge actually connect to a smaller copy of the same maze, and the smaller copy's inner labeled tiles connect to yet a smaller copy, and so on.

    When you enter the maze, you are at the outermost level; when at the outermost level, only the outer labels AA and ZZ function (as the start and end, respectively); all other outer labeled tiles are effectively walls. At any other level, AA and ZZ count as walls, but the other outer labeled tiles bring you one level outward.

    Your goal is to find a path through the maze that brings you back to ZZ at the outermost level of the maze.

    In the first example above, the shortest path is now the loop around the right side. If the starting level is 0, then taking the previously-shortest path would pass through BC (to level 1), DE (to level 2), and FG (back to level 1). Because this is not the outermost level, ZZ is a wall, and the only option is to go back around to BC, which would only send you even deeper into the recursive maze.

    In the second example above, there is no path that brings you to ZZ at the outermost level.

    Here is a more interesting example:

                 Z L X W       C
                 Z P Q B       K
      ###########.#.#.#.#######.###############
      #...#.......#.#.......#.#.......#.#.#...#
      ###.#.#.#.#.#.#.#.###.#.#.#######.#.#.###
      #.#...#.#.#...#.#.#...#...#...#.#.......#
      #.###.#######.###.###.#.###.###.#.#######
      #...#.......#.#...#...#.............#...#
      #.#########.#######.#.#######.#######.###
      #...#.#    F       R I       Z    #.#.#.#
      #.###.#    D       E C       H    #.#.#.#
      #.#...#                           #...#.#
      #.###.#                           #.###.#
      #.#....OA                       WB..#.#..ZH
      #.###.#                           #.#.#.#
    CJ......#                           #.....#
      #######                           #######
      #.#....CK                         #......IC
      #.###.#                           #.###.#
      #.....#                           #...#.#
      ###.###                           #.#.#.#
    XF....#.#                         RF..#.#.#
      #####.#                           #######
      #......CJ                       NM..#...#
      ###.#.#                           #.###.#
    RE....#.#                           #......RF
      ###.###        X   X       L      #.#.#.#
      #.....#        F   Q       P      #.#.#.#
      ###.###########.###.#######.#########.###
      #.....#...#.....#.......#...#.....#.#...#
      #####.#.###.#######.#######.###.###.#.#.#
      #.......#.......#.#.#.#.#...#...#...#.#.#
      #####.###.#####.#.#.#.#.###.###.#.###.###
      #.......#.....#.#...#...............#...#
      #############.#.#.###.###################
                   A O F   N
                   A A D   M
    One shortest path through the maze is the following:

    Walk from AA to XF (16 steps)
    Recurse into level 1 through XF (1 step)
    Walk from XF to CK (10 steps)
    Recurse into level 2 through CK (1 step)
    Walk from CK to ZH (14 steps)
    Recurse into level 3 through ZH (1 step)
    Walk from ZH to WB (10 steps)
    Recurse into level 4 through WB (1 step)
    Walk from WB to IC (10 steps)
    Recurse into level 5 through IC (1 step)
    Walk from IC to RF (10 steps)
    Recurse into level 6 through RF (1 step)
    Walk from RF to NM (8 steps)
    Recurse into level 7 through NM (1 step)
    Walk from NM to LP (12 steps)
    Recurse into level 8 through LP (1 step)
    Walk from LP to FD (24 steps)
    Recurse into level 9 through FD (1 step)
    Walk from FD to XQ (8 steps)
    Recurse into level 10 through XQ (1 step)
    Walk from XQ to WB (4 steps)
    Return to level 9 through WB (1 step)
    Walk from WB to ZH (10 steps)
    Return to level 8 through ZH (1 step)
    Walk from ZH to CK (14 steps)
    Return to level 7 through CK (1 step)
    Walk from CK to XF (10 steps)
    Return to level 6 through XF (1 step)
    Walk from XF to OA (14 steps)
    Return to level 5 through OA (1 step)
    Walk from OA to CJ (8 steps)
    Return to level 4 through CJ (1 step)
    Walk from CJ to RE (8 steps)
    Return to level 3 through RE (1 step)
    Walk from RE to IC (4 steps)
    Recurse into level 4 through IC (1 step)
    Walk from IC to RF (10 steps)
    Recurse into level 5 through RF (1 step)
    Walk from RF to NM (8 steps)
    Recurse into level 6 through NM (1 step)
    Walk from NM to LP (12 steps)
    Recurse into level 7 through LP (1 step)
    Walk from LP to FD (24 steps)
    Recurse into level 8 through FD (1 step)
    Walk from FD to XQ (8 steps)
    Recurse into level 9 through XQ (1 step)
    Walk from XQ to WB (4 steps)
    Return to level 8 through WB (1 step)
    Walk from WB to ZH (10 steps)
    Return to level 7 through ZH (1 step)
    Walk from ZH to CK (14 steps)
    Return to level 6 through CK (1 step)
    Walk from CK to XF (10 steps)
    Return to level 5 through XF (1 step)
    Walk from XF to OA (14 steps)
    Return to level 4 through OA (1 step)
    Walk from OA to CJ (8 steps)
    Return to level 3 through CJ (1 step)
    Walk from CJ to RE (8 steps)
    Return to level 2 through RE (1 step)
    Walk from RE to XQ (14 steps)
    Return to level 1 through XQ (1 step)
    Walk from XQ to FD (8 steps)
    Return to level 0 through FD (1 step)
    Walk from FD to ZZ (18 steps)
    This path takes a total of 396 steps to move from AA at the outermost layer to ZZ at the outermost layer.

    In your maze, when accounting for recursion, how many steps does it take to get from the open tile marked AA to the open tile marked ZZ, both at the outermost layer?
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
            Self::West =>  Self::East,
            Self::East =>  Self::West,
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
    next_depth: i32,
}

#[derive(Clone, Copy, Eq, Hash, PartialEq)]
struct Node {
    point: Point,
    depth: i32,
}

#[derive(Clone)]
struct Map {
    area: HashMap<Point, Space>,
    portals: HashMap<Point, Portal>,
}

impl Map {
    fn from_string(input: &str) -> Map {
        let mut area = HashMap::new();

        let mut p = Point {
            x: 0,
            y: 0,
        };
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

        let x_size = x_range.1 - x_range.0;
        let y_size = y_range.1 - y_range.0;
        for y in y_range.0 ..= y_range.1 {
            for x in x_range.0 ..= x_range.1 {
                if let Some(Space::PortalPiece(c0)) = self.area.get(&Point { x, y }) {
                    let portal_piece0 = *c0;

                    // We are scanning from left to right, top to bottom. So we only need to look to the right and down.
                    // If there's an adjacent letter in any other position, this is part of an existing portal.
                    let portal_piece1;
                    if let Some(Space::PortalPiece(c1)) = self.area.get(&Point { x: x + 1, y }) {
                        portal_piece1 = *c1;
                    } else if let Some(Space::PortalPiece(c1)) = self.area.get(&Point { x, y: y + 1 }) {
                        portal_piece1 = *c1;
                    } else {
                        continue; // Not a valid portal
                    }

                    // Find the walkable space. Relative to the first piece it is either one left, two right, one up, or two down.
                    // Also decide what relative depth the portal leads to. +1 means it goes inward / deeper (inner part of the donut),
                    // while -1 means it goes outward. The entrance and exit are special cases and have 0 relative depth.
                    let portal_walkable;
                    let portal_depth;
                    if let Some(Space::Empty) = self.area.get(&Point { x: x - 1, y }) {
                        portal_walkable = Point { x: x - 1, y };

                        if (portal_piece0 == 'A' && portal_piece1 == 'A') || (portal_piece0 == 'Z' && portal_piece1 == 'Z') {
                            portal_depth = 0; // Special case.
                        } else if x < x_size / 2 {
                            portal_depth = 1; // On left side of donut, and portal is to the right of the walkable space. Inward.
                        } else {
                            portal_depth = -1; // On right side of donut, and portal is to the right of the walkable space. Outward.
                        }
                    } else if let Some(Space::Empty) = self.area.get(&Point { x: x + 2, y }) {
                        portal_walkable = Point { x: x + 2, y };

                        if (portal_piece0 == 'A' && portal_piece1 == 'A') || (portal_piece0 == 'Z' && portal_piece1 == 'Z') {
                            portal_depth = 0; // Special case.
                        } else if x < x_size / 2 {
                            portal_depth = -1; // On left side of donut, and portal is to the left of the walkable space. Outward.
                        } else {
                            portal_depth = 1; // On right side of donut, and portal is to the left of the walkable space. Inward.
                        }
                    } else if let Some(Space::Empty) = self.area.get(&Point { x, y: y - 1 }) {
                        portal_walkable = Point { x, y: y - 1 };

                        if (portal_piece0 == 'A' && portal_piece1 == 'A') || (portal_piece0 == 'Z' && portal_piece1 == 'Z') {
                            portal_depth = 0; // Special case.
                        } else if y < y_size / 2 {
                            portal_depth = 1; // On upper side of donut, and portal is below the walkable space. Inward.
                        } else {
                            portal_depth = -1; // On lower side of donut, and portal is below the walkable space. Outward.
                        }
                    } else if let Some(Space::Empty) = self.area.get(&Point { x, y: y + 2 }) {
                        portal_walkable = Point { x, y: y + 2 };

                        if (portal_piece0 == 'A' && portal_piece1 == 'A') || (portal_piece0 == 'Z' && portal_piece1 == 'Z') {
                            portal_depth = 0; // Special case.
                        } else if y < y_size / 2 {
                            portal_depth = -1; // On upper side of donut, and portal is above the walkable space. Outward.
                        } else {
                            portal_depth = 1; // On lower side of donut, and portal is above the walkable space. Inward.
                        }
                    } else {
                        continue; // Not a valid portal
                    }

                    let portal = Portal {
                        value: [portal_piece0, portal_piece1],
                        next_depth: portal_depth,
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

        for y in y_range.0 ..= y_range.1 {
            for x in x_range.0 ..= x_range.1 {
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

        let mut frontier: Vec<Node> = Vec::new();
        let mut walked: HashSet<Node> = HashSet::new();

        let entrance_node = Node {
            point: entrance,
            depth: 0,
        };
        frontier.push(entrance_node);
        walked.insert(entrance_node);

        let mut steps = 0;
        loop {
            steps += 1;
            for location in frontier.drain(..).collect::<Vec<Node>>() {
                let candidates = [Cardinal::North,
                                  Cardinal::South,
                                  Cardinal::West,
                                  Cardinal::East];
                for direction in candidates.iter() {
                    let step_in_direction = direction.step_from(location.point);
                    //println!("Step: {:?}", step_in_direction);
                    let step_node = Node {
                        point: step_in_direction,
                        depth: location.depth,
                    };
                    if walked.get(&step_node) == None {
                        // Step into any adjacent empty space
                        if self.area.get(&step_in_direction) == Some(&Space::Empty) {
                            // If this is the goal space, return now
                            if let Some(&portal) = self.portals.get(&step_in_direction) {
                                if portal.value == ['Z', 'Z'] && location.depth == 0 {
                                    return steps;
                                }
                            }

                            // Otherwise, add it to the frontier
                            frontier.push(step_node);
                            walked.insert(step_node);
                        }
                    }
                }

                // Also step into any portals connected to the current space
                if let Some(&portal) = self.portals.get(&location.point) {
                    // The portal can be taken if stepping through it would result in a non-negative
                    let portal_depth = location.depth + portal.next_depth;
                    if portal_depth >= 0 {
                        // Find matching portal
                        for (&k, &v) in self.portals.iter() {
                            let portal_node = Node {
                                point: k,
                                depth: portal_depth,
                            };
                            if k != location.point && v.value == portal.value && walked.get(&portal_node) == None {
                                frontier.push(portal_node);
                                walked.insert(portal_node);
                            }
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

#[aoc(day20, part2)]
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
        assert_eq!(steps, 26);

let input = "
             Z L X W       C
             Z P Q B       K
  ###########.#.#.#.#######.###############
  #...#.......#.#.......#.#.......#.#.#...#
  ###.#.#.#.#.#.#.#.###.#.#.#######.#.#.###
  #.#...#.#.#...#.#.#...#...#...#.#.......#
  #.###.#######.###.###.#.###.###.#.#######
  #...#.......#.#...#...#.............#...#
  #.#########.#######.#.#######.#######.###
  #...#.#    F       R I       Z    #.#.#.#
  #.###.#    D       E C       H    #.#.#.#
  #.#...#                           #...#.#
  #.###.#                           #.###.#
  #.#....OA                       WB..#.#..ZH
  #.###.#                           #.#.#.#
CJ......#                           #.....#
  #######                           #######
  #.#....CK                         #......IC
  #.###.#                           #.###.#
  #.....#                           #...#.#
  ###.###                           #.#.#.#
XF....#.#                         RF..#.#.#
  #####.#                           #######
  #......CJ                       NM..#...#
  ###.#.#                           #.###.#
RE....#.#                           #......RF
  ###.###        X   X       L      #.#.#.#
  #.....#        F   Q       P      #.#.#.#
  ###.###########.###.#######.#########.###
  #.....#...#.....#.......#...#.....#.#...#
  #####.#.###.#######.#######.###.###.#.#.#
  #.......#.......#.#.#.#.#...#...#...#.#.#
  #####.###.#####.#.#.#.#.###.###.#.###.###
  #.......#.....#.#...#...............#...#
  #############.#.#.###.###################
               A O F   N
               A A D   M
";
        let map = Map::from_string(&input);
        let steps = map.a_to_z();
        assert_eq!(steps, 396);
    }
}
