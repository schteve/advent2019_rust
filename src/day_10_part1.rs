/*
    --- Day 10: Monitoring Station ---
    You fly into the asteroid belt and reach the Ceres monitoring station. The Elves here have an emergency: they're having trouble tracking all of the asteroids and can't be sure they're safe.

    The Elves would like to build a new monitoring station in a nearby area of space; they hand you a map of all of the asteroids in that region (your puzzle input).

    The map indicates whether each position is empty (.) or contains an asteroid (#). The asteroids are much smaller than they appear on the map, and every asteroid is exactly in the center of its marked position. The asteroids can be described with X,Y coordinates where X is the distance from the left edge and Y is the distance from the top edge (so the top-left corner is 0,0 and the position immediately to its right is 1,0).

    Your job is to figure out which asteroid would be the best place to build a new monitoring station. A monitoring station can detect any asteroid to which it has direct line of sight - that is, there cannot be another asteroid exactly between them. This line of sight can be at any angle, not just lines aligned to the grid or diagonally. The best location is the asteroid that can detect the largest number of other asteroids.

    For example, consider the following map:

    .#..#
    .....
    #####
    ....#
    ...##
    The best location for a new monitoring station on this map is the highlighted asteroid at 3,4 because it can detect 8 asteroids, more than any other location. (The only asteroid it cannot detect is the one at 1,0; its view of this asteroid is blocked by the asteroid at 2,2.) All other asteroids are worse locations; they can detect 7 or fewer other asteroids. Here is the number of other asteroids a monitoring station on each asteroid could detect:

    .7..7
    .....
    67775
    ....7
    ...87
    Here is an asteroid (#) and some examples of the ways its line of sight might be blocked. If there were another asteroid at the location of a capital letter, the locations marked with the corresponding lowercase letter would be blocked and could not be detected:

    #.........
    ...A......
    ...B..a...
    .EDCG....a
    ..F.c.b...
    .....c....
    ..efd.c.gb
    .......c..
    ....f...c.
    ...e..d..c
    Here are some larger examples:

    Best is 5,8 with 33 other asteroids detected:

    ......#.#.
    #..#.#....
    ..#######.
    .#.#.###..
    .#..#.....
    ..#....#.#
    #..#....#.
    .##.#..###
    ##...#..#.
    .#....####
    Best is 1,2 with 35 other asteroids detected:

    #.#...#.#.
    .###....#.
    .#....#...
    ##.#.#.#.#
    ....#.#.#.
    .##..###.#
    ..#...##..
    ..##....##
    ......#...
    .####.###.
    Best is 6,3 with 41 other asteroids detected:

    .#..#..###
    ####.###.#
    ....###.#.
    ..###.##.#
    ##.##.#.#.
    ....###..#
    ..#.#..#.#
    #..#.#.###
    .##...##.#
    .....#.#..
    Best is 11,13 with 210 other asteroids detected:

    .#..##.###...#######
    ##.############..##.
    .#.######.########.#
    .###.#######.####.#.
    #####.##.#.##.###.##
    ..#####..#.#########
    ####################
    #.####....###.#.#.##
    ##.#################
    #####.##.###..####..
    ..######..##.#######
    ####.##.####...##..#
    .#####..#.######.###
    ##...#.##########...
    #.##########.#######
    .####.#.###.###.#.##
    ....##.##.###..#####
    .#.#.###########.###
    #.#.#.#####.####.###
    ###.##.####.##.#..##
    Find the best location for a new monitoring station. How many other asteroids can be detected from that location?
*/

use num::integer::gcd;
use std::collections::HashSet;

struct AsteroidMap {
    data: HashSet<(i32, i32)>,
}

impl AsteroidMap {
    fn from_string(input: &str) -> AsteroidMap {
        let input_vec2d: Vec<Vec<char>> = input
                                        .trim()
                                        .lines()
                                        .map(|s| s.chars().collect())
                                        .collect();
        AsteroidMap::from_vec2d(input_vec2d)
    }

    fn from_vec2d(input: Vec<Vec<char>>) -> AsteroidMap {
        let mut data = HashSet::new();
        for (y, row) in input.iter().enumerate() {
            for (x, &pos) in row.iter().enumerate() {
                if pos == '#' {
                    data.insert((x as i32, y as i32));
                }
            }
        }

        AsteroidMap {
            data,
        }
    }

    fn is_visible(&self, point_a: (i32, i32), point_b: (i32, i32)) -> bool {
        if point_a == point_b {
            // Safe short circuit -- self is never visible
            return false;
        }

        let diff = (point_b.0 - point_a.0, point_b.1 - point_a.1);
        let step_gcd = gcd(diff.0, diff.1);
        let step = (diff.0 / step_gcd, diff.1 / step_gcd);

        let mut curr = point_a; // Start at the source; skip it.
        loop {
            curr.0 += step.0;
            curr.1 += step.1;

            if curr == point_b {
                // Found destination; skip it.
                return true;
            }

            if self.data.contains(&curr) {
                return false;
            }
        }
    }

