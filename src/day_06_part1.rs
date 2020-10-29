/*
    --- Day 6: Universal Orbit Map ---
    You've landed at the Universal Orbit Map facility on Mercury. Because navigation in space often involves transferring between orbits, the orbit maps here are useful for finding efficient routes between, for example, you and Santa. You download a map of the local orbits (your puzzle input).

    Except for the universal Center of Mass (COM), every object in space is in orbit around exactly one other object. An orbit looks roughly like this:

                      \
                       \
                        |
                        |
    AAA--> o            o <--BBB
                        |
                        |
                       /
                      /
    In this diagram, the object BBB is in orbit around AAA. The path that BBB takes around AAA (drawn with lines) is only partly shown. In the map data, this orbital relationship is written AAA)BBB, which means "BBB is in orbit around AAA".

    Before you use your map data to plot a course, you need to make sure it wasn't corrupted during the download. To verify maps, the Universal Orbit Map facility uses orbit count checksums - the total number of direct orbits (like the one shown above) and indirect orbits.

    Whenever A orbits B and B orbits C, then A indirectly orbits C. This chain can be any number of objects long: if A orbits B, B orbits C, and C orbits D, then A indirectly orbits D.

    For example, suppose you have the following map:

    COM)B
    B)C
    C)D
    D)E
    E)F
    B)G
    G)H
    D)I
    E)J
    J)K
    K)L
    Visually, the above map of orbits looks like this:

            G - H       J - K - L
           /           /
    COM - B - C - D - E - F
                   \
                    I
    In this visual representation, when two objects are connected by a line, the one on the right directly orbits the one on the left.

    Here, we can count the total number of orbits as follows:

    D directly orbits C and indirectly orbits B and COM, a total of 3 orbits.
    L directly orbits K and indirectly orbits J, E, D, C, B, and COM, a total of 7 orbits.
    COM orbits nothing.
    The total number of direct and indirect orbits in this example is 42.

    What is the total number of direct and indirect orbits in your map data?
*/

struct SpaceObject {
    name: String,
    parent_name: String,
    parent_idx: Option<usize>, // Index into vector; proxy for linked graph. Option because one node points to COM which is then None
    orbit_count: u32,
}

fn build_graph(input: &str) -> Vec<SpaceObject> {
    // Build vector of space objects
    let mut graph: Vec<SpaceObject> = Vec::new();
    for line in input.lines() {
        // println!("line: {}", line);
        let items: Vec<&str> = line.split(')').collect();
        let obj = SpaceObject {
            name: items[1].to_string(),
            parent_name: items[0].to_string(),
            parent_idx: None,
            orbit_count: 0, // 0 is invalid: no element can have this (except COM, which isn't instantiated)
        };
        graph.push(obj);
    }

    // Traverse the vector and update parent index
    for i in 0..graph.len() { // Can't directly iterate over objects because the borrow checker complains
        if graph[i].parent_name != "COM" {
            let parent_idx = graph.iter().position(|obj| obj.name == graph[i].parent_name);
            graph[i].parent_idx = parent_idx;
        }
    }

    // Traverse the vector and update orbit count
    for i in 0..graph.len() {
        let mut count = 0;
        let mut curr_idx = i;

        loop {
            if graph[curr_idx].orbit_count != 0 {
                // Found a node that knows its orbit count -- use it to short circuit the calculation.
                count += graph[curr_idx].orbit_count;
                break;
            } else {
                match graph[curr_idx].parent_idx {
                    Some(p_idx) => {
                        // Node doesn't know its orbit count, follow its parent
                        curr_idx = p_idx;
                        count += 1;
                    },
                    None => {
                        // Node points to COM -- its orbit count is effectively 1
                        count += 1;
                        break;
                    },
                }
            }
        }

        graph[i].orbit_count = count;
    }

    // graph.iter().for_each(|o| println!("{}", o.orbit_count));
    graph
}

fn count_orbits(graph: &[SpaceObject]) -> u32 {
    // Get the total number of orbits
    let total_orbits = graph.iter()
                            .map(|obj| obj.orbit_count)
                            .sum();
    total_orbits
}

#[aoc(day6, part1)]
pub fn solve(input: &str) -> u32 {
    let graph = build_graph(&input);
    let orbits = count_orbits(&graph);
    println!("Orbits: {}", orbits);
    orbits
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_count_orbits() {
        let input = "\
COM)B
B)C
C)D
D)E
E)F
B)G
G)H
D)I
E)J
J)K
K)L";
        let graph = build_graph(&input);
        assert_eq!(count_orbits(&graph), 42);
    }
}
