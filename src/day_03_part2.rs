/*
    --- Part Two ---
    It turns out that this circuit is very timing-sensitive; you actually need to minimize the signal delay.

    To do this, calculate the number of steps each wire takes to reach each intersection; choose the intersection where the sum of both wires' steps is lowest. If a wire visits a position on the grid multiple times, use the steps value from the first time it visits that position when calculating the total value of a specific intersection.

    The number of steps a wire takes is the total number of grid squares the wire has entered to get to that location, including the intersection being considered. Again consider the example from above:

    ...........
    .+-----+...
    .|.....|...
    .|..+--X-+.
    .|..|..|.|.
    .|.-X--+.|.
    .|..|....|.
    .|.......|.
    .o-------+.
    ...........
    In the above example, the intersection closest to the central port is reached after 8+5+5+2 = 20 steps by the first wire and 7+6+4+3 = 20 steps by the second wire for a total of 20+20 = 40 steps.

    However, the top-right intersection is better: the first wire takes only 8+5+2 = 15 and the second wire takes only 7+6+2 = 15, a total of 15+15 = 30 steps.

    Here are the best steps for the extra examples from above:

    R75,D30,R83,U83,L12,D49,R71,U7,L72
    U62,R66,U55,R34,D71,R55,D58,R83 = 610 steps
    R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51
    U98,R91,D20,R16,D67,R40,U7,R15,U6,R7 = 410 steps
    What is the fewest combined steps the wires must take to reach an intersection?
*/

#[derive(Copy, Clone)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(Copy, Clone)]
struct Coord {
    p: Point,
    steps: u32,
}

fn get_coords_from_path(path: Vec<&str>) -> Vec<Coord> {
    let mut coords = Vec::new();
    let mut current_coord = Coord { p: Point { x: 0, y: 0 }, steps: 0 };

    for &segment in path.iter() {
        let direction = segment.as_bytes()[0];
        let count = segment[1..].parse::<u32>().unwrap();

        // println!("Segment {}, {}", direction, count);

        for _ in 0..count {
            current_coord.steps += 1;

            match direction as char {
                'R' => {
                    current_coord.p.x += 1;
                    coords.push(current_coord);
                }
                'L' => {
                    current_coord.p.x -= 1;
                    coords.push(current_coord);
                }
                'U' => {
                    current_coord.p.y += 1;
                    coords.push(current_coord);
                }
                'D' => {
                    current_coord.p.y -= 1;
                    coords.push(current_coord);
                }
                _   => {
                    println!("FAIL");
                    break;
                }
            }
        }
    }

    coords
}

fn intersection(coords1: Vec<Coord>, coords2: Vec<Coord>) -> Vec<Coord> {
    let mut intersection = Vec::new();

    for &c1 in coords1.iter() {
        for &c2 in coords2.iter() {
            if (c1.p.x == c2.p.x) && (c1.p.y == c2.p.y) {
                let c = Coord { p: c1.p, steps: c1.steps + c2.steps };
                println!("Intersect = p ({}, {}), steps {}", c.p.x, c.p.y, c.steps);
                intersection.push(c);
            }
        }
    }

    // println!("Intersection = {:?}", intersection);
    intersection
}

fn best_intersection(path1: Vec<&str>, path2: Vec<&str>) -> u32 {
    let path1_coords = get_coords_from_path(path1);
    let path2_coords = get_coords_from_path(path2);

    let intersect_coords = intersection(path1_coords, path2_coords);

    let shortest_steps = intersect_coords.iter()
        .map(|&c| c.steps)
        .min()
        .unwrap();
    shortest_steps
}

#[aoc(day3, part2)]
pub fn solve(input: &str) -> u32 {
    let paths: Vec<&str> = input.lines().collect();
    let path_a = paths[0].split(",").collect::<Vec<&str>>();
    let path_b = paths[1].split(",").collect::<Vec<&str>>();

    let steps = best_intersection(path_a, path_b);
    println!("Steps = {}", steps);
    steps
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_best_intersection() {
        let path1_a = "R8,U5,L5,D3".split(",").collect::<Vec<&str>>();
        let path1_b = "U7,R6,D4,L4".split(",").collect::<Vec<&str>>();
        assert_eq!(best_intersection(path1_a, path1_b), 30);

        let path2_a = "R75,D30,R83,U83,L12,D49,R71,U7,L72".split(",").collect::<Vec<&str>>();
        let path2_b = "U62,R66,U55,R34,D71,R55,D58,R83".split(",").collect::<Vec<&str>>();
        assert_eq!(best_intersection(path2_a, path2_b), 610);

        let path3_a = "R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51".split(",").collect::<Vec<&str>>();
        let path3_b = "U98,R91,D20,R16,D67,R40,U7,R15,U6,R7".split(",").collect::<Vec<&str>>();
        assert_eq!(best_intersection(path3_a, path3_b), 410);
    }
}
