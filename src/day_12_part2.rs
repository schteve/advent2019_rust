/*
    --- Part Two ---
    All this drifting around in space makes you wonder about the nature of the universe. Does history really repeat itself? You're curious whether the moons will ever return to a previous state.

    Determine the number of steps that must occur before all of the moons' positions and velocities exactly match a previous point in time.

    For example, the first example above takes 2772 steps before they exactly match a previous point in time; it eventually returns to the initial state:

    After 0 steps:
    pos=<x= -1, y=  0, z=  2>, vel=<x=  0, y=  0, z=  0>
    pos=<x=  2, y=-10, z= -7>, vel=<x=  0, y=  0, z=  0>
    pos=<x=  4, y= -8, z=  8>, vel=<x=  0, y=  0, z=  0>
    pos=<x=  3, y=  5, z= -1>, vel=<x=  0, y=  0, z=  0>

    After 2770 steps:
    pos=<x=  2, y= -1, z=  1>, vel=<x= -3, y=  2, z=  2>
    pos=<x=  3, y= -7, z= -4>, vel=<x=  2, y= -5, z= -6>
    pos=<x=  1, y= -7, z=  5>, vel=<x=  0, y= -3, z=  6>
    pos=<x=  2, y=  2, z=  0>, vel=<x=  1, y=  6, z= -2>

    After 2771 steps:
    pos=<x= -1, y=  0, z=  2>, vel=<x= -3, y=  1, z=  1>
    pos=<x=  2, y=-10, z= -7>, vel=<x= -1, y= -3, z= -3>
    pos=<x=  4, y= -8, z=  8>, vel=<x=  3, y= -1, z=  3>
    pos=<x=  3, y=  5, z= -1>, vel=<x=  1, y=  3, z= -1>

    After 2772 steps:
    pos=<x= -1, y=  0, z=  2>, vel=<x=  0, y=  0, z=  0>
    pos=<x=  2, y=-10, z= -7>, vel=<x=  0, y=  0, z=  0>
    pos=<x=  4, y= -8, z=  8>, vel=<x=  0, y=  0, z=  0>
    pos=<x=  3, y=  5, z= -1>, vel=<x=  0, y=  0, z=  0>
    Of course, the universe might last for a very long time before repeating. Here's a copy of the second example from above:

    <x=-8, y=-10, z=0>
    <x=5, y=5, z=10>
    <x=2, y=-7, z=3>
    <x=9, y=-8, z=-3>
    This set of initial positions takes 4686774924 steps before it repeats a previous state! Clearly, you might need to find a more efficient way to simulate the universe.

    How many steps does it take to reach the first state that exactly matches a previous state?
*/

use num::integer::lcm;
use regex::Regex;

#[derive(Clone, Copy, PartialEq, Eq)]
struct Component {
    position: i32,
    velocity: i32,
}

impl Component {
    fn from_position(position: i32) -> Component {
        Component {
            position: position,
            velocity: 0,
        }
    }
}

struct SpaceObjects {
    x: Vec<Component>,
    y: Vec<Component>,
    z: Vec<Component>,
}

struct Sim {
    objects: SpaceObjects,
}

impl Sim {
    fn from_positions(positions: Vec<(i32, i32, i32)>) -> Sim {
        Sim {
            objects: SpaceObjects {
                x: positions.iter()
                        .map(|&p| Component::from_position(p.0))
                        .collect(),
                y: positions.iter()
                        .map(|&p| Component::from_position(p.1))
                        .collect(),
                z: positions.iter()
                        .map(|&p| Component::from_position(p.2))
                        .collect(),
            }
        }
    }

    fn get_gravity(a: i32, b: i32) -> i32 {
        if a < b {
            1
        } else if a > b {
            -1
        } else {
            0
        }
    }

