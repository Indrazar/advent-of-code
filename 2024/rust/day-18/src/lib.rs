use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Coord {
    x: usize,
    y: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Space {
    Safe,
    Corrupted,
}

fn corrupt_squares(
    incoming: &Vec<Coord>,
    incoming_limit: usize,
    width_max: usize,
    height_max: usize,
) -> (Vec<Vec<Space>>, HashMap<Coord, Space>) {
    if incoming_limit > incoming.len() {
        panic!("not enough bytes incoming");
    }
    let mut bytes: HashMap<Coord, Space> = HashMap::new();
    for i in 0..incoming_limit {
        bytes.insert(incoming[i], Space::Corrupted);
    }
    let mut res: Vec<Vec<Space>> = Vec::with_capacity(height_max + 1);
    for y in 0..=height_max {
        let mut row: Vec<Space> = Vec::with_capacity(width_max + 1);
        for x in 0..=width_max {
            let square_type = match bytes.get(&Coord { x, y }) {
                Some(x) => *x,
                None => Space::Safe,
            };
            row.push(square_type);
        }
        res.push(row);
    }
    (res, bytes)
}

fn parse(input: &str) -> Vec<Coord> {
    let mut res: Vec<Coord> = Vec::new();
    for line in input.lines() {
        let mut line_itr = line.split(",");
        let x: usize = line_itr
            .next()
            .expect("some text before ,")
            .parse()
            .expect("a number");
        let y: usize = line_itr
            .next()
            .expect("some text before ,")
            .parse()
            .expect("a number");
        res.push(Coord { x, y });
    }
    res
}

fn print_map(vec_map: &Vec<Vec<Space>>) {
    println!("current map:");
    for y in 0..vec_map.len() {
        for x in 0..vec_map[0].len() {
            match vec_map[y][x] {
                Space::Safe => {
                    print!(".")
                }
                Space::Corrupted => {
                    print!("#")
                }
            }
        }
        println!()
    }
}

fn flood_with_breadcrumbs(
    vec_map: &Vec<Vec<Space>>,
    current: &Coord,
    cost: usize,
    flood_map: Option<HashMap<Coord, usize>>,
) -> HashMap<Coord, usize> {
    let mut flood_map = match flood_map {
        Some(m) => m,
        None => HashMap::new(),
    };
    match vec_map[current.y][current.x] {
        Space::Safe => {
            //keep going
        }
        Space::Corrupted => return flood_map,
    };
    match flood_map.get(current) {
        Some(val) => {
            if cost < *val {
                flood_map.insert(*current, cost);
            } else {
                // another route was cheaper
                return flood_map;
            }
        }
        None => {
            flood_map.insert(*current, cost);
        }
    }
    // try up
    if current.y > 0 {
        let up = Coord {
            x: current.x,
            y: current.y - 1,
        };
        flood_map = flood_with_breadcrumbs(vec_map, &up, cost + 1, Some(flood_map));
    }
    // try down
    if current.y < vec_map.len() - 1 {
        let down = Coord {
            x: current.x,
            y: current.y + 1,
        };
        flood_map = flood_with_breadcrumbs(vec_map, &down, cost + 1, Some(flood_map));
    }
    // try left
    if current.x > 0 {
        let left = Coord {
            x: current.x - 1,
            y: current.y,
        };
        flood_map = flood_with_breadcrumbs(vec_map, &left, cost + 1, Some(flood_map));
    }
    // try right
    if current.x < vec_map[0].len() - 1 {
        let right = Coord {
            x: current.x + 1,
            y: current.y,
        };
        flood_map = flood_with_breadcrumbs(vec_map, &right, cost + 1, Some(flood_map));
    }

    flood_map
}

pub fn process_part1(input: &str) -> String {
    let coords = parse(input);
    let (vecs, _) = corrupt_squares(&coords, 1024, 70, 70);
    //print_map(&vecs);
    let flood_map = flood_with_breadcrumbs(&vecs, &Coord { x: 0, y: 0 }, 0, None);
    flood_map.get(&Coord { x: 70, y: 70 }).unwrap().to_string()
}

pub fn process_part2(input: &str) -> String {
    let coords = parse(input);
    for fall_count in (1024..coords.len()).rev() {
        let (vecs, _) = corrupt_squares(&coords, fall_count, 70, 70);
        let flood_map = flood_with_breadcrumbs(&vecs, &Coord { x: 0, y: 0 }, 0, None);
        match flood_map.get(&Coord { x: 70, y: 70 }) {
            Some(_) => return format!("{},{}", coords[fall_count].x, coords[fall_count].y),
            None => {
                continue;
            }
        }
    }
    "none found".to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input() {
        let file = include_str!("../test-input-1.txt");
        let coords = parse(file);
        let (vecs, _) = corrupt_squares(&coords, 12, 6, 6);
        //print_map(&vecs);
        let flood_map = flood_with_breadcrumbs(&vecs, &Coord { x: 0, y: 0 }, 0, None);
        assert_eq!(
            flood_map.get(&Coord { x: 6, y: 6 }).unwrap().to_string(),
            "22"
        );
    }
    #[test]
    fn test2_input() {
        let file = include_str!("../test-input-1.txt");
        let coords = parse(file);
        for fall_count in (1024..coords.len()).rev() {
            let (vecs, _) = corrupt_squares(&coords, fall_count, 70, 70);
            let flood_map = flood_with_breadcrumbs(&vecs, &Coord { x: 0, y: 0 }, 0, None);
            match flood_map.get(&Coord { x: 70, y: 70 }) {
                Some(_) => {
                    assert_eq!(
                        format!("{},{}", coords[fall_count - 1].x, coords[fall_count - 1].y),
                        "6,1",
                    );
                }
                None => {
                    continue;
                }
            }
        }
    }
}
