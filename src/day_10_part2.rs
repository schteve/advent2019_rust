/*
    --- Part Two ---
    Once you give them the coordinates, the Elves quickly deploy an Instant Monitoring Station to the location and discover the worst: there are simply too many asteroids.

    The only solution is complete vaporization by giant laser.

    Fortunately, in addition to an asteroid scanner, the new monitoring station also comes equipped with a giant rotating laser perfect for vaporizing asteroids. The laser starts by pointing up and always rotates clockwise, vaporizing any asteroid it hits.

    If multiple asteroids are exactly in line with the station, the laser only has enough power to vaporize one of them before continuing its rotation. In other words, the same asteroids that can be detected can be vaporized, but if vaporizing one asteroid makes another one detectable, the newly-detected asteroid won't be vaporized until the laser has returned to the same position by rotating a full 360 degrees.

    For example, consider the following map, where the asteroid with the new monitoring station (and laser) is marked X:

    .#....#####...#..
    ##...##.#####..##
    ##...#...#.#####.
    ..#.....X...###..
    ..#.#.....#....##
    The first nine asteroids to get vaporized, in order, would be:

    .#....###24...#..
    ##...##.13#67..9#
    ##...#...5.8####.
    ..#.....X...###..
    ..#.#.....#....##
    Note that some asteroids (the ones behind the asteroids marked 1, 5, and 7) won't have a chance to be vaporized until the next full rotation. The laser continues rotating; the next nine to be vaporized are:

    .#....###.....#..
    ##...##...#.....#
    ##...#......1234.
    ..#.....X...5##..
    ..#.9.....8....76
    The next nine to be vaporized are then:

    .8....###.....#..
    56...9#...#.....#
    34...7...........
    ..2.....X....##..
    ..1..............
    Finally, the laser completes its first full rotation (1 through 3), a second rotation (4 through 8), and vaporizes the last asteroid (9) partway through its third rotation:

    ......234.....6..
    ......1...5.....7
    .................
    ........X....89..
    .................
    In the large example above (the one with the best monitoring station location at 11,13):

    The 1st asteroid to be vaporized is at 11,12.
    The 2nd asteroid to be vaporized is at 12,1.
    The 3rd asteroid to be vaporized is at 12,2.
    The 10th asteroid to be vaporized is at 12,8.
    The 20th asteroid to be vaporized is at 16,0.
    The 50th asteroid to be vaporized is at 16,9.
    The 100th asteroid to be vaporized is at 10,16.
    The 199th asteroid to be vaporized is at 9,6.
    The 200th asteroid to be vaporized is at 8,2.
    The 201st asteroid to be vaporized is at 10,9.
    The 299th and final asteroid to be vaporized is at 11,1.
    The Elves are placing bets on which will be the 200th asteroid to be vaporized. Win the bet by determining which asteroid that will be; what do you get if you multiply its X coordinate by 100 and then add its Y coordinate? (For example, 8,2 becomes 802.)
*/

extern crate num;

use num::integer::gcd;
use std::collections::HashSet;
use std::f32;

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
            data: data,
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
            .map(|&x| x)
            .collect()
    }

    fn count_visible(&self, from_location: (i32, i32)) -> usize {
        self.get_visible(from_location).len()
    }

    fn best_station(&self) -> (i32, i32) {
        let mut best_asteroid = (0, 0);
        let mut best_count = 0;
        for &asteroid in &self.data {
            let count = self.count_visible(asteroid);
            if count > best_count {
                best_asteroid = asteroid;
                best_count = count;
            }
        }

        best_asteroid
    }

    fn vaporize(&mut self, from_location: (i32, i32), count: usize) -> (i32, i32) {
        let mut num_vaporized = 0;

        loop {
            let mut visible = self.get_visible(from_location);
            if visible.len() == 0 {
                return from_location;
            }

            // Take all visible asteroids and sort them in angular order
            visible.sort_by(|&a, &b| angle_between_points(from_location, a).partial_cmp(&angle_between_points(from_location, b)).unwrap());
            // visible.iter().for_each(|&p| println!("{:?}: {}", p, angle_between_points(from_location, p)));

            // Remove asteroids one by one
            for &v in &visible {
                self.data.remove(&v);

                if num_vaporized == count {
                    return v;
                } else {
                    num_vaporized += 1;
                }
            }
        }
    }
}

fn angle_between_points(point_a: (i32, i32), point_b: (i32, i32)) -> f32 {
    let diff = (point_b.0 - point_a.0, point_b.1 - point_a.1);
    let angle = (diff.1 as f32).atan2(diff.0 as f32).to_degrees();
    // angle
    let mut adjusted_angle = angle + 90.0;
    if adjusted_angle < 0.0 {
        adjusted_angle += 360.0;
    }
    adjusted_angle
}