    fn single_step_component(objects_n: &mut Vec<Component>) {
        for i in 0..objects_n.len() {
            for j in 0..objects_n.len() {
                let gravity = Sim::get_gravity(objects_n[i].position,
                                               objects_n[j].position);
                objects_n[i].velocity = objects_n[i].velocity + gravity;
            }
        }

        objects_n.iter_mut().for_each(|obj| obj.position += obj.velocity);
    }

    fn single_step(&mut self) {
        Sim::single_step_component(&mut self.objects.x);
        Sim::single_step_component(&mut self.objects.y);
        Sim::single_step_component(&mut self.objects.z);
    }

    fn step(&mut self, num_steps: u32) {
        for _ in 0..num_steps {
            self.single_step();
        }
    }

    fn find_repeat_component(objects_n: &mut Vec<Component>) -> u64 {
        let mut count = 0u64;

        let initial_state = objects_n.clone();
        Sim::single_step_component(objects_n);
        count += 1;

        while *objects_n != initial_state {
            Sim::single_step_component(objects_n);
            count += 1;
        }

        count
    }

    fn find_repeat(&mut self) -> u64 {
        let repeat_x = Sim::find_repeat_component(&mut self.objects.x);
        let repeat_y = Sim::find_repeat_component(&mut self.objects.y);
        let repeat_z = Sim::find_repeat_component(&mut self.objects.z);
        // println!("{} {} {}", repeat_x, repeat_y, repeat_z);
        let repeat = lcm(lcm(repeat_x, repeat_y), repeat_z);
        repeat
    }

    fn display(&self) {
        for i in 0..self.objects.x.len() {
            println!("pos: ({}, {}, {}), vel: ({}, {}, {})", self.objects.x[i].position,
                                                             self.objects.y[i].position,
                                                             self.objects.z[i].position,
                                                             self.objects.x[i].velocity,
                                                             self.objects.y[i].velocity,
                                                             self.objects.z[i].velocity);
        }
        println!();
    }
}

