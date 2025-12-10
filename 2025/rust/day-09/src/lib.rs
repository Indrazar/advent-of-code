use std::collections::{BTreeMap, HashMap, VecDeque};

use itertools::Itertools;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Ord, PartialOrd, Hash)]
struct Coord {
    x: usize,
    y: usize,
}

enum Tile {
    Green,
    Red,
    Neither,
}

impl Coord {
    fn _square_distance(&self, b: &Coord) -> usize {
        if self.x > b.x {
            if self.y > b.y {
                ((self.x - b.x) * (self.x - b.x)) + ((self.y - b.y) * (self.y - b.y))
            } else {
                ((self.x - b.x) * (self.x - b.x)) + ((b.y - self.y) * (b.y - self.y))
            }
        } else {
            if self.y > b.y {
                ((b.x - self.x) * (b.x - self.x)) + ((self.y - b.y) * (self.y - b.y))
            } else {
                ((b.x - self.x) * (b.x - self.x)) + ((b.y - self.y) * (b.y - self.y))
            }
        }
    }
    fn area(&self, b: &Coord) -> usize {
        if self.x > b.x {
            if self.y > b.y {
                (self.x - b.x + 1) * (self.y - b.y + 1)
            } else {
                (self.x - b.x + 1) * (b.y - self.y + 1)
            }
        } else {
            if self.y > b.y {
                (b.x - self.x + 1) * (self.y - b.y + 1)
            } else {
                (b.x - self.x + 1) * (b.y - self.y + 1)
            }
        }
    }
    fn left(&self) -> Option<Coord> {
        if self.x == 0 {
            None
        } else {
            Some(Coord {
                x: self.x - 1,
                y: self.y,
            })
        }
    }
    fn right(&self) -> Option<Coord> {
        if self.x == usize::MAX {
            None
        } else {
            Some(Coord {
                x: self.x + 1,
                y: self.y,
            })
        }
    }
    fn up(&self) -> Option<Coord> {
        if self.y == 0 {
            None
        } else {
            Some(Coord {
                x: self.x,
                y: self.y - 1,
            })
        }
    }
    fn down(&self) -> Option<Coord> {
        if self.y == usize::MAX {
            None
        } else {
            Some(Coord {
                x: self.x,
                y: self.y + 1,
            })
        }
    }
}

fn get_simple_map(input: &str) -> Vec<Coord> {
    input
        .lines()
        .map(|line| line.split_once(",").expect("line should contain 2 coords"))
        .map(|(x, y)| {
            let x = x.parse().expect("must be a number");
            let y = y.parse().expect("must be a number");
            Coord { x, y }
        })
        .collect()
}

fn get_greens(start: &Coord, end: &Coord) -> Vec<Coord> {
    let mut green_tiles: Vec<Coord> = Vec::new();
    if start.x == end.x {
        //going up or down
        if start.y > end.y {
            // down
            for y in end.y + 1..start.y {
                green_tiles.push(Coord { x: start.x, y });
            }
        } else {
            //up
            for y in start.y + 1..end.y {
                green_tiles.push(Coord { x: start.x, y });
            }
        }
    } else if start.y == end.y {
        //going left or right
        if start.x > end.x {
            // left
            for x in end.x + 1..start.x {
                green_tiles.push(Coord { x, y: start.y })
            }
        } else {
            // right
            for x in start.x + 1..end.x {
                green_tiles.push(Coord { x, y: start.y })
            }
        }
    } else {
        panic!("what the hell is a straight line?")
    }
    green_tiles
}

struct Limits {
    width: usize,
    height: usize,
    min_width: usize,
    min_height: usize,
}

struct Map {
    //limits: Limits,
    red_tiles: Vec<Coord>,
    map: HashMap<Coord, Tile>,
}

