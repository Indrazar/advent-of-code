#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Tile {
    Nothing,
    Wall,
    Box,
    Robot,
    BoxHalfLeft,
    BoxHalfRight,
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

fn try_move_small_box(location: &Coord, direction: &Direction, data: &mut MapData) -> bool {
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
            if try_move_small_box(&new_location, direction, data) {
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
        Tile::BoxHalfLeft => {
            panic!("cannot move double sized boxes here");
        }
        Tile::BoxHalfRight => {
            panic!("cannot move double sized boxes here");
        }
    }
}

fn run_all_small_movements(data: MapData) -> MapData {
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
                if try_move_small_box(&Coord { x: try_x, y: try_y }, &direction, &mut new_data) {
                    new_data.map[new_data.robot_pos.y][new_data.robot_pos.x] = Tile::Nothing;
                    new_data.map[try_y][try_x] = Tile::Robot;
                    new_data.robot_pos = Coord { x: try_x, y: try_y };
                } else {
                    continue;
                }
            }
            Tile::Robot => panic!("robot ran into itself at ({try_x},{try_y})"),
            Tile::BoxHalfLeft => {
                panic!("cannot move double sized boxes here");
            }
            Tile::BoxHalfRight => {
                panic!("cannot move double sized boxes here");
            }
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
                Tile::BoxHalfLeft => {
                    print!("[");
                }
                Tile::BoxHalfRight => {
                    print!("]");
                }
            }
        }
        println!()
    }
}