    fn get_visible(&self, from_location: (i32, i32)) -> Vec<(i32, i32)> {
        self.data
            .iter()
            .filter(|&&asteroid| self.is_visible(asteroid, from_location))
            .copied()
            .collect()
    }

    fn count_visible(&self, from_location: (i32, i32)) -> usize {
        self.get_visible(from_location).len()
    }

    fn best_station(&self) -> (i32, i32) {
        let best_asteroid = *self.data.iter()
                                    .max_by_key(|&&asteroid| self.count_visible(asteroid))
                                    .unwrap();
        best_asteroid
    }
}

#[aoc(day10, part1)]
pub fn solve(input: &str) -> usize {
    let asteroids = AsteroidMap::from_string(&input);
    // println!("{:?}", asteroids.data);

    let best_station = asteroids.best_station();
    let best_visible = asteroids.count_visible(best_station);
    println!("Best station: {:?} with {} visible", best_station, best_visible);
    best_visible
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_is_visible() {
        let input_str =
"
#.........
...#......
...#......
.####.....
..#.......
..........
..........
..........
..........
..........
";
        let asteroids = AsteroidMap::from_string(&input_str);
        assert_eq!(asteroids.is_visible((0, 0), (3, 1)), true); // A
        assert_eq!(asteroids.is_visible((0, 0), (3, 2)), true); // B
        assert_eq!(asteroids.is_visible((0, 0), (3, 3)), true); // C
        assert_eq!(asteroids.is_visible((0, 0), (2, 3)), true); // D
        assert_eq!(asteroids.is_visible((0, 0), (1, 3)), true); // E
        assert_eq!(asteroids.is_visible((0, 0), (2, 4)), true); // F
        assert_eq!(asteroids.is_visible((0, 0), (4, 3)), true); // G

        assert_eq!(asteroids.is_visible((0, 0), (6, 2)), false); // a
        assert_eq!(asteroids.is_visible((0, 0), (9, 3)), false); // a

        assert_eq!(asteroids.is_visible((0, 0), (6, 4)), false); // b
        assert_eq!(asteroids.is_visible((0, 0), (9, 6)), false); // b

        assert_eq!(asteroids.is_visible((0, 0), (4, 4)), false); // c
        assert_eq!(asteroids.is_visible((0, 0), (5, 5)), false); // c
        assert_eq!(asteroids.is_visible((0, 0), (6, 6)), false); // c
        assert_eq!(asteroids.is_visible((0, 0), (7, 7)), false); // c
        assert_eq!(asteroids.is_visible((0, 0), (8, 8)), false); // c
        assert_eq!(asteroids.is_visible((0, 0), (9, 9)), false); // c

        assert_eq!(asteroids.is_visible((0, 0), (4, 6)), false); // d
        assert_eq!(asteroids.is_visible((0, 0), (6, 9)), false); // d

        assert_eq!(asteroids.is_visible((0, 0), (2, 6)), false); // e
        assert_eq!(asteroids.is_visible((0, 0), (3, 9)), false); // e

        assert_eq!(asteroids.is_visible((0, 0), (3, 6)), false); // f
        assert_eq!(asteroids.is_visible((0, 0), (4, 8)), false); // f

        assert_eq!(asteroids.is_visible((0, 0), (8, 6)), false); // g

        assert_eq!(asteroids.is_visible((3, 1), (0, 0)), true); // A
        assert_eq!(asteroids.is_visible((3, 2), (0, 0)), true); // B
        assert_eq!(asteroids.is_visible((3, 3), (0, 0)), true); // C
        assert_eq!(asteroids.is_visible((2, 3), (0, 0)), true); // D
        assert_eq!(asteroids.is_visible((1, 3), (0, 0)), true); // E
        assert_eq!(asteroids.is_visible((2, 4), (0, 0)), true); // F
        assert_eq!(asteroids.is_visible((4, 3), (0, 0)), true); // G

        assert_eq!(asteroids.is_visible((6, 2), (0, 0)), false); // a
        assert_eq!(asteroids.is_visible((9, 3), (0, 0)), false); // a

        assert_eq!(asteroids.is_visible((6, 4), (0, 0)), false); // b
        assert_eq!(asteroids.is_visible((9, 6), (0, 0)), false); // b

        assert_eq!(asteroids.is_visible((4, 4), (0, 0)), false); // c
        assert_eq!(asteroids.is_visible((5, 5), (0, 0)), false); // c
        assert_eq!(asteroids.is_visible((6, 6), (0, 0)), false); // c
        assert_eq!(asteroids.is_visible((7, 7), (0, 0)), false); // c
        assert_eq!(asteroids.is_visible((8, 8), (0, 0)), false); // c
        assert_eq!(asteroids.is_visible((9, 9), (0, 0)), false); // c

        assert_eq!(asteroids.is_visible((4, 6), (0, 0)), false); // d
        assert_eq!(asteroids.is_visible((6, 9), (0, 0)), false); // d

        assert_eq!(asteroids.is_visible((2, 6), (0, 0)), false); // e
        assert_eq!(asteroids.is_visible((3, 9), (0, 0)), false); // e

        assert_eq!(asteroids.is_visible((3, 6), (0, 0)), false); // f
        assert_eq!(asteroids.is_visible((4, 8), (0, 0)), false); // f

        assert_eq!(asteroids.is_visible((8, 6), (0, 0)), false); // g
    }

    #[test]
    fn test_get_visible() {
        let input_str =
"
#.........
...#......
...#..#...
.####....#
..#.#.#...
.....#....
..###.#.##
.......#..
....#...#.
...#..#..#
";
        let asteroids = AsteroidMap::from_string(&input_str);
        let visible_list = asteroids.get_visible((0, 0));
        let test_list = vec![(3, 1), // A
                             (3, 2), // B
                             (3, 3), // C
                             (2, 3), // D
                             (1, 3), // E
                             (2, 4), // F
                             (4, 3), // G
                             ];
        for item in &visible_list {
            assert!(test_list.contains(&item));
        }
        for item in &test_list {
            assert!(visible_list.contains(&item));
        }
    }

    #[test]
    fn test_count_visible() {
        let input_str =
"
.#..#
.....
#####
....#
...##
";
        let asteroids = AsteroidMap::from_string(&input_str);
        assert_eq!(asteroids.count_visible((1, 0)), 7);
        assert_eq!(asteroids.count_visible((4, 0)), 7);
        assert_eq!(asteroids.count_visible((0, 2)), 6);
        assert_eq!(asteroids.count_visible((1, 2)), 7);
        assert_eq!(asteroids.count_visible((2, 2)), 7);
        assert_eq!(asteroids.count_visible((3, 2)), 7);
        assert_eq!(asteroids.count_visible((4, 2)), 5);
        assert_eq!(asteroids.count_visible((4, 3)), 7);
        assert_eq!(asteroids.count_visible((3, 4)), 8);
        assert_eq!(asteroids.count_visible((4, 4)), 7);
    }

    #[test]
    fn test_best_station() {
        let input_str =
"
.#..#
.....
#####
....#
...##
";
        let asteroids = AsteroidMap::from_string(&input_str);
        let best_station = asteroids.best_station();
        assert_eq!(best_station, (3, 4));
        assert_eq!(asteroids.count_visible(best_station), 8);

        let input_str =
"
......#.#.
#..#.#....
..#######.
.#.#.###..
.#..#.....
..#....#.#
#..#....#.
.##.#..###
##...#..#.
.#....####
";
        let asteroids = AsteroidMap::from_string(&input_str);
        let best_station = asteroids.best_station();
        assert_eq!(best_station, (5, 8));
        assert_eq!(asteroids.count_visible(best_station), 33);

        let input_str =
"
#.#...#.#.
.###....#.
.#....#...
##.#.#.#.#
....#.#.#.
.##..###.#
..#...##..
..##....##
......#...
.####.###.
";
        let asteroids = AsteroidMap::from_string(&input_str);
        let best_station = asteroids.best_station();
        assert_eq!(best_station, (1, 2));
        assert_eq!(asteroids.count_visible(best_station), 35);

        let input_str =
"
.#..#..###
####.###.#
....###.#.
..###.##.#
##.##.#.#.
....###..#
..#.#..#.#
#..#.#.###
.##...##.#
.....#.#..
";
        let asteroids = AsteroidMap::from_string(&input_str);
        let best_station = asteroids.best_station();
        assert_eq!(best_station, (6, 3));
        assert_eq!(asteroids.count_visible(best_station), 41);

        let input_str =
"
.#..##.###...#######
##.############..##.
.#.######.########.#
.###.#######.####.#.
#####.##.#.##.###.##
..#####..#.#########
####################
#.####....###.#.#.##
##.#################
#####.##.###..####..
..######..##.#######
####.##.####...##..#
.#####..#.######.###
##...#.##########...
#.##########.#######
.####.#.###.###.#.##
....##.##.###..#####
.#.#.###########.###
#.#.#.#####.####.###
###.##.####.##.#..##
";
        let asteroids = AsteroidMap::from_string(&input_str);
        let best_station = asteroids.best_station();
        assert_eq!(best_station, (11, 13));
        assert_eq!(asteroids.count_visible(best_station), 210);
    }
}
