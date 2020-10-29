/*
    --- Part Two ---
    Now, you just need to figure out how many orbital transfers you (YOU) need to take to get to Santa (SAN).

    You start at the object YOU are orbiting; your destination is the object SAN is orbiting. An orbital transfer lets you move from any object to an object orbiting or orbited by that object.

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
    K)YOU
    I)SAN
    Visually, the above map of orbits looks like this:

                              YOU
                             /
            G - H       J - K - L
           /           /
    COM - B - C - D - E - F
                   \
                    I - SAN
    In this example, YOU are in orbit around K, and SAN is in orbit around I. To move from K to I, a minimum of 4 orbital transfers are required:

    K to J
    J to E
    E to D
    D to I
    Afterward, the map of orbits looks like this:

            G - H       J - K - L
           /           /
    COM - B - C - D - E - F
                   \
                    I - SAN
                     \
                      YOU
    What is the minimum number of orbital transfers required to move from the object YOU are orbiting to the object SAN is orbiting? (Between the objects they are orbiting - not between YOU and SAN.)
*/

#[derive(Clone)]
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

fn find_node_in_graph(graph: &[SpaceObject], node_name: &str) -> usize {
    let node_idx = graph.iter().position(|obj| obj.name == node_name).expect("Failed to find node!");
    node_idx
}

fn get_path_to_root(graph: &[SpaceObject], from_name: &str) -> Vec<SpaceObject>{
    let mut path = Vec::new();

    let mut idx = find_node_in_graph(graph, from_name);
    while let Some(next_idx) = graph[idx].parent_idx {
        path.push(graph[next_idx].clone());
        idx = next_idx;
    }

    path
}

// Basic strategy is to trace a path from each given node to the root and find the intersection point.
// Then, sum the hops from each node to the intersection point.
fn count_orbital_transfers(graph: &[SpaceObject], src_name: &str, dst_name: &str) -> u32 {
    let src_path = get_path_to_root(graph, src_name);
    let dst_path = get_path_to_root(graph, dst_name);

    // Find intersection
    let src_to_intersection = src_path
                                .iter()
                                .position(|src_obj| dst_path
                                                        .iter()
                                                        .any(|dst_obj| src_obj.name == dst_obj.name))
                                .unwrap();
    let intersection_to_dst = dst_path
                                .iter()
                                .position(|dst_obj| src_path
                                                        .iter()
                                                        .any(|src_obj| src_obj.name == dst_obj.name))
                                .unwrap();

    let total_transfers = src_to_intersection + intersection_to_dst;
    total_transfers as u32
}

#[aoc(day6, part2)]
pub fn solve(input: &str) -> u32 {
    let graph = build_graph(&input);
    let transfers = count_orbital_transfers(&graph, "YOU", "SAN");
    println!("Transfers: {}", transfers);
    transfers
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_count_orbital_transfers() {
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
K)L
K)YOU
I)SAN";
        let graph = build_graph(&input);
        assert_eq!(count_orbital_transfers(&graph, "YOU", "SAN"), 4);
    }
}