fn calculate_total(data: &MapData) -> usize {
    let mut sum = 0;
    let mut big_detected = false;
    let mut small_detected = false;
    for (y, tile_row) in data.map.iter().enumerate() {
        for (x, tile) in tile_row.iter().enumerate() {
            match tile {
                Tile::Box => {
                    if big_detected {
                        panic!("should only be counting small or large, not both");
                    }
                    small_detected = true;
                    sum += 100 * y + x;
                }
                Tile::BoxHalfLeft => {
                    if small_detected {
                        panic!("should only be counting large or small, not both");
                    }
                    big_detected = true;
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
    let new_data = run_all_small_movements(data);
    //print_map(&new_data);
    calculate_total(&new_data).to_string()
}

fn big_parse(input: &str) -> MapData {
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
            let left_tile_type: Tile = match ch {
                '#' => Tile::Wall,
                'O' => Tile::BoxHalfLeft,
                '.' => Tile::Nothing,
                '@' => {
                    robot_pos = Coord {
                        x: (x * 2).try_into().unwrap(),
                        y: y.try_into().unwrap(),
                    };
                    Tile::Robot
                }
                unknown => {
                    panic!("unknown tile type: {unknown}");
                }
            };
            let right_tile_type: Tile = match ch {
                '#' => Tile::Wall,
                'O' => Tile::BoxHalfRight,
                '.' => Tile::Nothing,
                '@' => Tile::Nothing,
                unknown => {
                    panic!("unknown tile type: {unknown}");
                }
            };
            tile_line.push(left_tile_type);
            tile_line.push(right_tile_type);
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

fn large_box_moveable(
    location_left: &Coord,
    location_right: &Coord,
    direction: &Direction,
    data: &MapData,
) -> bool {
    //validate locations
    //println!("location_left: {location_left:?}, location_right: {location_right:?}, direction: {direction:?}");
    if *location_right != location_left.add(Direction::Right) {
        panic!(
            "box is not a full box somehow??? left: {location_left:?}, right: {location_right:?}, direction: {direction:?}"
        );
    }
    if data.map[location_left.y][location_left.x] != Tile::BoxHalfLeft {
        panic!(
            "left side is not a left half box: left: {location_left:?}, right: {location_right:?}, direction: {direction:?}"
        )
    }
    if data.map[location_right.y][location_right.x] != Tile::BoxHalfRight {
        panic!(
            "right side is not a right half box: left: {location_left:?}, right: {location_right:?}, direction: {direction:?}"
        )
    }
    let new_location_left = location_left.add(*direction);
    let new_location_right = location_right.add(*direction);
    if *direction == Direction::Up || *direction == Direction::Down {
        match (
            data.map[new_location_left.y][new_location_left.x],
            data.map[new_location_right.y][new_location_right.x],
        ) {
            (Tile::Box, _) => {
                panic!("cannot move small boxes");
            }
            (_, Tile::Box) => {
                panic!("cannot move small boxes");
            }
            (Tile::Wall, _) => {
                return false;
            }
            (_, Tile::Wall) => {
                return false;
            }
            (Tile::Robot, _) => {
                panic!("ran into robot left");
            }
            (_, Tile::Robot) => {
                panic!("ran into robot right");
            }
            (Tile::Nothing, Tile::Nothing) => {
                return true;
            }

            (Tile::Nothing, Tile::BoxHalfLeft) => {
                if large_box_moveable(
                    &new_location_right,
                    &Coord {
                        x: new_location_right.x + 1,
                        y: new_location_right.y,
                    },
                    direction,
                    data,
                ) {
                    return true;
                } else {
                    return false;
                }
            }
            (Tile::Nothing, Tile::BoxHalfRight) => {
                panic!("right box side with no left is impossible");
            }
            (Tile::BoxHalfLeft, Tile::Nothing) => {
                panic!("left box side with no right is impossible");
            }
            (Tile::BoxHalfLeft, Tile::BoxHalfLeft) => {
                panic!("two left box sides is impossible");
            }
            (Tile::BoxHalfLeft, Tile::BoxHalfRight) => {
                if large_box_moveable(&new_location_left, &new_location_right, direction, data) {
                    return true;
                } else {
                    return false;
                }
            }
            (Tile::BoxHalfRight, Tile::Nothing) => {
                if large_box_moveable(
                    &Coord {
                        x: new_location_left.x - 1,
                        y: new_location_left.y,
                    },
                    &new_location_left,
                    direction,
                    data,
                ) {
                    return true;
                } else {
                    return false;
                }
            }
            (Tile::BoxHalfRight, Tile::BoxHalfLeft) => {
                //two box situation
                if large_box_moveable(
                    &Coord {
                        x: new_location_left.x - 1,
                        y: new_location_left.y,
                    },
                    &new_location_left,
                    direction,
                    data,
                ) && large_box_moveable(
                    &new_location_right,
                    &Coord {
                        x: new_location_right.x + 1,
                        y: new_location_right.y,
                    },
                    direction,
                    data,
                ) {
                    return true;
                } else {
                    return false;
                }
            }
            (Tile::BoxHalfRight, Tile::BoxHalfRight) => {
                panic!("two right sides is impossible");
            }
        }
    } else {
        //box is moving right or left
        if *direction == Direction::Right {
            // right
            match data.map[new_location_right.y][new_location_right.x] {
                Tile::Nothing => {
                    return true;
                }
                Tile::Wall => {
                    return false;
                }
                Tile::Box => {
                    panic!("cannot move small boxes here");
                }
                Tile::Robot => {
                    panic!("ran into robot")
                }
                Tile::BoxHalfLeft => {
                    // recursively call move_large_box
                    return large_box_moveable(
                        &new_location_right,
                        &Coord {
                            x: new_location_right.x + 1,
                            y: new_location_right.y,
                        },
                        direction,
                        data,
                    );
                }
                Tile::BoxHalfRight => {
                    panic!("ran into a right half with no left while moving right");
                }
            }
        } else {
            // left
            match data.map[new_location_left.y][new_location_left.x] {
                Tile::Nothing => {
                    return true;
                }
                Tile::Wall => {
                    return false;
                }
                Tile::Box => {
                    panic!("cannot move small boxes here");
                }
                Tile::Robot => {
                    panic!("ran into robot")
                }
                Tile::BoxHalfLeft => {
                    panic!("ran into a left half with no right while moving left");
                }
                Tile::BoxHalfRight => {
                    // recursively call move_large_box
                    return large_box_moveable(
                        &Coord {
                            x: new_location_left.x - 1,
                            y: new_location_left.y,
                        },
                        &new_location_left,
                        direction,
                        data,
                    );
                }
            }
        }
    }
}

fn move_large_box(
    location_left: &Coord,
    location_right: &Coord,
    direction: &Direction,
    data: &mut MapData,
) {
    //validate locations
    if *location_right != location_left.add(Direction::Right) {
        panic!(
            "box is not a full box somehow??? left: {location_left:?}, right: {location_right:?}, direction: {direction:?}"
        );
    }
    if data.map[location_left.y][location_left.x] != Tile::BoxHalfLeft {
        panic!(
            "left side is not a left half box: left: {location_left:?}, right: {location_right:?}, direction: {direction:?}"
        )
    }
    if data.map[location_right.y][location_right.x] != Tile::BoxHalfRight {
        panic!(
        "right side is not a right half box: left: {location_left:?}, right: {location_right:?}, direction: {direction:?}"
    )
    }
    let new_location_left = location_left.add(*direction);
    let new_location_right = location_right.add(*direction);
    if *direction == Direction::Up || *direction == Direction::Down {
        match (
            data.map[new_location_left.y][new_location_left.x],
            data.map[new_location_right.y][new_location_right.x],
        ) {
            (Tile::Box, _) => {
                panic!("cannot move small boxes");
            }
            (_, Tile::Box) => {
                panic!("cannot move small boxes");
            }
            (Tile::Wall, _) => {
                panic!("cannot move into wall");
            }
            (_, Tile::Wall) => {
                panic!("cannot move into wall");
            }
            (Tile::Robot, _) => {
                panic!("ran into robot left");
            }
            (_, Tile::Robot) => {
                panic!("ran into robot right");
            }
            (Tile::Nothing, Tile::Nothing) => {
                data.map[location_left.y][location_left.x] = Tile::Nothing;
                data.map[location_right.y][location_right.x] = Tile::Nothing;
                data.map[new_location_left.y][new_location_left.x] = Tile::BoxHalfLeft;
                data.map[new_location_right.y][new_location_right.x] = Tile::BoxHalfRight;
            }
            (Tile::Nothing, Tile::BoxHalfLeft) => {
                move_large_box(
                    &new_location_right,
                    &Coord {
                        x: new_location_right.x + 1,
                        y: new_location_right.y,
                    },
                    direction,
                    data,
                );
                data.map[location_left.y][location_left.x] = Tile::Nothing;
                data.map[location_right.y][location_right.x] = Tile::Nothing;
                data.map[new_location_left.y][new_location_left.x] = Tile::BoxHalfLeft;
                data.map[new_location_right.y][new_location_right.x] = Tile::BoxHalfRight;
            }
            (Tile::Nothing, Tile::BoxHalfRight) => {
                panic!("right box side with no left is impossible");
            }
            (Tile::BoxHalfLeft, Tile::Nothing) => {
                panic!("left box side with no right is impossible");
            }
            (Tile::BoxHalfLeft, Tile::BoxHalfLeft) => {
                panic!("two left box sides is impossible");
            }
            (Tile::BoxHalfLeft, Tile::BoxHalfRight) => {
                move_large_box(&new_location_left, &new_location_right, direction, data);
                data.map[location_left.y][location_left.x] = Tile::Nothing;
                data.map[location_right.y][location_right.x] = Tile::Nothing;
                data.map[new_location_left.y][new_location_left.x] = Tile::BoxHalfLeft;
                data.map[new_location_right.y][new_location_right.x] = Tile::BoxHalfRight;
            }
            (Tile::BoxHalfRight, Tile::Nothing) => {
                move_large_box(
                    &Coord {
                        x: new_location_left.x - 1,
                        y: new_location_left.y,
                    },
                    &new_location_left,
                    direction,
                    data,
                );
                data.map[location_left.y][location_left.x] = Tile::Nothing;
                data.map[location_right.y][location_right.x] = Tile::Nothing;
                data.map[new_location_left.y][new_location_left.x] = Tile::BoxHalfLeft;
                data.map[new_location_right.y][new_location_right.x] = Tile::BoxHalfRight;
            }
            (Tile::BoxHalfRight, Tile::BoxHalfLeft) => {
                move_large_box(
                    &Coord {
                        x: new_location_left.x - 1,
                        y: new_location_left.y,
                    },
                    &new_location_left,
                    direction,
                    data,
                );
                move_large_box(
                    &new_location_right,
                    &Coord {
                        x: new_location_right.x + 1,
                        y: new_location_right.y,
                    },
                    direction,
                    data,
                );
                data.map[location_left.y][location_left.x] = Tile::Nothing;
                data.map[location_right.y][location_right.x] = Tile::Nothing;
                data.map[new_location_left.y][new_location_left.x] = Tile::BoxHalfLeft;
                data.map[new_location_right.y][new_location_right.x] = Tile::BoxHalfRight;
            }
            (Tile::BoxHalfRight, Tile::BoxHalfRight) => {
                panic!("two right sides is impossible");
            }
        }
    } else {
        //box is moving right or left
        if *direction == Direction::Right {
            // right
            match data.map[new_location_right.y][new_location_right.x] {
                Tile::Nothing => {
                    //successful move
                    data.map[location_left.y][location_left.x] = Tile::Nothing;
                    data.map[location_right.y][location_right.x] = Tile::Nothing;
                    data.map[new_location_left.y][new_location_left.x] = Tile::BoxHalfLeft;
                    data.map[new_location_right.y][new_location_right.x] = Tile::BoxHalfRight;
                }
                Tile::Wall => {
                    panic!("cannot move into wall");
                }
                Tile::Box => {
                    panic!("cannot move small boxes here");
                }
                Tile::Robot => {
                    panic!("ran into robot")
                }
                Tile::BoxHalfLeft => {
                    // recursively call move_large_box
                    move_large_box(
                        &new_location_right,
                        &Coord {
                            x: new_location_right.x + 1,
                            y: new_location_right.y,
                        },
                        direction,
                        data,
                    );
                    data.map[location_left.y][location_left.x] = Tile::Nothing;
                    data.map[location_right.y][location_right.x] = Tile::Nothing;
                    data.map[new_location_left.y][new_location_left.x] = Tile::BoxHalfLeft;
                    data.map[new_location_right.y][new_location_right.x] = Tile::BoxHalfRight;
                }
                Tile::BoxHalfRight => {
                    panic!("ran into a right half with no left while moving right");
                }
            }
        } else {
            // left
            match data.map[new_location_left.y][new_location_left.x] {
                Tile::Nothing => {
                    //successful move
                    data.map[location_left.y][location_left.x] = Tile::Nothing;
                    data.map[new_location_left.y][new_location_left.x] = Tile::BoxHalfLeft;
                    data.map[new_location_right.y][new_location_right.x] = Tile::BoxHalfRight;
                }
                Tile::Wall => {
                    panic!("cannot move into wall");
                }
                Tile::Box => {
                    panic!("cannot move small boxes here");
                }
                Tile::Robot => {
                    panic!("ran into robot")
                }
                Tile::BoxHalfLeft => {
                    panic!("ran into a left half with no right while moving left");
                }
                Tile::BoxHalfRight => {
                    // recursively call move_large_box
                    move_large_box(
                        &Coord {
                            x: new_location_left.x - 1,
                            y: new_location_left.y,
                        },
                        &new_location_left,
                        direction,
                        data,
                    );
                    data.map[location_right.y][location_right.x] = Tile::Nothing;
                    data.map[new_location_left.y][new_location_left.x] = Tile::BoxHalfLeft;
                    data.map[new_location_right.y][new_location_right.x] = Tile::BoxHalfRight;
                }
            }
        }
    }
}

fn run_all_big_movements(data: MapData) -> MapData {
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
                panic!("should not have small boxes in this function");
            }
            Tile::Robot => panic!("robot ran into itself at ({try_x},{try_y})"),
            Tile::BoxHalfLeft => {
                let left_side = Coord { x: try_x, y: try_y };
                let right_side = left_side.add(Direction::Right);
                if large_box_moveable(&left_side, &right_side, &direction, &new_data) {
                    move_large_box(&left_side, &right_side, &direction, &mut new_data);
                    new_data.map[new_data.robot_pos.y][new_data.robot_pos.x] = Tile::Nothing;
                    new_data.map[try_y][try_x] = Tile::Robot;
                    new_data.robot_pos = Coord { x: try_x, y: try_y };
                } else {
                    continue;
                }
            }
            Tile::BoxHalfRight => {
                let right_side = Coord { x: try_x, y: try_y };
                let left_side = right_side.add(Direction::Left);
                if large_box_moveable(&left_side, &right_side, &direction, &new_data) {
                    move_large_box(&left_side, &right_side, &direction, &mut new_data);
                    new_data.map[new_data.robot_pos.y][new_data.robot_pos.x] = Tile::Nothing;
                    new_data.map[try_y][try_x] = Tile::Robot;
                    new_data.robot_pos = Coord { x: try_x, y: try_y };
                } else {
                    continue;
                }
            }
        }
        //print_map(&new_data);
    }
    new_data
}

pub fn process_part2(input: &str) -> String {
    let data = big_parse(input);
    //print_map(&data);
    let new_data = run_all_big_movements(data);
    calculate_total(&new_data).to_string()
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
        assert_eq!(process_part2(file), "9021")
    }
}
