use kd_tree::{KdPoint, KdTree};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
struct Coord {
    x: i64,
    y: i64,
    z: i64,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
struct Node {
    id: usize,
    loc: Coord,
    connections: Vec<usize>,
    group_id: Option<usize>,
}

impl KdPoint for Node {
    type Scalar = i64;
    type Dim = typenum::U3;
    fn at(&self, k: usize) -> i64 {
        match k {
            0 => self.loc.x,
            1 => self.loc.y,
            2 => self.loc.z,
            _ => panic!("asked for higher dimension: {k}"),
        }
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
struct Connection {
    sq_dist: i64,
    left: usize,
    right: usize,
}

struct NodeList {
    all: Vec<Node>,
    shortest: Vec<Connection>,
    group_size: Vec<usize>,
    connection_index: usize,
}

impl NodeList {
    pub fn new(input: Vec<Node>, nearests: usize) -> NodeList {
        let all = input.clone();
        let kdtree: KdTree<Node> = KdTree::par_build(input);
        let mut shortest: Vec<Connection> = Vec::new();
        for node in &all {
            let start_id = node.id;
            let nearest_data = kdtree.nearests(node, nearests);
            for data in nearest_data.iter().skip(1) {
                let sq_dist = data.squared_distance;
                let nearest_id = data.item.id;
                if shortest.contains(&Connection {
                    sq_dist,
                    left: start_id,
                    right: nearest_id,
                }) || shortest.contains(&Connection {
                    sq_dist,
                    left: nearest_id,
                    right: start_id,
                }) {
                    // skip it
                } else {
                    shortest.push(Connection {
                        sq_dist,
                        left: start_id,
                        right: nearest_id,
                    });
                }
            }
        }
        shortest.sort_by(|a, b| a.sq_dist.cmp(&b.sq_dist));
        //println!("shortest list len: {}", shortest.len());
        NodeList {
            all,
            shortest,
            group_size: Vec::new(),
            connection_index: 0,
        }
    }
    fn build_all_connections(&mut self, number: usize) {
        if number == 0 {
            return;
        }
        for x in 0..number {
            //println!("running connection {x}");
            let candidate = self.shortest[x];
            let left = candidate.left;
            let right = candidate.right;
            // println!(
            //     "connecting: {:#?} and {:#?}",
            //     self.all[left].loc, self.all[right].loc
            // );
            // connect left
            self.all[left].connections.push(right);
            // connect right
            self.all[right].connections.push(left);
        }
        self.connection_index = number;
    }
    fn build_next_n_connections(&mut self, number: usize) {
        if number == 0 {
            return;
        }
        for x in self.connection_index..(number + self.connection_index) {
            //println!("running connection {x}");
            if x >= self.shortest.len() {
                panic!("ran out of possible connections, try starting with more");
            }
            let candidate = self.shortest[x];
            let left = candidate.left;
            let right = candidate.right;
            // println!(
            //     "connecting: {:#?} and {:#?}",
            //     self.all[left].loc, self.all[right].loc
            // );
            // connect left
            self.all[left].connections.push(right);
            // connect right
            self.all[right].connections.push(left);
        }
        self.connection_index += number;
    }
    fn build_one_connection(&mut self) -> (Coord, Coord) {
        //println!("running connection {x}");
        if self.connection_index >= self.shortest.len() {
            panic!("ran out of possible connections, try starting with more");
        }
        let candidate = self.shortest[self.connection_index];
        let left = candidate.left;
        let right = candidate.right;
        // println!(
        //     "connecting: {:#?} and {:#?}",
        //     self.all[left].loc, self.all[right].loc
        // );
        // connect left
        self.all[left].connections.push(right);
        // connect right
        self.all[right].connections.push(left);
        self.connection_index += 1;
        (self.all[left].loc, self.all[right].loc)
    }
    fn flood(&self, start: usize, flood_map: &mut Vec<bool>) {
        if !flood_map[start] {
            flood_map[start] = true;
            for connection in &self.all[start].connections {
                self.flood(*connection, flood_map);
            }
        }
    }
    fn tag_groups(&mut self) {
        let node_total = self.len();
        let mut flood_map = Vec::with_capacity(node_total);
        flood_map.resize(node_total, false);
        let mut group_index = 0;
        for id in 0..node_total {
            if self.all[id].group_id.is_none() {
                // floodfill that target
                self.flood(id, &mut flood_map);
                let mut group_size = 0;
                for (check_id, check) in flood_map.iter().enumerate().take(node_total) {
                    if *check {
                        self.all[check_id].group_id = Some(group_index);
                        group_size += 1;
                    }
                }
                assert_eq!(self.group_size.len(), group_index);
                self.group_size.push(group_size);
                // reset fill
                flood_map.clear();
                flood_map.resize(self.len(), false);
                group_index += 1;
            }
        }
        // for (id, group) in self.group_size.iter().enumerate() {
        //     println!("group {id} contains {group} members");
        //     if *group == 0 {
        //         panic!("***** had 0 members??");
        //     }
        // }
    }
    fn quick_fill(&self) -> bool {
        let node_total = self.len();
        let mut flood_map = Vec::with_capacity(node_total);
        flood_map.resize(node_total, false);
        // floodfill node 0
        self.flood(0, &mut flood_map);
        let mut group_size: usize = 0;
        for check in flood_map.iter().take(node_total) {
            if *check {
                group_size += 1;
            }
        }
        group_size == self.all.len()
    }
    // fn clear_groups(&mut self) {
    //     self.group_size.clear();
    //     for node in self.all.iter_mut() {
    //         node.group_id = None;
    //     }
    // }
    fn len(&self) -> usize {
        self.all.len()
    }
}

fn build_node_vec(input: &str) -> Vec<Node> {
    let mut list: Vec<Node> = Vec::new();
    for (current_id, line) in input.lines().enumerate() {
        let mut itr = line.splitn(3, ',');
        let x: i64 = itr
            .next()
            .expect("there should be first")
            .parse()
            .expect("x must be an int");
        let y: i64 = itr
            .next()
            .expect("there should be second")
            .parse()
            .expect("y must be an int");
        let z: i64 = itr
            .next()
            .expect("there should be third")
            .parse()
            .expect("z must be an int");
        list.push(Node {
            id: current_id,
            loc: Coord { x, y, z },
            connections: Vec::new(),
            group_id: None,
        });
    }
    list
}

pub fn process_part1(input: &str) -> String {
    let list: Vec<Node> = build_node_vec(input);
    let connections: usize = {
        if list.len() == 20 {
            10
        } else {
            1000
        }
    };
    let mut nodes = NodeList::new(list, 15); // 8 just barely works. First time I ran this I set it to 100
    nodes.build_all_connections(connections);
    //println!("nodes: {}, connections: {connections}", list.len());
    nodes.tag_groups();
    let mut groups = nodes.group_size.clone();
    groups.sort();
    groups.reverse();
    (groups[0] * groups[1] * groups[2]).to_string()
}

pub fn process_part2(input: &str) -> String {
    let list: Vec<Node> = build_node_vec(input);
    let starting_connections: usize = {
        if list.len() == 20 {
            10
        } else {
            1000
        }
    };
    let mut nodes = NodeList::new(list, 15); // 4 just barely works. First time I ran this I set it to 100
    nodes.build_next_n_connections(starting_connections);
    let (mut left, mut right) = nodes.build_one_connection();
    while !nodes.quick_fill() {
        (left, right) = nodes.build_one_connection();
    }
    // println!(
    //     "possible connections generated: {}\nconnections made: {}",
    //     nodes.shortest.len(),
    //     nodes.connection_index
    // );
    (left.x * right.x).to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input() {
        let file = include_str!("../test-input-1.txt");
        assert_eq!(process_part1(file), "40");
        assert_eq!(process_part2(file), "25272");
    }
    #[test]
    fn test_real() {
        let real_file = include_str!("../input.txt");
        assert_eq!(process_part1(real_file), "96672");
        assert_eq!(process_part2(real_file), "22517595");
    }
}
