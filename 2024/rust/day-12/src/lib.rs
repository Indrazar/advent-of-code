use std::{collections::HashMap, iter::zip};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Coord {
    x: i64,
    y: i64,
}

impl Coord {
    fn from(x: usize, y: usize) -> Coord {
        Coord {
            x: x.try_into().unwrap(),
            y: y.try_into().unwrap(),
        }
    }
}

#[derive(Debug, Clone)]
struct MapData {
    map: HashMap<Coord, char>,
    letters: HashMap<char, Vec<Coord>>,
    height: i64,
    width: i64,
}

fn parse(input: &str) -> MapData {
    let mut map: HashMap<Coord, char> = HashMap::new();
    let mut letters: HashMap<char, Vec<Coord>> = HashMap::new();
    let mut height: usize = 0;
    let mut width: usize = 0;
    for (y, line) in input.lines().enumerate() {
        for (x, ch) in line.chars().enumerate() {
            map.insert(Coord::from(x, y), ch);
            match letters.get_mut(&ch) {
                Some(l) => {
                    l.push(Coord::from(x, y));
                }
                None => {
                    letters.insert(ch, vec![Coord::from(x, y)]);
                }
            }
            if y == 0 {
                width += 1;
            }
        }
        height += 1;
    }
    MapData {
        map,
        letters,
        height: height.try_into().unwrap(),
        width: width.try_into().unwrap(),
    }
}

fn cardnial_flood(
    start: Coord,
    letter: char,
    map: &MapData,
    flood_map: &mut HashMap<Coord, bool>,
    perimeter_count: &mut usize,
) {
    match map.map.get(&start) {
        Some(ch) => {
            if *ch != letter {
                *perimeter_count += 1;
                return;
            }
        }
        None => panic!(
            "should not get here:\nmap_width: {}\nmap_height: {}\nstart: {start:?}\nletter: {letter}\nmap: {map:?}\nflood_map: {flood_map:?}\nperimeter: {perimeter_count}",
        map.width,
        map.height
        ),
    }
    match flood_map.get(&start) {
        Some(flood) => {
            if *flood {
                return;
            }
        }
        None => {
            flood_map.insert(start, true);
            // flood up, flood down, flood right, flood left
            if start.y > 0 {
                let new_start = Coord {
                    x: start.x,
                    y: start.y - 1,
                };
                cardnial_flood(new_start, letter, map, flood_map, perimeter_count);
            } else {
                *perimeter_count += 1;
            }
            if start.y < map.height - 1 {
                let new_start = Coord {
                    x: start.x,
                    y: start.y + 1,
                };
                cardnial_flood(new_start, letter, map, flood_map, perimeter_count);
            } else {
                *perimeter_count += 1;
            }
            if start.x > 0 {
                let new_start = Coord {
                    x: start.x - 1,
                    y: start.y,
                };
                cardnial_flood(new_start, letter, map, flood_map, perimeter_count);
            } else {
                *perimeter_count += 1;
            }
            if start.x < map.width - 1 {
                let new_start = Coord {
                    x: start.x + 1,
                    y: start.y,
                };
                cardnial_flood(new_start, letter, map, flood_map, perimeter_count);
            } else {
                *perimeter_count += 1;
            }
        }
    }
}

fn prune_list(input: Vec<Coord>, remove_these: &Vec<Coord>) -> Vec<Coord> {
    let mut new_list = Vec::new();
    for item in input {
        if !remove_these.contains(&item) {
            new_list.push(item);
        }
    }
    new_list
}

fn determine_cost_p1(map: &MapData) -> usize {
    let mut sum = 0;
    for ch in map.letters.keys() {
        // get all coords for a letter
        let mut letter_list = map.letters.get(&ch).unwrap().clone();
        //println!("letter: {ch}, locations: {letter_list:?}");
        while !letter_list.is_empty() {
            // split into regions plots with same letter but not touching
            let mut flood_map: HashMap<Coord, bool> = HashMap::new();
            let mut perimeter: usize = 0;
            cardnial_flood(letter_list[0], *ch, map, &mut flood_map, &mut perimeter);
            let region: Vec<Coord> = flood_map.keys().map(|x| *x).collect();
            letter_list = prune_list(letter_list, &region);
            //println!(
            //    "letter: {ch}, region area: {}, preimeter: {perimeter:?}",
            //    region.len()
            //);
            //region_list.push(region);
            //perimeter_list.push(perimeter);
            sum += region.len() * perimeter;
        }
    }
    sum
}

fn print_region(input: &Vec<Coord>, input_ch: &char, map: &MapData) {
    println!("{input_ch} region");
    for y in 0..map.height {
        for x in 0..map.width {
            if input.contains(&Coord { x, y }) {
                print!("{input_ch}");
            } else {
                print!(".");
            }
        }
        println!();
    }
}

