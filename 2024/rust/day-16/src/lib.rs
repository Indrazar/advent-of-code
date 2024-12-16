use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Direction {
    North,
    South,
    East,
    West,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Tile {
    Empty,
    Wall,
    Start,
    End,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Coord {
    x: usize,
    y: usize,
}

#[derive(Debug, Clone)]
struct MapData {
    start: Coord,
    end: Coord,
    map: Vec<Vec<Tile>>,
    current: Coord,
}

fn parse(input: &str) -> MapData {
    let mut map: Vec<Vec<Tile>> = Vec::new();
    let mut start = Coord { x: 0, y: 0 };
    let mut end = Coord { x: 0, y: 0 };
    for (y, line) in input.lines().enumerate() {
        let mut map_row = Vec::new();
        for (x, ch) in line.chars().enumerate() {
            let tile = match ch {
                '.' => Tile::Empty,
                '#' => Tile::Wall,
                'S' => {
                    start = Coord { x, y };
                    Tile::Start
                }
                'E' => {
                    end = Coord { x, y };
                    Tile::End
                }
                unknown => {
                    panic!("unknown tile found: {unknown}")
                }
            };
            map_row.push(tile);
        }
        map.push(map_row);
    }
    if (start.x == 0 && start.y == 0) || (end.x == 0 && end.y == 0) {
        panic!("start or end not found");
    }
    MapData {
        start,
        end,
        map,
        current: start,
    }
}

fn print_map(map: &MapData) {
    println!("current map");
    for (y, row) in map.map.iter().enumerate() {
        for (x, tile) in row.iter().enumerate() {
            if map.current.y == y && map.current.x == x {
                print!("@");
            } else {
                match tile {
                    Tile::Empty => {
                        print!(".");
                    }
                    Tile::Wall => {
                        print!("#");
                    }
                    Tile::Start => {
                        print!("S");
                    }
                    Tile::End => {
                        print!("E");
                    }
                }
            }
        }
        println!()
    }
}

fn calc_cost(current_direction: Direction, new_direction: Direction) -> usize {
    match (current_direction, new_direction) {
        (Direction::North, Direction::North) => 1,
        (Direction::North, Direction::South) => 2001,
        (Direction::North, Direction::East) => 1001,
        (Direction::North, Direction::West) => 1001,
        (Direction::South, Direction::North) => 2001,
        (Direction::South, Direction::South) => 1,
        (Direction::South, Direction::East) => 1001,
        (Direction::South, Direction::West) => 1001,
        (Direction::East, Direction::North) => 1001,
        (Direction::East, Direction::South) => 1001,
        (Direction::East, Direction::East) => 1,
        (Direction::East, Direction::West) => 2001,
        (Direction::West, Direction::North) => 1001,
        (Direction::West, Direction::South) => 1001,
        (Direction::West, Direction::East) => 2001,
        (Direction::West, Direction::West) => 1,
    }
}

fn flood_with_cost(
    data: &MapData,
    location: Coord,
    flood_map: Option<HashMap<Coord, usize>>,
    cost: Option<usize>,
    direction: Direction,
) -> HashMap<Coord, usize> {
    let mut flood_map: HashMap<Coord, usize> = match flood_map {
        Some(f_m) => f_m,
        None => HashMap::new(),
    };
    let cost: usize = match cost {
        Some(c) => c,
        None => 0,
    };
    // first check if square is valid to travel to
    match data.map[location.y][location.x] {
        Tile::Wall => {
            // invalid, stop flood
            return flood_map;
        }
        Tile::Empty | Tile::End | Tile::Start => {
            // valid location
        }
    }
    // second update hashmap with new(?) cost
    match flood_map.get_mut(&location) {
        Some(val) => {
            // check if current cost is lower, if so: update it and continue flooding
            // if it's not lower, there is a less costly path to get here so we stop
            if *val < cost {
                return flood_map;
            } else {
                *val = cost;
            }
        }
        None => {
            // if the map doesn't have us at all and we know the route is valid, add it
            flood_map.insert(location, cost);
        }
    };
    // third, cost was lower or didn't exist so we need to flood more
    // try up
    if location.y > 0 {
        let new_direction = Direction::North;
        flood_map = flood_with_cost(
            data,
            Coord {
                x: location.x,
                y: location.y - 1,
            },
            Some(flood_map),
            Some(cost + calc_cost(direction, new_direction)),
            new_direction,
        );
    }
    // try down
    if location.y < data.map.len() - 1 {
        let new_direction = Direction::South;
        flood_map = flood_with_cost(
            data,
            Coord {
                x: location.x,
                y: location.y + 1,
            },
            Some(flood_map),
            Some(cost + calc_cost(direction, new_direction)),
            new_direction,
        );
    }
    // try left
    if location.x > 0 {
        let new_direction = Direction::West;
        flood_map = flood_with_cost(
            data,
            Coord {
                x: location.x - 1,
                y: location.y,
            },
            Some(flood_map),
            Some(cost + calc_cost(direction, new_direction)),
            new_direction,
        );
    }
    // try right
    if location.x < data.map[0].len() - 1 {
        let new_direction = Direction::East;
        flood_map = flood_with_cost(
            data,
            Coord {
                x: location.x + 1,
                y: location.y,
            },
            Some(flood_map),
            Some(cost + calc_cost(direction, new_direction)),
            new_direction,
        );
    }
    flood_map
}

pub fn process_part1(input: &str) -> String {
    let data = parse(input);
    //print_map(&data);
    let cost_map = flood_with_cost(&data, data.start, None, None, Direction::East);
    cost_map
        .get(&data.end)
        .expect("there should be a cost")
        .to_string()
}

fn calc_backwards_cost(
    step_back_direction: Direction,
    current_direction: Option<Direction>,
) -> (usize, Direction) {
    match (step_back_direction, current_direction) {
        (Direction::North, None) => (1, Direction::North),
        (Direction::South, None) => (1, Direction::East),
        (Direction::East, None) => (1, Direction::East),
        (Direction::West, None) => (1, Direction::West),
        (Direction::North, Some(Direction::North)) => (1, Direction::North),
        (Direction::North, Some(Direction::South)) => (2001, Direction::North),
        (Direction::North, Some(Direction::East)) => (1001, Direction::North),
        (Direction::North, Some(Direction::West)) => (1001, Direction::North),
        (Direction::South, Some(Direction::North)) => (2001, Direction::South),
        (Direction::South, Some(Direction::South)) => (1, Direction::South),
        (Direction::South, Some(Direction::East)) => (1001, Direction::South),
        (Direction::South, Some(Direction::West)) => (1001, Direction::South),
        (Direction::East, Some(Direction::North)) => (1001, Direction::East),
        (Direction::East, Some(Direction::South)) => (1001, Direction::East),
        (Direction::East, Some(Direction::East)) => (1, Direction::East),
        (Direction::East, Some(Direction::West)) => (2001, Direction::East),
        (Direction::West, Some(Direction::North)) => (1001, Direction::West),
        (Direction::West, Some(Direction::South)) => (1001, Direction::West),
        (Direction::West, Some(Direction::East)) => (2001, Direction::West),
        (Direction::West, Some(Direction::West)) => (1, Direction::West),
    }
}

fn look_backwards(
    cost_map: &HashMap<Coord, usize>,
    end: &Coord,
    start: &Coord,
    max_cost: usize,
) -> Vec<Coord> {
    let mut valid_positions = Vec::new();
    let mut cursors: Vec<(Coord, usize, Option<Direction>, Vec<Coord>)> = Vec::new();
    cursors.push((*end, max_cost, None, vec![*end]));
    while cursors.len() > 0 {
        //println!("cursors contains: {} elements.", cursors.len());
        let current_cursor = cursors.pop().expect("there should be a cursor");
        let current_postion = current_cursor.0;
        let current_cost = current_cursor.1;
        let current_direction = current_cursor.2;
        let history = current_cursor.3;
        // if we're at the start then this cursor is done and
        // we add our history to the valid_positions uniquely
        if current_postion == *start {
            for pos in history {
                if !valid_positions.contains(&pos) {
                    valid_positions.push(pos);
                }
            }
            continue;
        }
        // if we're not at the start and we are out of
        // cost then this cursor is done and invalid
        if current_cost == 0 {
            continue;
        }
        //try up (came from south)
        if current_postion.y > 0 {
            let (next_cost, new_direction) =
                calc_backwards_cost(Direction::South, current_direction);
            if current_cost < next_cost {
                // we can't afford to go here
                // no new cursor is generated
            } else {
                // we CAN afford to go here, generate a new cursor
                let next_position = Coord {
                    x: current_postion.x,
                    y: current_postion.y - 1,
                };
                let remaining_cost = current_cost - next_cost;
                match cost_map.get(&next_position) {
                    Some(cost) => {
                        if remaining_cost >= *cost {
                            // can progress
                            let mut up_history = history.clone();
                            up_history.push(next_position);
                            cursors.push((
                                next_position,
                                remaining_cost,
                                Some(new_direction),
                                up_history,
                            ))
                        } else {
                            // can't progress
                        }
                    }
                    None => {
                        // can't go here
                    }
                }
            }
        }
        //try down (came from north)
        let (next_cost, new_direction) = calc_backwards_cost(Direction::North, current_direction);
        if current_cost < next_cost {
            // we can't afford to go here
            // no new cursor is generated
        } else {
            // we CAN afford to go here, generate a new cursor
            let next_position = Coord {
                x: current_postion.x,
                y: current_postion.y + 1,
            };
            let remaining_cost = current_cost - next_cost;
            match cost_map.get(&next_position) {
                Some(cost) => {
                    if remaining_cost >= *cost {
                        // can progress
                        let mut up_history = history.clone();
                        up_history.push(next_position);
                        cursors.push((
                            next_position,
                            remaining_cost,
                            Some(new_direction),
                            up_history,
                        ))
                    } else {
                        // can't progress
                    }
                }
                None => {
                    // can't go here
                }
            }
        }
        //try left (came from East)
        if current_postion.x > 0 {
            let (next_cost, new_direction) =
                calc_backwards_cost(Direction::East, current_direction);
            if current_cost < next_cost {
                // we can't afford to go here
                // no new cursor is generated
            } else {
                // we CAN afford to go here, generate a new cursor
                let next_position = Coord {
                    x: current_postion.x - 1,
                    y: current_postion.y,
                };
                let remaining_cost = current_cost - next_cost;
                match cost_map.get(&next_position) {
                    Some(cost) => {
                        if remaining_cost >= *cost {
                            // can progress
                            let mut up_history = history.clone();
                            up_history.push(next_position);
                            cursors.push((
                                next_position,
                                remaining_cost,
                                Some(new_direction),
                                up_history,
                            ))
                        } else {
                            // can't progress
                        }
                    }
                    None => {
                        // can't go here
                    }
                }
            }
        }
        //try right (came from West)
        let (next_cost, new_direction) = calc_backwards_cost(Direction::West, current_direction);
        if current_cost < next_cost {
            // we can't afford to go here
            // no new cursor is generated
        } else {
            // we CAN afford to go here, generate a new cursor
            let next_position = Coord {
                x: current_postion.x + 1,
                y: current_postion.y,
            };
            let remaining_cost = current_cost - next_cost;
            match cost_map.get(&next_position) {
                Some(cost) => {
                    if remaining_cost >= *cost {
                        // can progress
                        let mut up_history = history.clone();
                        up_history.push(next_position);
                        cursors.push((
                            next_position,
                            remaining_cost,
                            Some(new_direction),
                            up_history,
                        ))
                    } else {
                        // can't progress
                    }
                }
                None => {
                    // can't go here
                }
            }
        }
    }
    valid_positions
}

pub fn process_part2(input: &str) -> String {
    let data = parse(input);
    //print_map(&data);
    let cost_map = flood_with_cost(&data, data.start, None, None, Direction::East);
    let max_cost = *cost_map.get(&data.end).expect("there should be a cost");
    let position_list = look_backwards(&cost_map, &data.end, &data.start, max_cost);
    position_list.len().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input() {
        let file = include_str!("../test-input-1.txt");
        assert_eq!(process_part1(file), "7036");
        assert_eq!(process_part2(file), "45");
    }
    #[test]
    fn test_input2() {
        let file = include_str!("../test-input-2.txt");
        assert_eq!(process_part1(file), "11048");
        assert_eq!(process_part2(file), "64");
    }
}
