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

use std::collections::HashMap;

#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

fn get_points_from_path(path: Vec<&str>) -> HashMap<Point, u32> {
    let mut points: HashMap<Point, u32> = HashMap::new();
    let mut current_point = Point { x: 0, y: 0 };
    let mut steps = 0;

    for segment in path {
        let direction = segment.as_bytes()[0];
        let count = segment[1..].parse::<u32>().unwrap();
        // println!("Segment {}, {}", direction, count);

        for _ in 0..count {
            steps += 1;

            match direction as char {
                'R' => current_point.x += 1,
                'L' => current_point.x -= 1,
                'U' => current_point.y += 1,
                'D' => current_point.y -= 1,
                _ => panic!("Bad format"),
            }

            points.insert(current_point, steps);
        }
    }

    points
}

fn intersection(points1: &HashMap<Point, u32>, points2: &HashMap<Point, u32>) -> Vec<Point> {
    let intersection: Vec<Point> = points1
        .keys()
        .filter(|&&p1| points2.contains_key(&p1))
        .copied()
        .collect();
    // println!("Intersection = {:?}", intersection);
    intersection
}

fn best_intersection(path1: Vec<&str>, path2: Vec<&str>) -> u32 {
    let path1_points = get_points_from_path(path1);
    let path2_points = get_points_from_path(path2);

    let intersect_points = intersection(&path1_points, &path2_points);

    let shortest_steps = intersect_points
        .iter()
        .map(|&c| path1_points[&c] + path2_points[&c])
        .min()
        .unwrap();
    shortest_steps
}

#[aoc(day3, part2)]
pub fn solve(input: &str) -> u32 {
    let paths: Vec<&str> = input.lines().collect();
    let path_a: Vec<&str> = paths[0].split(',').collect();
    let path_b: Vec<&str> = paths[1].split(',').collect();

    let steps = best_intersection(path_a, path_b);
    println!("Steps = {}", steps);
    steps
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_best_intersection() {
        let path1_a = "R8,U5,L5,D3".split(',').collect::<Vec<&str>>();
        let path1_b = "U7,R6,D4,L4".split(',').collect::<Vec<&str>>();
        assert_eq!(best_intersection(path1_a, path1_b), 30);

        let path2_a = "R75,D30,R83,U83,L12,D49,R71,U7,L72"
            .split(',')
            .collect::<Vec<&str>>();
        let path2_b = "U62,R66,U55,R34,D71,R55,D58,R83"
            .split(',')
            .collect::<Vec<&str>>();
        assert_eq!(best_intersection(path2_a, path2_b), 610);

        let path3_a = "R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51"
            .split(',')
            .collect::<Vec<&str>>();
        let path3_b = "U98,R91,D20,R16,D67,R40,U7,R15,U6,R7"
            .split(',')
            .collect::<Vec<&str>>();
        assert_eq!(best_intersection(path3_a, path3_b), 410);
    }
}