fn neighbors(input: &Coord) -> (Coord, Coord, Coord, Coord) {
    (
        //up
        Coord {
            x: input.x,
            y: input.y - 1,
        },
        // down
        Coord {
            x: input.x,
            y: input.y + 1,
        },
        // left
        Coord {
            x: input.x - 1,
            y: input.y,
        },
        // right
        Coord {
            x: input.x + 1,
            y: input.y,
        },
    )
}

fn diags(input: &Coord) -> (Coord, Coord, Coord, Coord) {
    (
        // up left
        Coord {
            x: input.x - 1,
            y: input.y - 1,
        },
        // up right
        Coord {
            x: input.x + 1,
            y: input.y - 1,
        },
        // down left
        Coord {
            x: input.x - 1,
            y: input.y + 1,
        },
        // down right
        Coord {
            x: input.x + 1,
            y: input.y + 1,
        },
    )
}

fn count_sides(input: &Vec<Coord>) -> usize {
    if input.is_empty() {
        panic!("should not be called on an empty region")
    }
    // all in a row: 4 sides
    let same_y = input[0].y;
    if !input.iter().any(|c| c.y != same_y) {
        return 4;
    }
    // all in a column: 4 sides
    let same_x = input[0].x;
    if !input.iter().any(|c| c.x != same_x) {
        return 4;
    }
    //not all in a row or column, time to count corners
    let mut corners = 0;
    for start in input {
        let (n1, n2, n3, n4) = neighbors(start);
        let up = input.contains(&n1);
        let down = input.contains(&n2);
        let left = input.contains(&n3);
        let right = input.contains(&n4);
        // exterior corners
        //print!("checking: ({},{}); {input:?}\n", start.x, start.y);
        // top right
        if !up && !right {
            //print!("top right exterior corner, ");
            corners += 1;
        }
        // top left
        if !up && !left {
            //print!("top left exterior corner, ");
            corners += 1;
        }
        // bottom right
        if !down && !right {
            //print!("bottom right exterior corner, ");
            corners += 1;
        }
        // bottom left
        //print!("bottom left exterior corner, ");
        if !down && !left {
            corners += 1;
        }
        // interior corners
        let (d1, d2, d3, d4) = diags(start);
        let upleft = input.contains(&d1);
        let upright = input.contains(&d2);
        let downleft = input.contains(&d3);
        let downright = input.contains(&d4);
        // up left
        if up && left && !upleft {
            //print!("diag upleft interior corner, ");
            corners += 1;
        }
        // up right
        if up && right && !upright {
            //print!("diag upright interior corner, ");
            corners += 1;
        }
        // down left
        if down && left && !downleft {
            //print!("diag downleft interior corner, ");
            corners += 1;
        }
        // down right
        if down && right && !downright {
            //print!("diag downright interior corner, ");
            corners += 1;
        }
        //println!();
    }
    corners
}

fn determine_cost_p2(map: &MapData) -> usize {
    let mut sum = 0;
    for ch in map.letters.keys() {
        // get all coords for a letter
        let mut letter_list = map.letters.get(&ch).unwrap().clone();
        //println!("letter: {ch}, locations: {letter_list:?}");
        while !letter_list.is_empty() {
            // split into regions plots with same letter but not touching
            let mut flood_map: HashMap<Coord, bool> = HashMap::new();
            let mut perimeter: usize = 0;
            cardnial_flood(letter_list[0], *ch, map, &mut flood_map, &mut perimeter);
            let region: Vec<Coord> = flood_map.keys().map(|x| *x).collect();
            letter_list = prune_list(letter_list, &region);
            let sides = count_sides(&region);
            //print_region(&region, &ch, map);
            //println!(
            //    "letter: {ch}, region area: {}, perimeter: {perimeter:?}, sides: {sides}, cost: {}\n",
            //    region.len(),
            //    region.len() * sides
            //);
            sum += region.len() * sides;
        }
    }
    sum
}

pub fn process_part1(input: &str) -> String {
    let data = parse(input);
    determine_cost_p1(&data).to_string()
}

pub fn process_part2(input: &str) -> String {
    let data = parse(input);
    determine_cost_p2(&data).to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input_negative_1() {
        let file = include_str!("../test-input--1.txt");
        assert_eq!(process_part1(file), "140");
        assert_eq!(process_part2(file), "80");
    }
    #[test]
    fn test_input0() {
        let file = include_str!("../test-input-0.txt");
        assert_eq!(process_part1(file), "772");
        assert_eq!(process_part2(file), "436");
    }
    #[test]
    fn test_input() {
        let file = include_str!("../test-input-1.txt");
        assert_eq!(process_part1(file), "1930");
        assert_eq!(process_part2(file), "1206");
    }
    #[test]
    fn test_input2() {
        let file = include_str!("../test-input-2.txt");
        assert_eq!(process_part2(file), "236")
    }
    #[test]
    fn test_input3() {
        let file = include_str!("../test-input-3.txt");
        assert_eq!(process_part2(file), "368")
    }
}
