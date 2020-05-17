/*
    --- Part Two ---
    You arrive at the vault only to discover that there is not one vault, but four - each with its own entrance.

    On your map, find the area in the middle that looks like this:

    ...
    .@.
    ...
    Update your map to instead use the correct data:

    @#@
    ###
    @#@
    This change will split your map into four separate sections, each with its own entrance:

    #######       #######
    #a.#Cd#       #a.#Cd#
    ##...##       ##@#@##
    ##.@.##  -->  #######
    ##...##       ##@#@##
    #cB#Ab#       #cB#Ab#
    #######       #######
    Because some of the keys are for doors in other vaults, it would take much too long to collect all of the keys by yourself. Instead, you deploy four remote-controlled robots. Each starts at one of the entrances (@).

    Your goal is still to collect all of the keys in the fewest steps, but now, each robot has its own position and can move independently. You can only remotely control a single robot at a time. Collecting a key instantly unlocks any corresponding doors, regardless of the vault in which the key or door is found.

    For example, in the map above, the top-left robot first collects key a, unlocking door A in the bottom-right vault:

    #######
    #@.#Cd#
    ##.#@##
    #######
    ##@#@##
    #cB#.b#
    #######
    Then, the bottom-right robot collects key b, unlocking door B in the bottom-left vault:

    #######
    #@.#Cd#
    ##.#@##
    #######
    ##@#.##
    #c.#.@#
    #######
    Then, the bottom-left robot collects key c:

    #######
    #@.#.d#
    ##.#@##
    #######
    ##.#.##
    #@.#.@#
    #######
    Finally, the top-right robot collects key d:

    #######
    #@.#.@#
    ##.#.##
    #######
    ##.#.##
    #@.#.@#
    #######
    In this example, it only took 8 steps to collect all of the keys.

    Sometimes, multiple robots might have keys available, or a robot might have to wait for multiple keys to be collected:

    ###############
    #d.ABC.#.....a#
    ######@#@######
    ###############
    ######@#@######
    #b.....#.....c#
    ###############
    First, the top-right, bottom-left, and bottom-right robots take turns collecting keys a, b, and c, a total of 6 + 6 + 6 = 18 steps. Then, the top-left robot can access key d, spending another 6 steps; collecting all of the keys here takes a minimum of 24 steps.

    Here's a more complex example:

    #############
    #DcBa.#.GhKl#
    #.###@#@#I###
    #e#d#####j#k#
    ###C#@#@###J#
    #fEbA.#.FgHi#
    #############
    Top-left robot collects key a.
    Bottom-left robot collects key b.
    Top-left robot collects key c.
    Bottom-left robot collects key d.
    Top-left robot collects key e.
    Bottom-left robot collects key f.
    Bottom-right robot collects key g.
    Top-right robot collects key h.
    Bottom-right robot collects key i.
    Top-right robot collects key j.
    Bottom-right robot collects key k.
    Top-right robot collects key l.
    In the above example, the fewest steps to collect all of the keys is 32.

    Here's an example with more choices:

    #############
    #g#f.D#..h#l#
    #F###e#E###.#
    #dCba@#@BcIJ#
    #############
    #nK.L@#@G...#
    #M###N#H###.#
    #o#m..#i#jk.#
    #############
    One solution with the fewest steps is:

    Top-left robot collects key e.
    Top-right robot collects key h.
    Bottom-right robot collects key i.
    Top-left robot collects key a.
    Top-left robot collects key b.
    Top-right robot collects key c.
    Top-left robot collects key d.
    Top-left robot collects key f.
    Top-left robot collects key g.
    Bottom-right robot collects key k.
    Bottom-right robot collects key j.
    Top-right robot collects key l.
    Bottom-left robot collects key n.
    Bottom-left robot collects key m.
    Bottom-left robot collects key o.
    This example requires at least 72 steps to collect all keys.

    After updating your map and using the remote-controlled robots, what is the fewest steps necessary to collect all of the keys?
*/