fn floodfill(map: &mut HashMap<Coord, Tile>, start: Coord, limits: &Limits) {
    // println!("flood start: {},{}", start.x, start.y);
    let mut flood_queue: VecDeque<Coord> = VecDeque::new();
    flood_queue.push_back(start);
    while !flood_queue.is_empty() {
        let current = flood_queue
            .pop_front()
            .expect("there should be something, we just checked!");
        if current.x > limits.width || current.x < limits.min_width {
            // println!(
            //     "warning, flood left region: x: {}, {}-{}",
            //     current.x, limits.min_width, limits.width
            // );
            continue;
        }
        if current.y > limits.height || current.y < limits.min_height {
            // println!(
            //     "warning, flood left region: y: {}, {}-{}",
            //     current.y, limits.min_height, limits.height
            // );
            continue;
        }
        // first flood here
        if !map.contains_key(&current) {
            map.insert(current, Tile::Green);
            // println!("flooding: {},{}", current.x, current.y);
            // up
            let next = current.up().expect("should be able to go up");
            if map.get(&next).is_none() {
                flood_queue.push_back(next);
            }
            // down
            let next = current.down().expect("should be able to go down");
            if map.get(&next).is_none() {
                flood_queue.push_back(next);
            }
            // left
            let next = current.left().expect("should be able to go left");
            if map.get(&next).is_none() {
                flood_queue.push_back(next);
            }
            // right
            let next = current.right().expect("should be able to go right");
            if map.get(&next).is_none() {
                flood_queue.push_back(next);
            }
        }
        // println!("current flood_queue: {:?}", flood_queue)
    }
    println!("flood complete");
}

fn get_full_map(input: &str) -> Map {
    let mut width = 0;
    let mut height = 0;
    let mut min_width = usize::MAX;
    let mut min_height = usize::MAX;
    let mut map = HashMap::new();
    let red_tiles: Vec<Coord> = input
        .lines()
        .map(|line| line.split_once(",").expect("line should contain 2 coords"))
        .map(|(x, y)| {
            let x = x.parse().expect("must be a number");
            let y = y.parse().expect("must be a number");
            if width < x {
                width = x;
            }
            if height < y {
                height = y;
            }
            if min_width > x {
                min_width = x;
            }
            if min_height > y {
                min_height = y;
            }
            Coord { x, y }
        })
        .collect();
    let mut green_tiles: Vec<Coord> = Vec::new();
    for (start, end) in red_tiles.iter().tuple_windows() {
        green_tiles.extend(get_greens(start, end).iter());
    }
    let final_greens = get_greens(
        red_tiles.last().expect("there must be an end"),
        red_tiles.first().expect("there must be a start"),
    );
    // println!(
    //     "first green: {},{}",
    //     green_tiles[0].x - min_width,
    //     green_tiles[0].y - min_height
    // );
    // print!("final greens: ");
    // for green in &final_greens {
    //     print!("{},{} ", green.x - min_width, green.y - min_height)
    // }
    // println!();
    green_tiles.extend(final_greens);
    for tile in &red_tiles {
        map.insert(*tile, Tile::Red);
    }
    for tile in &green_tiles {
        map.entry(*tile)
            .and_modify(|_| panic!("inserting green tile on red"))
            .or_insert(Tile::Green);
    }
    let limits = Limits {
        width,
        height,
        min_width,
        min_height,
    };
    println!("x: {min_width}-{width}");
    println!("y: {min_height}-{height}");
    // try and find the inside of the loop
    // first go left
    let mut start = green_tiles[0];
    let mut seen_neither = false;
    let mut success = false;
    while start.x > min_width {
        start = start.left().expect("should be able to go left");
        match map.get(&start).unwrap_or(&Tile::Neither) {
            Tile::Green => {
                if seen_neither {
                    success = true;
                    break;
                }
            }
            Tile::Red => {
                if seen_neither {
                    success = true;
                    break;
                }
            }
            Tile::Neither => {
                seen_neither = true;
            }
        }
    }
    if success {
        //flood leftward
        start = green_tiles[0];
        floodfill(
            &mut map,
            start.left().expect("should be able to go left"),
            &limits,
        );
        return Map { red_tiles, map };
    }
    // then try right
    let mut start = green_tiles[0];
    let mut seen_neither = false;
    let mut success = false;
    while start.x < width {
        start = start.right().expect("should be able to go right");
        match map.get(&start).unwrap_or(&Tile::Neither) {
            Tile::Green => {
                if seen_neither {
                    success = true;
                    break;
                }
            }
            Tile::Red => {
                if seen_neither {
                    success = true;
                    break;
                }
            }
            Tile::Neither => {
                seen_neither = true;
            }
        }
    }
    if success {
        //flood rightward
        start = green_tiles[0];
        floodfill(
            &mut map,
            start.right().expect("should be able to go right"),
            &limits,
        );
        return Map { red_tiles, map };
    }
    // then try up
    let mut start = green_tiles[0];
    let mut seen_neither = false;
    let mut success = false;
    while start.y > min_height {
        start = start.up().expect("should be able to go up");
        match map.get(&start).unwrap_or(&Tile::Neither) {
            Tile::Green => {
                if seen_neither {
                    success = true;
                    break;
                }
            }
            Tile::Red => {
                if seen_neither {
                    success = true;
                    break;
                }
            }
            Tile::Neither => {
                seen_neither = true;
            }
        }
    }
    if success {
        //flood upward
        start = green_tiles[0];
        floodfill(
            &mut map,
            start.up().expect("should be able to go up"),
            &limits,
        );
        return Map { red_tiles, map };
    }
    // then try down
    let mut start = green_tiles[0];
    let mut seen_neither = false;
    let mut success = false;
    while start.y < height {
        start = start.down().expect("should be able to go down");
        match map.get(&start).unwrap_or(&Tile::Neither) {
            Tile::Green => {
                if seen_neither {
                    success = true;
                    break;
                }
            }
            Tile::Red => {
                if seen_neither {
                    success = true;
                    break;
                }
            }
            Tile::Neither => {
                seen_neither = true;
            }
        }
    }
    if success {
        //flood downward
        start = green_tiles[0];
        floodfill(
            &mut map,
            start.down().expect("should be able to go down"),
            &limits,
        );
        // for y in 0..=height {
        //     for x in 0..=width {
        //         match map.get(&Coord { x, y }).unwrap_or(&Tile::Neither) {
        //             Tile::Green => {
        //                 print!("X")
        //             }
        //             Tile::Red => {
        //                 print!("#")
        //             }
        //             Tile::Neither => {
        //                 print!(".")
        //             }
        //         }
        //     }
        //     println!();
        // }
        // println!();
        return Map { red_tiles, map };
    }
    // then give up
    panic!("could not find interior");
}