#[aoc(day10, part2)]
pub fn solve(input: &str) -> i32 {
    let mut asteroids = AsteroidMap::from_string(&input);
    // println!("{:?}", asteroids.data);

    let best_station = asteroids.best_station();
    let last = asteroids.vaporize(best_station, 199);
    let key = last.0 * 100 + last.1;
    println!("Last asteroid: {:?}, key: {}", last, key);
    key
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

    #[test]
    fn test_angle_between_points() {
        let angle = angle_between_points((0, 0), (0, -1));
        let abs_difference = (angle - 0.0).abs();
        assert!(abs_difference <= f32::EPSILON);

        let angle = angle_between_points((0, 0), (1, -1));
        let abs_difference = (angle - 45.0).abs();
        assert!(abs_difference <= f32::EPSILON);

        let angle = angle_between_points((0, 0), (1, 0));
        let abs_difference = (angle - 90.0).abs();
        assert!(abs_difference <= f32::EPSILON);

        let angle = angle_between_points((0, 0), (1, 1));
        let abs_difference = (angle - 135.0).abs();
        assert!(abs_difference <= f32::EPSILON);

        let angle = angle_between_points((0, 0), (0, 1));
        let abs_difference = (angle - 180.0).abs();
        assert!(abs_difference <= f32::EPSILON);

        let angle = angle_between_points((0, 0), (-1, 1));
        let abs_difference = (angle - 225.0).abs();
        assert!(abs_difference <= f32::EPSILON);

        let angle = angle_between_points((0, 0), (-1, 0));
        let abs_difference = (angle - 270.0).abs();
        assert!(abs_difference <= f32::EPSILON);

        let angle = angle_between_points((0, 0), (-1, -1));
        let abs_difference = (angle - 315.0).abs();
        assert!(abs_difference <= f32::EPSILON);
    }

    #[test]
    fn test_vaporize() {
        let input_str =
"
.#....#####...#..
##...##.#####..##
##...#...#.#####.
..#.....#...###..
..#.#.....#....##
";
        let mut asteroids = AsteroidMap::from_string(&input_str);
        assert_eq!(asteroids.vaporize((8, 3), 0),  (8,  1)); // 1
        let mut asteroids = AsteroidMap::from_string(&input_str);
        assert_eq!(asteroids.vaporize((8, 3), 1),  (9,  0)); // 2
        let mut asteroids = AsteroidMap::from_string(&input_str);
        assert_eq!(asteroids.vaporize((8, 3), 2),  (9,  1)); // 3
        let mut asteroids = AsteroidMap::from_string(&input_str);
        assert_eq!(asteroids.vaporize((8, 3), 3),  (10, 0)); // 4
        let mut asteroids = AsteroidMap::from_string(&input_str);
        assert_eq!(asteroids.vaporize((8, 3), 4),  (9,  2)); // 5
        let mut asteroids = AsteroidMap::from_string(&input_str);
        assert_eq!(asteroids.vaporize((8, 3), 5),  (11, 1)); // 6
        let mut asteroids = AsteroidMap::from_string(&input_str);
        assert_eq!(asteroids.vaporize((8, 3), 6),  (12, 1)); // 7
        let mut asteroids = AsteroidMap::from_string(&input_str);
        assert_eq!(asteroids.vaporize((8, 3), 7),  (11, 2)); // 8
        let mut asteroids = AsteroidMap::from_string(&input_str);
        assert_eq!(asteroids.vaporize((8, 3), 8),  (15, 1)); // 9

        let mut asteroids = AsteroidMap::from_string(&input_str);
        assert_eq!(asteroids.vaporize((8, 3), 9),  (12, 2)); // 1
        let mut asteroids = AsteroidMap::from_string(&input_str);
        assert_eq!(asteroids.vaporize((8, 3), 10), (13, 2)); // 2
        let mut asteroids = AsteroidMap::from_string(&input_str);
        assert_eq!(asteroids.vaporize((8, 3), 11), (14, 2)); // 3
        let mut asteroids = AsteroidMap::from_string(&input_str);
        assert_eq!(asteroids.vaporize((8, 3), 12), (15, 2)); // 4
        let mut asteroids = AsteroidMap::from_string(&input_str);
        assert_eq!(asteroids.vaporize((8, 3), 13), (12, 3)); // 5
        let mut asteroids = AsteroidMap::from_string(&input_str);
        assert_eq!(asteroids.vaporize((8, 3), 14), (16, 4)); // 6
        let mut asteroids = AsteroidMap::from_string(&input_str);
        assert_eq!(asteroids.vaporize((8, 3), 15), (15, 4)); // 7
        let mut asteroids = AsteroidMap::from_string(&input_str);
        assert_eq!(asteroids.vaporize((8, 3), 16), (10, 4)); // 8
        let mut asteroids = AsteroidMap::from_string(&input_str);
        assert_eq!(asteroids.vaporize((8, 3), 17), (4,  4)); // 9

        let mut asteroids = AsteroidMap::from_string(&input_str);
        assert_eq!(asteroids.vaporize((8, 3), 18), (2,  4)); // 1
        let mut asteroids = AsteroidMap::from_string(&input_str);
        assert_eq!(asteroids.vaporize((8, 3), 19), (2,  3)); // 2
        let mut asteroids = AsteroidMap::from_string(&input_str);
        assert_eq!(asteroids.vaporize((8, 3), 20), (0,  2)); // 3
        let mut asteroids = AsteroidMap::from_string(&input_str);
        assert_eq!(asteroids.vaporize((8, 3), 21), (1,  2)); // 4
        let mut asteroids = AsteroidMap::from_string(&input_str);
        assert_eq!(asteroids.vaporize((8, 3), 22), (0,  1)); // 5
        let mut asteroids = AsteroidMap::from_string(&input_str);
        assert_eq!(asteroids.vaporize((8, 3), 23), (1,  1)); // 6
        let mut asteroids = AsteroidMap::from_string(&input_str);
        assert_eq!(asteroids.vaporize((8, 3), 24), (5,  2)); // 7
        let mut asteroids = AsteroidMap::from_string(&input_str);
        assert_eq!(asteroids.vaporize((8, 3), 25), (1,  0)); // 8
        let mut asteroids = AsteroidMap::from_string(&input_str);
        assert_eq!(asteroids.vaporize((8, 3), 26), (5,  1)); // 9

        let mut asteroids = AsteroidMap::from_string(&input_str);
        assert_eq!(asteroids.vaporize((8, 3), 27), (6,  1)); // 1
        let mut asteroids = AsteroidMap::from_string(&input_str);
        assert_eq!(asteroids.vaporize((8, 3), 28), (6,  0)); // 2
        let mut asteroids = AsteroidMap::from_string(&input_str);
        assert_eq!(asteroids.vaporize((8, 3), 29), (7,  0)); // 3
        let mut asteroids = AsteroidMap::from_string(&input_str);
        assert_eq!(asteroids.vaporize((8, 3), 30), (8,  0)); // 4
        let mut asteroids = AsteroidMap::from_string(&input_str);
        assert_eq!(asteroids.vaporize((8, 3), 31), (10, 1)); // 5
        let mut asteroids = AsteroidMap::from_string(&input_str);
        assert_eq!(asteroids.vaporize((8, 3), 32), (14, 0)); // 6
        let mut asteroids = AsteroidMap::from_string(&input_str);
        assert_eq!(asteroids.vaporize((8, 3), 33), (16, 1)); // 7
        let mut asteroids = AsteroidMap::from_string(&input_str);
        assert_eq!(asteroids.vaporize((8, 3), 34), (13, 3)); // 8
        let mut asteroids = AsteroidMap::from_string(&input_str);
        assert_eq!(asteroids.vaporize((8, 3), 35), (14, 3)); // 9

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
        let mut asteroids = AsteroidMap::from_string(&input_str);
        assert_eq!(asteroids.vaporize((11, 13), 0),   (11, 12)); // 1st
        let mut asteroids = AsteroidMap::from_string(&input_str);
        assert_eq!(asteroids.vaporize((11, 13), 1),   (12, 1));  // 2nd
        let mut asteroids = AsteroidMap::from_string(&input_str);
        assert_eq!(asteroids.vaporize((11, 13), 2),   (12, 2));  // 3rd
        let mut asteroids = AsteroidMap::from_string(&input_str);
        assert_eq!(asteroids.vaporize((11, 13), 9),   (12, 8));  // 10th
        let mut asteroids = AsteroidMap::from_string(&input_str);
        assert_eq!(asteroids.vaporize((11, 13), 19),  (16, 0));  // 20th
        let mut asteroids = AsteroidMap::from_string(&input_str);
        assert_eq!(asteroids.vaporize((11, 13), 49),  (16, 9));  // 50th
        let mut asteroids = AsteroidMap::from_string(&input_str);
        assert_eq!(asteroids.vaporize((11, 13), 99),  (10, 16)); // 100th
        let mut asteroids = AsteroidMap::from_string(&input_str);
        assert_eq!(asteroids.vaporize((11, 13), 198), (9,  6));  // 199th
        let mut asteroids = AsteroidMap::from_string(&input_str);
        assert_eq!(asteroids.vaporize((11, 13), 199), (8,  2));  // 200th
        let mut asteroids = AsteroidMap::from_string(&input_str);
        assert_eq!(asteroids.vaporize((11, 13), 200), (10, 9));  // 201st
        let mut asteroids = AsteroidMap::from_string(&input_str);
        assert_eq!(asteroids.vaporize((11, 13), 298), (11, 1));  // 299th
    }
}
