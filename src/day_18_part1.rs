/*
    --- Day 18: Many-Worlds Interpretation ---
    As you approach Neptune, a planetary security system detects you and activates a giant tractor beam on Triton! You have no choice but to land.

    A scan of the local area reveals only one interesting feature: a massive underground vault. You generate a map of the tunnels (your puzzle input). The tunnels are too narrow to move diagonally.

    Only one entrance (marked @) is present among the open passages (marked .) and stone walls (#), but you also detect an assortment of keys (shown as lowercase letters) and doors (shown as uppercase letters). Keys of a given letter open the door of the same letter: a opens A, b opens B, and so on. You aren't sure which key you need to disable the tractor beam, so you'll need to collect all of them.

    For example, suppose you have the following map:

    #########
    #b.A.@.a#
    #########
    Starting from the entrance (@), you can only access a large door (A) and a key (a). Moving toward the door doesn't help you, but you can move 2 steps to collect the key, unlocking A in the process:

    #########
    #b.....@#
    #########
    Then, you can move 6 steps to collect the only other key, b:

    #########
    #@......#
    #########
    So, collecting every key took a total of 8 steps.

    Here is a larger example:

    ########################
    #f.D.E.e.C.b.A.@.a.B.c.#
    ######################.#
    #d.....................#
    ########################
    The only reasonable move is to take key a and unlock door A:

    ########################
    #f.D.E.e.C.b.....@.B.c.#
    ######################.#
    #d.....................#
    ########################
    Then, do the same with key b:

    ########################
    #f.D.E.e.C.@.........c.#
    ######################.#
    #d.....................#
    ########################
    ...and the same with key c:

    ########################
    #f.D.E.e.............@.#
    ######################.#
    #d.....................#
    ########################
    Now, you have a choice between keys d and e. While key e is closer, collecting it now would be slower in the long run than collecting key d first, so that's the best choice:

    ########################
    #f...E.e...............#
    ######################.#
    #@.....................#
    ########################
    Finally, collect key e to unlock door E, then collect key f, taking a grand total of 86 steps.

    Here are a few more examples:

    ########################
    #...............b.C.D.f#
    #.######################
    #.....@.a.B.c.d.A.e.F.g#
    ########################
    Shortest path is 132 steps: b, a, c, d, f, e, g

    #################
    #i.G..c...e..H.p#
    ########.########
    #j.A..b...f..D.o#
    ########@########
    #k.E..a...g..B.n#
    ########.########
    #l.F..d...h..C.m#
    #################
    Shortest paths are 136 steps;
    one is: a, f, b, j, g, n, h, d, l, o, e, p, c, i, k, m

    ########################
    #@..............ac.GI.b#
    ###d#e#f################
    ###A#B#C################
    ###g#h#i################
    ########################
    Shortest paths are 81 steps; one is: a, c, f, i, d, g, b, e, h

    How many steps is the shortest path that collects all of the keys?
*/

use std::collections::HashMap;
use std::collections::HashSet;

#[derive(Clone, Copy, Debug, PartialEq)]
enum Cardinal {
    North,
    South,
    West,
    East,
}

impl Cardinal {
    fn to_string(&self) -> String {
        match *self {
            Self::North => "North".to_string(),
            Self::South => "South".to_string(),
            Self::West => "West".to_string(),
            Self::East => "East".to_string(),
        }
    }

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
    Entrance,
    Key(char),
    Door(char),
}

impl Space {
    fn from_value(value: char) -> Self {
        match value {
            '.' => Self::Empty,
            '#' => Self::Wall,
            '@' => Self::Entrance,
            'a'..='z' => Self::Key(value),
            'A'..='Z' => Self::Door(value.to_lowercase().collect::<Vec<_>>()[0]),
            _ => Self::Unknown,
        }
    }

    fn char(&self) -> char {
        match *self {
            Self::Unknown => '?',
            Self::Empty => ' ',
            Self::Wall => '#',
            Self::Entrance => '@',
            Self::Key(c) => c,
            Self::Door(c) => c.to_uppercase().collect::<Vec<_>>()[0],
        }
    }
}