#[aoc(day12, part2)]
pub fn solve(input: &str) -> u64 {
    let re = Regex::new(r"<x=([-\d]+), y=([-\d]+), z=([-\d]+)>").unwrap();
    let positions: Vec<(i32, i32, i32)> = re.captures_iter(input)
                                            .map(|cap| (cap[1].parse::<i32>().unwrap(),
                                                        cap[2].parse::<i32>().unwrap(),
                                                        cap[3].parse::<i32>().unwrap()))
                                            .collect();
    let mut sim = Sim::from_positions(positions);

    sim.display();
    let repeat_steps = sim.find_repeat();
    println!("Repeat found after {} steps!", repeat_steps);
    repeat_steps
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_step() {
        let positions = vec![(-1, 0, 2), (2, -10, -7), (4, -8, 8), (3, 5, -1)];
        let mut sim = Sim::from_positions(positions);
        assert_eq!(sim.objects.x[0].position, -1);
        assert_eq!(sim.objects.y[0].position, 0);
        assert_eq!(sim.objects.z[0].position, 2);
        assert_eq!(sim.objects.x[0].velocity, 0);
        assert_eq!(sim.objects.y[0].velocity, 0);
        assert_eq!(sim.objects.z[0].velocity, 0);
        assert_eq!(sim.objects.x[1].position, 2);
        assert_eq!(sim.objects.y[1].position, -10);
        assert_eq!(sim.objects.z[1].position, -7);
        assert_eq!(sim.objects.x[1].velocity, 0);
        assert_eq!(sim.objects.y[1].velocity, 0);
        assert_eq!(sim.objects.z[1].velocity, 0);
        assert_eq!(sim.objects.x[2].position, 4);
        assert_eq!(sim.objects.y[2].position, -8);
        assert_eq!(sim.objects.z[2].position, 8);
        assert_eq!(sim.objects.x[2].velocity, 0);
        assert_eq!(sim.objects.y[2].velocity, 0);
        assert_eq!(sim.objects.z[2].velocity, 0);
        assert_eq!(sim.objects.x[3].position, 3);
        assert_eq!(sim.objects.y[3].position, 5);
        assert_eq!(sim.objects.z[3].position, -1);
        assert_eq!(sim.objects.x[3].velocity, 0);
        assert_eq!(sim.objects.y[3].velocity, 0);
        assert_eq!(sim.objects.z[3].velocity, 0);
        sim.step(10);
        assert_eq!(sim.objects.x[0].position, 2);
        assert_eq!(sim.objects.y[0].position, 1);
        assert_eq!(sim.objects.z[0].position, -3);
        assert_eq!(sim.objects.x[0].velocity, -3);
        assert_eq!(sim.objects.y[0].velocity, -2);
        assert_eq!(sim.objects.z[0].velocity, 1);
        assert_eq!(sim.objects.x[1].position, 1);
        assert_eq!(sim.objects.y[1].position, -8);
        assert_eq!(sim.objects.z[1].position, 0);
        assert_eq!(sim.objects.x[1].velocity, -1);
        assert_eq!(sim.objects.y[1].velocity, 1);
        assert_eq!(sim.objects.z[1].velocity, 3);
        assert_eq!(sim.objects.x[2].position, 3);
        assert_eq!(sim.objects.y[2].position, -6);
        assert_eq!(sim.objects.z[2].position, 1);
        assert_eq!(sim.objects.x[2].velocity, 3);
        assert_eq!(sim.objects.y[2].velocity, 2);
        assert_eq!(sim.objects.z[2].velocity, -3);
        assert_eq!(sim.objects.x[3].position, 2);
        assert_eq!(sim.objects.y[3].position, 0);
        assert_eq!(sim.objects.z[3].position, 4);
        assert_eq!(sim.objects.x[3].velocity, 1);
        assert_eq!(sim.objects.y[3].velocity, -1);
        assert_eq!(sim.objects.z[3].velocity, -1);

        let positions = vec![(-8, -10, 0), (5, 5, 10), (2, -7, 3), (9, -8, -3)];
        let mut sim = Sim::from_positions(positions);
        sim.step(100);
        assert_eq!(sim.objects.x[0].position, 8);
        assert_eq!(sim.objects.y[0].position, -12);
        assert_eq!(sim.objects.z[0].position, -9);
        assert_eq!(sim.objects.x[0].velocity, -7);
        assert_eq!(sim.objects.y[0].velocity, 3);
        assert_eq!(sim.objects.z[0].velocity, 0);
        assert_eq!(sim.objects.x[1].position, 13);
        assert_eq!(sim.objects.y[1].position, 16);
        assert_eq!(sim.objects.z[1].position, -3);
        assert_eq!(sim.objects.x[1].velocity, 3);
        assert_eq!(sim.objects.y[1].velocity, -11);
        assert_eq!(sim.objects.z[1].velocity, -5);
        assert_eq!(sim.objects.x[2].position, -29);
        assert_eq!(sim.objects.y[2].position, -11);
        assert_eq!(sim.objects.z[2].position, -1);
        assert_eq!(sim.objects.x[2].velocity, -3);
        assert_eq!(sim.objects.y[2].velocity, 7);
        assert_eq!(sim.objects.z[2].velocity, 4);
        assert_eq!(sim.objects.x[3].position, 16);
        assert_eq!(sim.objects.y[3].position, -13);
        assert_eq!(sim.objects.z[3].position, 23);
        assert_eq!(sim.objects.x[3].velocity, 7);
        assert_eq!(sim.objects.y[3].velocity, 1);
        assert_eq!(sim.objects.z[3].velocity, 1);
    }

    #[test]
    fn test_find_repeat() {
        let positions = vec![(-1, 0, 2), (2, -10, -7), (4, -8, 8), (3, 5, -1)];
        let mut sim = Sim::from_positions(positions);
        assert_eq!(sim.find_repeat(), 2772);

        let positions = vec![(-8, -10, 0), (5, 5, 10), (2, -7, 3), (9, -8, -3)];
        let mut sim = Sim::from_positions(positions);
        assert_eq!(sim.find_repeat(), 4686774924);
    }
}