fn all_red_or_green(map: &HashMap<Coord, Tile>, start: Coord, end: Coord) -> bool {
    if start.x > end.x {
        if start.y > end.y {
            for x in end.x..=start.x {
                for y in end.y..=start.y {
                    if map.get(&Coord { x, y }).is_none() {
                        return false;
                    }
                }
            }
        } else {
            for x in end.x..=start.x {
                for y in start.y..=end.y {
                    if map.get(&Coord { x, y }).is_none() {
                        return false;
                    }
                }
            }
        }
    } else {
        if start.y > end.y {
            for x in start.x..=end.x {
                for y in end.y..=start.y {
                    if map.get(&Coord { x, y }).is_none() {
                        return false;
                    }
                }
            }
        } else {
            for x in start.x..=end.x {
                for y in start.y..=end.x {
                    if map.get(&Coord { x, y }).is_none() {
                        return false;
                    }
                }
            }
        }
    }
    true
}

pub fn process_part1(input: &str) -> String {
    let map = get_simple_map(input);
    let mut biggest_area = 0;
    for (i, corner) in map.iter().enumerate() {
        for next_corner in map.iter().skip(i) {
            let area = corner.area(next_corner);
            if biggest_area < area {
                biggest_area = area;
            }
        }
    }
    biggest_area.to_string()
}

pub fn process_part2(input: &str) -> String {
    let map = get_full_map(input);
    let mut biggest_area = 0;
    for (i, corner) in map.red_tiles.iter().enumerate() {
        for next_corner in map.red_tiles.iter().skip(i + 1) {
            let area = corner.area(next_corner);
            if all_red_or_green(&map.map, *corner, *next_corner) && biggest_area < area {
                biggest_area = area;
            }
        }
    }
    biggest_area.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input() {
        let file = include_str!("../test-input-1.txt");
        assert_eq!(process_part1(file), "50");
        assert_eq!(process_part2(file), "24");
    }
}