#[derive(Clone)]
struct Map {
    area: HashMap<Point, Space>,
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
                } else {
                    panic!("Unknown input: 0x{:02x}", c as u8);
                }
            }
            p.x = 0;
            p.y += 1;
        }

        Map {
            area: area,
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
            println!("");
        }
        println!("");
        println!("");
    }

    fn get_entrance(&self) -> Point {
        for (&k, &v) in self.area.iter() {
            match v {
                Space::Entrance => return k,
                _ => (),
            }
        }

        panic!("Could not find entrance");
    }

    fn find_keys(&self, start_node: &Node) -> Vec<(Node, u32)> {
        let mut frontier: Vec<Point> = Vec::new();
        let mut walked: HashSet<Point> = HashSet::new();

        frontier.push(start_node.point);
        walked.insert(start_node.point);

        let mut nodes: Vec<(Node, u32)> = Vec::new();
        let mut counter = 0;
        loop {
            counter += 1;
            for location in frontier.drain(..).collect::<Vec<Point>>() {
                let candidates = [Cardinal::North,
                                  Cardinal::South,
                                  Cardinal::West,
                                  Cardinal::East];
                for direction in candidates.iter() {
                    let step_in_direction = direction.step_from(location);
                    //println!("Step: {:?}", step_in_direction);
                    if walked.get(&step_in_direction) == None {
                        match self.area.get(&step_in_direction) {
                            Some(Space::Empty) | Some(Space::Entrance) => {
                                frontier.push(step_in_direction);
                                walked.insert(step_in_direction);
                            },
                            Some(Space::Key(c)) => {
                                if start_node.has_key(*c) == false {
                                    //println!("Key: {}", c);
                                    let mut node = Node {
                                        point: step_in_direction,
                                        space: Space::Key(*c),
                                        inventory: start_node.inventory,
                                    };
                                    node.set_key(*c, true);
                                    nodes.push((node, counter));
                                } else {
                                    frontier.push(step_in_direction);
                                    walked.insert(step_in_direction);
                                }
                            },
                            Some(Space::Door(c)) => {
                                if start_node.has_key(*c) == true {
                                    //println!("Door: {}", c);
                                    frontier.push(step_in_direction);
                                    walked.insert(step_in_direction);
                                }
                            }
                            _ => (),
                        }
                    }
                }
            }

            //println!("frontier: {:?}", frontier);
            if frontier.len() == 0 {
                break;
            }
        }

        nodes
    }

    fn build_graph(&self) -> Graph {
        let entrance = self.get_entrance();
        //println!("Entrance: {:?}", entrance);

        let starting_vertex = Vertex {
            id: 0,
            node: Node {
                point: entrance,
                space: Space::Entrance,
                inventory: 0,
            },
            distance: 0,
            pi: None,
            connected: Vec::new(),
        };

        let mut node_to_vertex_map: HashMap<Node, usize> = HashMap::new();
        node_to_vertex_map.insert(starting_vertex.node, starting_vertex.id);

        let mut graph = Graph {
            vertices: Vec::new(),
        };
        graph.vertices.push(starting_vertex);

        let mut frontier: Vec<usize> = Vec::new();
        frontier.push(0); // Visit the starting vertex first

        loop {
            for frontier_id in frontier.drain(..).collect::<Vec<usize>>() {
                let connected_nodes = self.find_keys(&graph.vertices[frontier_id].node);
                /*println!("Nodes:");
                for (node, distance) in &connected_nodes {
                    println!("   {:?} @ {}", node, distance);
                }*/

                // Check if any of these connected nodes are new. If they are, create them as vertices in the graph.
                for (node, distance) in connected_nodes {
                    let id;
                    if let Some(&idx) = node_to_vertex_map.get(&node) {
                        // This one exists. Save its ID.
                        id = idx;
                    } else {
                        // This one doesn't exist. Create it and save its ID.
                        id = graph.vertices.len();
                        node_to_vertex_map.insert(node, id);

                        let v = Vertex {
                            id: id,
                            node: node,
                            distance: u32::max_value(),
                            pi: Some(graph.vertices[frontier_id].id),
                            connected: Vec::new(),
                        };
                        graph.vertices.push(v);
                        frontier.push(id);
                    }

                    // Create an edge based on the saved ID and add it to the current node in the graph.
                    let edge = Edge {
                        vertex_id: id,
                        distance: distance,
                    };
                    graph.vertices[frontier_id].connected.push(edge);
                }
            }

            //println!("Graph frontier: {:?}", frontier);
            if frontier.len() == 0 {
                break;
            }
        }

        //println!("Graph: {:#?}", graph);
        graph
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
struct Node {
    point: Point,
    space: Space,
    inventory: u32,
}

impl Node {
    fn idx_from_char(c: char) -> usize {
        if c.is_ascii_lowercase() == false {
            panic!("Invalid character!");
        }

        let idx = (c as u8) - ('a' as u8);
        idx as usize
    }

    fn has_key(&self, c: char) -> bool {
        let idx = Node::idx_from_char(c);
        let key_bit = self.inventory & (1 << idx);
        key_bit != 0
    }

    fn set_key(&mut self, c: char, value: bool) {
        let idx = Node::idx_from_char(c);
        let key_bit = if value == true { 1 } else { 0 };
        self.inventory |= key_bit << idx;
    }
}

#[derive(Debug)]
struct Edge {
    vertex_id: usize,
    distance: u32,
}

#[derive(Debug)]
struct Vertex {
    id: usize, // Not technically needed but useful for debug
    node: Node,
    distance: u32, // This is used purely for Dijkstra's, where u32::max_value() is used to indicate infinity distance
    pi: Option<usize>, // The "parent" or the vertex from which the fastest path comes
    connected: Vec<Edge>,
}

struct Graph {
    vertices: Vec<Vertex>,
}

impl Graph {
    fn dijkstra(&mut self, start_vertex: usize) {
        // Set initial conditions:
        //  - set all distances to infinity except the starting vertex which is set to 0
        //  - set the starting vertex as the first vertex to process
        for vertex in self.vertices.iter_mut() {
            vertex.distance = u32::max_value();
        }
        self.vertices[start_vertex].distance = 0;

        let mut visited: HashSet<usize> = HashSet::new();
        let mut next: HashSet<usize> = HashSet::new();
        next.insert(start_vertex);

        while next.len() > 0 {
            // Find the next vertex with minimum distance. Mark it as visited
            // (we're visiting it now) and remove it from the 'next' list.
            let mut min_id = 0;
            let mut min_value = u32::max_value();
            for &vertex_id in next.iter() {
                if self.vertices[vertex_id].distance < min_value {
                    min_value = self.vertices[vertex_id].distance;
                    min_id = vertex_id;
                }
            }
            let current_vertex_id = min_id;
            visited.insert(current_vertex_id);
            next.remove(&min_id);
            //println!("Dijkstra visit vertex #{}", current_vertex_id);

            // Update distances of all adjacent vertices and add them to the 'next' list.
            // Skip any vertices we've already visited.
            let mut temp: Vec<(usize, u32, Option<usize>)> = Vec::new();
            for e in self.vertices[current_vertex_id].connected.iter() {
                if visited.get(&e.vertex_id) != None {
                    continue;
                }

                let new_distance = self.vertices[current_vertex_id].distance + e.distance;
                let old_distance = self.vertices[e.vertex_id].distance;
                if new_distance < old_distance {
                    temp.push((e.vertex_id, new_distance, Some(current_vertex_id)));
                }

                next.insert(e.vertex_id);
            }

            for (vertex_id, distance, pi) in temp {
                self.vertices[vertex_id].distance = distance;
                self.vertices[vertex_id].pi = pi;
            }
        }
    }

    fn get_shortest_path(&self) -> (u32, Vec<Space>) {
        let mut longest_inventory = 0;
        let mut shortest_distance = u32::max_value();
        let mut shortest_end_vertex_id = 0;
        for vertex in self.vertices.iter() {
            let current_inventory = vertex.node.inventory;
            if current_inventory > longest_inventory {
                longest_inventory = current_inventory;
                shortest_distance = u32::max_value();
                shortest_end_vertex_id = 0;
            }

            if vertex.distance < shortest_distance {
                shortest_distance = vertex.distance;
                shortest_end_vertex_id = vertex.id;
            }
        }

        //println!("End vertex: {:?}", graph[shortest_end_vertex_id]);

        let mut path: Vec<Space> = Vec::new();
        let mut current_vertex_id = shortest_end_vertex_id;
        path.push(self.vertices[current_vertex_id].node.space);
        while let Some(parent_id) = self.vertices[current_vertex_id].pi {
            current_vertex_id = parent_id;
            path.push(self.vertices[current_vertex_id].node.space);
        }
        path.reverse();

        (self.vertices[shortest_end_vertex_id].distance, path)
    }
}

#[aoc(day18, part1)]
pub fn solve(input: &str) -> u32 {
    let map = Map::from_string(&input);
    map.display();

    let mut graph = map.build_graph();

    /*println!("Graph:");
    println!("   {} vertices", graph.vertices.len());
    for (id, vertex) in graph.vertices.iter().enumerate() {
        println!("   Vertex {}:", id);
        println!("      id: {}", vertex.id);
        println!("      node.point: {:?}", vertex.node.point);
        println!("      node.space: {:?}", vertex.node.space);
        println!("      node.inventory: {}", vertex.node.inventory);
        println!("      distance: {}", vertex.distance);
        println!("      pi: {:?}", vertex.pi);
        println!("      edges:");
        for e in vertex.connected.iter() {
            println!("         {:?}", e);
        }
        println!("");
    }*/

    graph.dijkstra(0);

    let (distance, path) = graph.get_shortest_path();
    println!("Distance: {}", distance);
    println!("Path: {:?}", path);
    distance
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_get_shortest_path() {
        let input = "
#########
#b.A.@.a#
#########
";
        let map = Map::from_string(&input);
        let mut graph = map.build_graph();
        graph.dijkstra(0);
        let (distance, path) = graph.get_shortest_path();
        assert_eq!(distance, 8);
        assert_eq!(path, [Space::Entrance,
                          Space::Key('a'),
                          Space::Key('b')]);

        let input = "
########################
#f.D.E.e.C.b.A.@.a.B.c.#
######################.#
#d.....................#
########################
";
        let map = Map::from_string(&input);
        let mut graph = map.build_graph();
        graph.dijkstra(0);
        let (distance, path) = graph.get_shortest_path();
        assert_eq!(distance, 86);
        assert_eq!(path, [Space::Entrance,
                        Space::Key('a'),
                        Space::Key('b'),
                        Space::Key('c'),
                        Space::Key('d'),
                        Space::Key('e'),
                        Space::Key('f')]);

        let input = "
########################
#...............b.C.D.f#
#.######################
#.....@.a.B.c.d.A.e.F.g#
########################
";
        let map = Map::from_string(&input);
        let mut graph = map.build_graph();
        graph.dijkstra(0);
        let (distance, path) = graph.get_shortest_path();
        assert_eq!(distance, 132);
        assert_eq!(path, [Space::Entrance,
                          Space::Key('b'),
                          Space::Key('a'),
                          Space::Key('c'),
                          Space::Key('d'),
                          Space::Key('f'),
                          Space::Key('e'),
                          Space::Key('g')]);

        let input = "
#################
#i.G..c...e..H.p#
########.########
#j.A..b...f..D.o#
########@########
#k.E..a...g..B.n#
########.########
#l.F..d...h..C.m#
#################
";
        let map = Map::from_string(&input);
        let mut graph = map.build_graph();
        graph.dijkstra(0);
        let (distance, _path) = graph.get_shortest_path();
        assert_eq!(distance, 136);
        // There are multiple possible paths, don't verify

let input = "
########################
#@..............ac.GI.b#
###d#e#f################
###A#B#C################
###g#h#i################
########################
";
        let map = Map::from_string(&input);
        let mut graph = map.build_graph();
        graph.dijkstra(0);
        let (distance, _path) = graph.get_shortest_path();
        assert_eq!(distance, 81);
        // There are multiple possible paths, don't verify
    }
}