use std::cmp::Ordering;
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
        for c in input.chars() {
            let space = Space::from_value(c);
            if space != Space::Unknown {
                area.insert(p, space);
                p.x += 1;
            } else {
                if c == '\n' {
                    p.x = 0;
                    p.y += 1;
                } else {
                    println!("Unknown input: 0x{:02x}", c as u8);
                }
            }
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

        Point {
            x: 0,
            y: 0,
        }
    }

    fn get_entrances(&self) -> Vec<Point> {
        let mut entrances = Vec::new();
        for (&k, &v) in self.area.iter() {
            if v == Space::Entrance {
                entrances.push(k);
            }
        }

        // Put the list of entrances in order from top to bottom, left to right.
        // This makes the path results predictable and testable but is not strictly required.
        entrances.sort_by(
            |a, b| {
                let primary = a.y.cmp(&b.y);
                match primary {
                    Ordering::Equal => a.x.cmp(&b.x),
                    _ => primary,
                }
            });
        entrances
    }

    fn split_entrance(&mut self, entrance: &Point) {
        let offsets = vec![(( 0,  0), Space::Wall),
                           ((-1, -1), Space::Entrance),
                           ((-1,  0), Space::Wall),
                           ((-1,  1), Space::Entrance),
                           (( 0,  1), Space::Wall),
                           (( 1,  1), Space::Entrance),
                           (( 1,  0), Space::Wall),
                           (( 1, -1), Space::Entrance),
                           (( 0, -1), Space::Wall)];

        for ((x, y), s) in offsets {
            let p = Point {
                x: entrance.x + x,
                y: entrance.y + y,
            };
            self.area.insert(p, s);
        }
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
        // Attach all other entrances to the starting vertex
        let entrances = self.get_entrances();
        let mut starting_vertex = Vertex {
            id: 0,
            nodes: Vec::new(),
            distance: 0,
            pi: None,
            connected: Vec::new(),
        };
        for e in entrances {
            let n = Node {
                point: e,
                space: Space::Entrance,
                inventory: 0,
            };
            starting_vertex.nodes.push(n);
        }

        let mut node_to_vertex_map: HashMap<Vec<Node>, usize> = HashMap::new();
        node_to_vertex_map.insert(starting_vertex.nodes.clone(), starting_vertex.id);

        let mut graph = Graph {
            vertices: Vec::new(),
        };
        graph.vertices.push(starting_vertex);

        let mut frontier: Vec<usize> = Vec::new();
        frontier.push(0); // Visit the starting vertex first

        loop {
            for frontier_id in frontier.drain(..).collect::<Vec<usize>>() {
                for node_idx in 0..graph.vertices[frontier_id].nodes.len() {
                    let connected_nodes = self.find_keys(&graph.vertices[frontier_id].nodes[node_idx]);
                    /*println!("Nodes:");
                    for (node, distance) in &connected_nodes {
                        println!("   {:?} @ {}", node, distance);
                    }*/

                    // Check if any of these connected nodes are new. If they are, create them as vertices in the graph.
                    for (node, distance) in connected_nodes {
                        // Create a new nodes array. The current node is replaced, and also every other node
                        // must have its inventory updated (all inventories are just copies of each other for laziness reasons).
                        let mut nodes = graph.vertices[frontier_id].nodes.clone();
                        nodes[node_idx] = node;
                        for n in &mut nodes {
                            n.inventory = node.inventory;
                        }

                        let id;
                        if let Some(&idx) = node_to_vertex_map.get(&nodes) {
                            // This one exists. Save its ID.
                            id = idx;
                        } else {
                            // This one doesn't exist. Create it and save its ID.
                            id = graph.vertices.len();
                            node_to_vertex_map.insert(nodes.clone(), id);

                            let v = Vertex {
                                id: id,
                                nodes: nodes,
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
    nodes: Vec<Node>,
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

    fn get_shortest_path(&self) -> (u32, Vec<Vec<Space>>) {
        let mut longest_inventory = 0;
        let mut shortest_distance = u32::max_value();
        let mut shortest_end_vertex_id = 0;
        for vertex in self.vertices.iter() {
            let current_inventory = vertex.nodes[0].inventory; // All inventories are guaranteed to be the same; just use the first one.
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

        let mut path: Vec<Vec<Space>> = Vec::new();
        let mut current_vertex_id = shortest_end_vertex_id;
        let spaces: Vec<Space> = self.vertices[current_vertex_id].nodes.iter().map(|n| n.space).collect();
        path.push(spaces);
        while let Some(parent_id) = self.vertices[current_vertex_id].pi {
            current_vertex_id = parent_id;
            let spaces: Vec<Space> = self.vertices[current_vertex_id].nodes.iter().map(|n| n.space).collect();
            path.push(spaces);
        }
        path.reverse();

        (self.vertices[shortest_end_vertex_id].distance, path)
    }
}

#[aoc(day18, part2)]
pub fn solve(input: &str) -> u32 {
    let mut map = Map::from_string(&input);
    // Replace the entrance with a specific pattern. This is a requirement for part 2.
    let entrance = map.get_entrance();
    map.split_entrance(&entrance);
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
    println!("Path:");
    println!("    Start: {:?}", path[0]);
    for path_idx in 0..path.len() {
        // Check what changed between this index and the previous one (if any) and print it
        if path_idx > 0 {
            let mut zip_iter = path[path_idx].iter().zip(path[path_idx - 1].iter());
            while let Some((&curr, &prev)) = zip_iter.next() {
                if curr != prev {
                    println!("    {:?}", curr);
                }
            }
        }
    }
    distance
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_get_shortest_path() {
        let input = "
#######
#a.#Cd#
##...##
##.@.##
##...##
#cB#Ab#
#######
";
        let mut map = Map::from_string(&input);
        let entrance = map.get_entrance();
        map.split_entrance(&entrance);
        let mut graph = map.build_graph();
        graph.dijkstra(0);
        let (distance, path) = graph.get_shortest_path();
        assert_eq!(distance, 8);
        assert_eq!(path, [[Space::Entrance, Space::Entrance, Space::Entrance, Space::Entrance],
                          [Space::Key('a'), Space::Entrance, Space::Entrance, Space::Entrance],
                          [Space::Key('a'), Space::Entrance, Space::Entrance, Space::Key('b')],
                          [Space::Key('a'), Space::Entrance, Space::Key('c'), Space::Key('b')],
                          [Space::Key('a'), Space::Key('d'), Space::Key('c'), Space::Key('b')]]);

        let input = "
###############
#d.ABC.#.....a#
######...######
######.@.######
######...######
#b.....#.....c#
###############
";
        let mut map = Map::from_string(&input);
        let entrance = map.get_entrance();
        map.split_entrance(&entrance);
        let mut graph = map.build_graph();
        graph.dijkstra(0);
        let (distance, _path) = graph.get_shortest_path();
        assert_eq!(distance, 24);
        // There are multiple possible paths, don't verify

        let input = "
#############
#DcBa.#.GhKl#
#.###...#I###
#e#d#.@.#j#k#
###C#...###J#
#fEbA.#.FgHi#
#############
";
        let mut map = Map::from_string(&input);
        let entrance = map.get_entrance();
        map.split_entrance(&entrance);
        let mut graph = map.build_graph();
        graph.dijkstra(0);
        let (distance, path) = graph.get_shortest_path();
        assert_eq!(distance, 32);
        assert_eq!(path, [[Space::Entrance, Space::Entrance, Space::Entrance, Space::Entrance],
                          [Space::Key('a'), Space::Entrance, Space::Entrance, Space::Entrance],
                          [Space::Key('a'), Space::Entrance, Space::Key('b'), Space::Entrance],
                          [Space::Key('c'), Space::Entrance, Space::Key('b'), Space::Entrance],
                          [Space::Key('c'), Space::Entrance, Space::Key('d'), Space::Entrance],
                          [Space::Key('e'), Space::Entrance, Space::Key('d'), Space::Entrance],
                          [Space::Key('e'), Space::Entrance, Space::Key('f'), Space::Entrance],
                          [Space::Key('e'), Space::Entrance, Space::Key('f'), Space::Key('g')],
                          [Space::Key('e'), Space::Key('h'), Space::Key('f'), Space::Key('g')],
                          [Space::Key('e'), Space::Key('h'), Space::Key('f'), Space::Key('i')],
                          [Space::Key('e'), Space::Key('j'), Space::Key('f'), Space::Key('i')],
                          [Space::Key('e'), Space::Key('j'), Space::Key('f'), Space::Key('k')],
                          [Space::Key('e'), Space::Key('l'), Space::Key('f'), Space::Key('k')],
                          ]);

        let input = "
#############
#g#f.D#..h#l#
#F###e#E###.#
#dCba...BcIJ#
#####.@.#####
#nK.L...G...#
#M###N#H###.#
#o#m..#i#jk.#
#############
";
        let mut map = Map::from_string(&input);
        let entrance = map.get_entrance();
        map.split_entrance(&entrance);
        let mut graph = map.build_graph();
        graph.dijkstra(0);
        let (distance, _path) = graph.get_shortest_path();
        assert_eq!(distance, 72);
        // There are multiple possible paths, don't verify
    }
}
