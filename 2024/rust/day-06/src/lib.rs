#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Tile {
    Clear,
    Obstructed,
    Guard,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct TileData {
    tile_type: Tile,
    visited: bool,
    facing_direciton_when_visited: Vec<Direction>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Direction {
    North,
    East,
    South,
    West,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Coord {
    x: usize,
    y: usize,
}

#[derive(Debug, Clone)]
struct MapData {
    map: Vec<Vec<TileData>>,
    guard_facing: Direction,
    guard_location: Coord,
    visited_total: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum EndType {
    LeftMap,
    LoopFound,
    StepComplete,
}

fn print_map(map: &MapData) {
    println!(
        "\n\nguard is at {},{}, facing: {:?} and has visited {} tiles\n",
        map.guard_location.x, map.guard_location.y, map.guard_facing, map.visited_total
    );
    for row in &map.map {
        for tile in row {
            match (tile.tile_type, tile.visited) {
                (Tile::Clear, true) => {
                    print!("X")
                }
                (Tile::Clear, false) => {
                    print!(".")
                }
                (Tile::Obstructed, true) => {
                    panic!("\n reached a visited but obstructed tile")
                }
                (Tile::Obstructed, false) => {
                    print!("#")
                }
                (Tile::Guard, true) => {
                    print!("G")
                }
                (Tile::Guard, false) => {
                    panic!("\n reached an unvisited but guard tile")
                }
            }
        }
        print!("\n");
    }
}

fn parse(input: &str) -> MapData {
    let mut guard_facing: Direction = Direction::North;
    let mut guard_location: Coord = Coord { x: 0, y: 0 };
    let mut map: Vec<Vec<TileData>> = Vec::new();
    for (y, row) in input.lines().enumerate() {
        let mut map_row: Vec<TileData> = Vec::new();
        for (x, char) in row.chars().enumerate() {
            match char {
                '.' => {
                    map_row.push(TileData {
                        tile_type: Tile::Clear,
                        visited: false,
                        facing_direciton_when_visited: vec![],
                    });
                }
                '#' => {
                    map_row.push(TileData {
                        tile_type: Tile::Obstructed,
                        visited: false,
                        facing_direciton_when_visited: vec![],
                    });
                }
                '^' => {
                    map_row.push(TileData {
                        tile_type: Tile::Guard,
                        visited: true,
                        facing_direciton_when_visited: vec![Direction::North],
                    });
                    guard_facing = Direction::North;
                    guard_location = Coord { x, y };
                }
                _ => {
                    panic!("invalid character in parse");
                }
            }
        }
        map.push(map_row);
    }
    MapData {
        map,
        guard_facing,
        visited_total: 1usize,
        guard_location,
    }
}

fn iterate_guard_once(map: &mut MapData) -> EndType {
    let next_peek_coord = match map.guard_facing {
        Direction::North => {
            if map.guard_location.y == 0 {
                return EndType::LeftMap;
            } else {
                Coord {
                    x: map.guard_location.x,
                    y: map.guard_location.y - 1,
                }
            }
        }
        Direction::East => {
            if map.guard_location.x == map.map[0].len() - 1 {
                return EndType::LeftMap;
            } else {
                Coord {
                    x: map.guard_location.x + 1,
                    y: map.guard_location.y,
                }
            }
        }
        Direction::South => {
            if map.guard_location.y == map.map.len() - 1 {
                return EndType::LeftMap;
            } else {
                Coord {
                    x: map.guard_location.x,
                    y: map.guard_location.y + 1,
                }
            }
        }
        Direction::West => {
            if map.guard_location.x == 0 {
                return EndType::LeftMap;
            } else {
                Coord {
                    x: map.guard_location.x - 1,
                    y: map.guard_location.y,
                }
            }
        }
    };
    match map.map[next_peek_coord.y][next_peek_coord.x].tile_type {
        Tile::Clear => {
            // first check for loop
            if map.map[next_peek_coord.y][next_peek_coord.x]
                .facing_direciton_when_visited
                .contains(&map.guard_facing)
            {
                return EndType::LoopFound;
            } else {
                // remove guard from previous tile
                map.map[map.guard_location.y][map.guard_location.x].tile_type = Tile::Clear;
                // move to this tile
                map.guard_location = next_peek_coord;
                map.map[next_peek_coord.y][next_peek_coord.x].tile_type = Tile::Guard;
                // mark visited
                if !map.map[next_peek_coord.y][next_peek_coord.x].visited {
                    map.map[next_peek_coord.y][next_peek_coord.x].visited = true;
                    map.visited_total += 1;
                }
                // mark facing direction for this tile
                map.map[next_peek_coord.y][next_peek_coord.x]
                    .facing_direciton_when_visited
                    .push(map.guard_facing);
                return EndType::StepComplete;
            }
        }
        Tile::Obstructed => {
            map.guard_facing = match map.guard_facing {
                Direction::North => Direction::East,
                Direction::East => Direction::South,
                Direction::South => Direction::West,
                Direction::West => Direction::North,
            };
            if map.map[map.guard_location.y][map.guard_location.x]
                .facing_direciton_when_visited
                .contains(&map.guard_facing)
            {
                return EndType::LoopFound;
            } else {
                map.map[map.guard_location.y][map.guard_location.x]
                    .facing_direciton_when_visited
                    .push(map.guard_facing);
                return EndType::StepComplete;
            }
        }
        Tile::Guard => {
            print_map(map);
            panic!("the guard ran into itself??");
        }
    }
}

pub fn process_part1(input: &str) -> String {
    let mut map = parse(input);
    //print_map(&map);
    let mut check: EndType = iterate_guard_once(&mut map);
    while check == EndType::StepComplete {
        check = iterate_guard_once(&mut map);
    }
    match check {
        EndType::LeftMap => {
            //print_map(&map);
            return map.visited_total.to_string();
        }
        EndType::LoopFound => {
            //print_map(&map);
            panic!("we found a loop")
        }
        EndType::StepComplete => {
            //print_map(&map);
            panic!("shouldn't get here")
        }
    }
}

pub fn process_part2(input: &str) -> String {
    let map = parse(input);
    //print_map(&map);
    let mut loops_created = 0;
    for y in 0..map.map.len() {
        for x in 0..map.map[y].len() {
            let mut trial_map = map.clone();
            match trial_map.map[y][x].tile_type {
                Tile::Clear => trial_map.map[y][x].tile_type = Tile::Obstructed,
                Tile::Obstructed => continue,
                Tile::Guard => continue,
            }
            let mut check: EndType = iterate_guard_once(&mut trial_map);
            while check == EndType::StepComplete {
                check = iterate_guard_once(&mut trial_map);
            }
            match check {
                EndType::LeftMap => {}
                EndType::LoopFound => {
                    loops_created += 1;
                }
                EndType::StepComplete => {
                    panic!("shouldn't get here")
                }
            }
        }
    }
    loops_created.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input() {
        let file = include_str!("../test-input-1.txt");
        assert_eq!(process_part1(file), "41");
        assert_eq!(process_part2(file), "6");
    }
}
