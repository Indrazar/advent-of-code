use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Tile {
    Nothing,
    Wall,
    Box,
    Robot,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Coord {
    x: usize,
    y: usize,
}

impl Coord {
    fn add(self, other: Direction) -> Coord {
        let (x, y) = match other {
            Direction::Up => (self.x, self.y - 1),
            Direction::Down => (self.x, self.y + 1),
            Direction::Left => (self.x - 1, self.y),
            Direction::Right => (self.x + 1, self.y),
        };
        Coord { x, y }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn add(self, other: Coord) -> Coord {
        let (x, y) = match self {
            Direction::Up => (other.x, other.y - 1),
            Direction::Down => (other.x, other.y + 1),
            Direction::Left => (other.x - 1, other.y),
            Direction::Right => (other.x + 1, other.y),
        };
        Coord { x, y }
    }
}

#[derive(Debug, Clone)]
struct MapData {
    map: Vec<Vec<Tile>>,
    robot_pos: Coord,
    move_list: Vec<Direction>,
}

fn parse(input: &str) -> MapData {
    let mut input_iter = input.split("\n\n");
    let map = input_iter.next().expect("there should be a map");
    let directions = input_iter
        .next()
        .expect("there should be a list of directions");

    let mut tile_map: Vec<Vec<Tile>> = Vec::new();
    let mut robot_pos: Coord = Coord { x: 0, y: 0 };
    for (y, line) in map.lines().enumerate() {
        let mut tile_line: Vec<Tile> = Vec::new();
        for (x, ch) in line.chars().enumerate() {
            let tile_type: Tile = match ch {
                '#' => Tile::Wall,
                'O' => Tile::Box,
                '.' => Tile::Nothing,
                '@' => {
                    robot_pos = Coord {
                        x: x.try_into().unwrap(),
                        y: y.try_into().unwrap(),
                    };
                    Tile::Robot
                }
                unknown => {
                    panic!("unknown tile type: {unknown}");
                }
            };
            tile_line.push(tile_type);
        }
        tile_map.push(tile_line);
    }
    let mut move_list: Vec<Direction> = Vec::new();
    for ch in directions.chars() {
        let dir: Direction = match ch {
            '^' => Direction::Up,
            'v' => Direction::Down,
            '<' => Direction::Left,
            '>' => Direction::Right,
            '\n' => continue,
            unknown => {
                panic!("unknown direction type: {unknown}");
            }
        };
        move_list.push(dir);
    }
    if robot_pos == (Coord { x: 0, y: 0 }) {
        panic!("could not find robot")
    }
    MapData {
        map: tile_map,
        robot_pos,
        move_list,
    }
}

fn try_move_box(location: &Coord, direction: &Direction, data: &mut MapData) -> bool {
    let new_location = location.add(*direction);
    match data.map[new_location.y][new_location.x] {
        Tile::Nothing => {
            data.map[new_location.y][new_location.x] = Tile::Box;
            return true;
        }
        Tile::Wall => {
            return false;
        }
        Tile::Box => {
            if try_move_box(&new_location, direction, data) {
                data.map[new_location.y][new_location.x] = Tile::Box;
                return true;
            } else {
                return false;
            }
        }
        Tile::Robot => {
            panic!(
                "pushed box ran into the robot at ({},{})",
                new_location.x, new_location.y
            );
        }
    }
}

fn run_all_movements(data: MapData) -> MapData {
    let mut new_data = MapData {
        map: data.map.clone(),
        robot_pos: data.robot_pos,
        move_list: Vec::new(),
    };
    for direction in data.move_list {
        let Coord { x: try_x, y: try_y } = new_data.robot_pos.add(direction);
        match new_data.map[try_y][try_x] {
            Tile::Nothing => {
                new_data.map[new_data.robot_pos.y][new_data.robot_pos.x] = Tile::Nothing;
                new_data.map[try_y][try_x] = Tile::Robot;
                new_data.robot_pos = Coord { x: try_x, y: try_y };
            }
            Tile::Wall => {
                continue;
            }
            Tile::Box => {
                if try_move_box(&Coord { x: try_x, y: try_y }, &direction, &mut new_data) {
                    new_data.map[new_data.robot_pos.y][new_data.robot_pos.x] = Tile::Nothing;
                    new_data.map[try_y][try_x] = Tile::Robot;
                    new_data.robot_pos = Coord { x: try_x, y: try_y };
                } else {
                    continue;
                }
            }
            Tile::Robot => panic!("robot ran into itself at ({try_x},{try_y})"),
        }
    }
    new_data
}

fn print_map(data: &MapData) {
    println!("current map:");
    for tile_row in &data.map {
        for tile in tile_row {
            match tile {
                Tile::Nothing => {
                    print!(".");
                }
                Tile::Wall => {
                    print!("#");
                }
                Tile::Box => {
                    print!("O");
                }
                Tile::Robot => {
                    print!("@");
                }
            }
        }
        println!()
    }
}

fn calculate_total(data: &MapData) -> usize {
    let mut sum = 0;
    for (y, tile_row) in data.map.iter().enumerate() {
        for (x, tile) in tile_row.iter().enumerate() {
            match tile {
                Tile::Box => {
                    sum += 100 * y + x;
                }
                _ => {
                    continue;
                }
            }
        }
    }
    sum
}

pub fn process_part1(input: &str) -> String {
    let data = parse(input);
    let new_data = run_all_movements(data);
    //print_map(&new_data);
    calculate_total(&new_data).to_string()
}

pub fn process_part2(input: &str) -> String {
    "works".to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input() {
        let file = include_str!("../test-input-1.txt");
        assert_eq!(process_part1(file), "2028");
    }
    #[test]
    fn test_input2() {
        let file = include_str!("../test-input-2.txt");
        assert_eq!(process_part1(file), "10092");
    }
}
