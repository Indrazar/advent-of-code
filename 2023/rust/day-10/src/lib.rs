#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum TileType {
    Vertical,
    Horizontal,
    NorthEast90,
    NorthWest90,
    SouthWest90,
    SouthEast90,
    Ground,
    Start,
    Unknown,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ExitedFrom {
    North,
    East,
    South,
    West,
    None,
}

impl TryFrom<char> for TileType {
    fn try_from(input: char) -> Result<Self, ()> {
        match input {
            '|' => Ok(TileType::Vertical),
            '-' => Ok(TileType::Horizontal),
            'L' => Ok(TileType::NorthEast90),
            'J' => Ok(TileType::NorthWest90),
            '7' => Ok(TileType::SouthWest90),
            'F' => Ok(TileType::SouthEast90),
            '.' => Ok(TileType::Ground),
            'S' => Ok(TileType::Start),
            _ => Err(()),
        }
    }

    type Error = ();
}

#[derive(Debug, Clone, Copy)]
struct Tile {
    tile_type: TileType,
    start_tile_type: TileType,
    start_exit_type: ExitedFrom,
    dead_end: bool,
    loop_flooded: bool,
    depth: i64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ExpandedTileType {
    Nothing,
    Pipe,
}

#[derive(Debug, Clone, Copy)]
struct ExpandedTile {
    tile_type: ExpandedTileType,
    flooded: bool,
    flooding: bool,
}

fn parse_map(input: &str) -> (usize, usize, Vec<Vec<Tile>>) {
    let mut pipes: Vec<Vec<Tile>> = Vec::default();
    let mut start_y: usize = usize::MAX;
    let mut start_x: usize = usize::MAX;
    let mut x_pos = 0;
    let mut y_pos = 0;
    for line in input.lines() {
        let mut pipe_row: Vec<Tile> = Vec::default();
        for char in line.chars() {
            let tile = char.try_into().expect("should be a valid pipe type");
            match tile {
                TileType::Start => {
                    start_y = y_pos;
                    start_x = x_pos;
                }
                _ => {}
            };
            pipe_row.push(Tile {
                tile_type: tile,
                start_tile_type: TileType::Unknown,
                start_exit_type: ExitedFrom::None,
                loop_flooded: false,
                dead_end: false,
                depth: i64::MAX,
            });
            x_pos += 1;
        }
        pipes.push(pipe_row);
        x_pos = 0;
        y_pos += 1;
    }
    if start_x == usize::MAX || start_y == usize::MAX {
        panic!("input did not contain start location")
    }
    (start_y, start_x, pipes)
}

fn flood(
    fill: bool,
    depth: i64,
    in_direction: ExitedFrom,
    pos_y: usize,
    pos_x: usize,
    input_map: &mut Vec<Vec<Tile>>,
) {
    let current_tile: &mut Tile = &mut input_map[pos_y][pos_x];
    if current_tile.tile_type == TileType::Start {
        current_tile.loop_flooded = true;
        if fill {
            current_tile.start_tile_type = match (in_direction, current_tile.start_exit_type) {
                (ExitedFrom::North, ExitedFrom::North) => TileType::Vertical,
                (ExitedFrom::North, ExitedFrom::East) => TileType::SouthEast90,
                (ExitedFrom::North, ExitedFrom::South) => panic!("invalid start shape"),
                (ExitedFrom::North, ExitedFrom::West) => TileType::SouthWest90,
                (ExitedFrom::East, ExitedFrom::North) => TileType::NorthWest90,
                (ExitedFrom::East, ExitedFrom::East) => TileType::Horizontal,
                (ExitedFrom::East, ExitedFrom::South) => TileType::SouthWest90,
                (ExitedFrom::East, ExitedFrom::West) => panic!("invalid start shape"),
                (ExitedFrom::South, ExitedFrom::North) => panic!("invalid start shape"),
                (ExitedFrom::South, ExitedFrom::East) => TileType::NorthEast90,
                (ExitedFrom::South, ExitedFrom::South) => TileType::Vertical,
                (ExitedFrom::South, ExitedFrom::West) => TileType::NorthWest90,
                (ExitedFrom::West, ExitedFrom::North) => TileType::NorthEast90,
                (ExitedFrom::West, ExitedFrom::East) => panic!("invalid start shape"),
                (ExitedFrom::West, ExitedFrom::South) => TileType::SouthEast90,
                (ExitedFrom::West, ExitedFrom::West) => TileType::Horizontal,
                (_, ExitedFrom::None) => panic!("invalid start shape"),
                (ExitedFrom::None, _) => panic!("invalid start shape"),
            };
        }
        return;
    }
    if fill {
        current_tile.loop_flooded = true;
    }
    current_tile.depth = std::cmp::min(depth, current_tile.depth);

    match (current_tile.tile_type, in_direction) {
        (TileType::Vertical, ExitedFrom::North) => {
            if pos_y != 0 {
                flood(
                    fill,
                    depth + 1,
                    ExitedFrom::North,
                    pos_y - 1,
                    pos_x,
                    input_map,
                )
            }
        }
        (TileType::Vertical, ExitedFrom::East) => {
            current_tile.dead_end = true;
        }
        (TileType::Vertical, ExitedFrom::South) => {
            if pos_y + 1 <= input_map.len() {
                flood(
                    fill,
                    depth + 1,
                    ExitedFrom::South,
                    pos_y + 1,
                    pos_x,
                    input_map,
                )
            }
        }
        (TileType::Vertical, ExitedFrom::West) => {
            current_tile.dead_end = true;
        }
        (TileType::Horizontal, ExitedFrom::North) => {
            current_tile.dead_end = true;
        }
        (TileType::Horizontal, ExitedFrom::East) => {
            if pos_x + 1 <= input_map[pos_y].len() {
                flood(
                    fill,
                    depth + 1,
                    ExitedFrom::East,
                    pos_y,
                    pos_x + 1,
                    input_map,
                )
            }
        }
        (TileType::Horizontal, ExitedFrom::South) => {
            current_tile.dead_end = true;
        }
        (TileType::Horizontal, ExitedFrom::West) => {
            if pos_x != 0 {
                flood(
                    fill,
                    depth + 1,
                    ExitedFrom::West,
                    pos_y,
                    pos_x - 1,
                    input_map,
                )
            }
        }
        (TileType::NorthEast90, ExitedFrom::North) => {
            current_tile.dead_end = true;
        }
        (TileType::NorthEast90, ExitedFrom::East) => {
            current_tile.dead_end = true;
        }
        (TileType::NorthEast90, ExitedFrom::South) => {
            if pos_x + 1 <= input_map[pos_y].len() {
                flood(
                    fill,
                    depth + 1,
                    ExitedFrom::East,
                    pos_y,
                    pos_x + 1,
                    input_map,
                )
            }
        }
        (TileType::NorthEast90, ExitedFrom::West) => {
            if pos_y != 0 {
                flood(
                    fill,
                    depth + 1,
                    ExitedFrom::North,
                    pos_y - 1,
                    pos_x,
                    input_map,
                )
            }
        }
        (TileType::NorthWest90, ExitedFrom::North) => {
            current_tile.dead_end = true;
        }
        (TileType::NorthWest90, ExitedFrom::East) => {
            if pos_y != 0 {
                flood(
                    fill,
                    depth + 1,
                    ExitedFrom::North,
                    pos_y - 1,
                    pos_x,
                    input_map,
                )
            }
        }
        (TileType::NorthWest90, ExitedFrom::South) => {
            if pos_x != 0 {
                flood(
                    fill,
                    depth + 1,
                    ExitedFrom::West,
                    pos_y,
                    pos_x - 1,
                    input_map,
                )
            }
        }
        (TileType::NorthWest90, ExitedFrom::West) => {
            current_tile.dead_end = true;
        }
        (TileType::SouthWest90, ExitedFrom::North) => {
            if pos_x != 0 {
                flood(
                    fill,
                    depth + 1,
                    ExitedFrom::West,
                    pos_y,
                    pos_x - 1,
                    input_map,
                )
            }
        }
        (TileType::SouthWest90, ExitedFrom::East) => {
            if pos_y + 1 <= input_map.len() {
                flood(
                    fill,
                    depth + 1,
                    ExitedFrom::South,
                    pos_y + 1,
                    pos_x,
                    input_map,
                )
            }
        }
        (TileType::SouthWest90, ExitedFrom::South) => {
            current_tile.dead_end = true;
        }
        (TileType::SouthWest90, ExitedFrom::West) => {
            current_tile.dead_end = true;
        }
        (TileType::SouthEast90, ExitedFrom::North) => {
            if pos_x + 1 <= input_map[pos_y].len() {
                flood(
                    fill,
                    depth + 1,
                    ExitedFrom::East,
                    pos_y,
                    pos_x + 1,
                    input_map,
                )
            }
        }
        (TileType::SouthEast90, ExitedFrom::East) => {
            current_tile.dead_end = true;
        }
        (TileType::SouthEast90, ExitedFrom::South) => {
            current_tile.dead_end = true;
        }
        (TileType::SouthEast90, ExitedFrom::West) => {
            if pos_y + 1 <= input_map.len() {
                flood(
                    fill,
                    depth + 1,
                    ExitedFrom::South,
                    pos_y + 1,
                    pos_x,
                    input_map,
                )
            }
        }
        (TileType::Ground, _) => {
            current_tile.dead_end = true;
        }
        _ => {
            panic!("got something unexpected")
        }
    };
}

fn flood_from_start(start_y: usize, start_x: usize, input_map: &mut Vec<Vec<Tile>>) -> ExitedFrom {
    let mut res = ExitedFrom::None;
    let mut first_loop = true;
    let depth: i64 = 0;
    input_map[start_y][start_x].depth = 0;
    //scan for connected pipes starting on West and going clockwise
    if start_x != 0 {
        flood(
            false,
            depth + 1,
            ExitedFrom::West,
            start_y,
            start_x - 1,
            input_map,
        );
        if first_loop && input_map[start_y][start_x].loop_flooded {
            res = ExitedFrom::West;
            input_map[start_y][start_x].start_exit_type = ExitedFrom::West;
            first_loop = false;
        }
    } // check North
    if start_y != 0 {
        flood(
            false,
            depth + 1,
            ExitedFrom::North,
            start_y - 1,
            start_x,
            input_map,
        );
        if first_loop && input_map[start_y][start_x].loop_flooded {
            res = ExitedFrom::North;
            input_map[start_y][start_x].start_exit_type = ExitedFrom::North;
            first_loop = false;
        }
    } // check East
    if start_x + 1 <= input_map[start_y].len() {
        flood(
            false,
            depth + 1,
            ExitedFrom::East,
            start_y,
            start_x + 1,
            input_map,
        );
        if first_loop && input_map[start_y][start_x].loop_flooded {
            res = ExitedFrom::East;
            input_map[start_y][start_x].start_exit_type = ExitedFrom::East;
            first_loop = false;
        }
    } // check South
    if start_y + 1 <= input_map.len() {
        flood(
            false,
            depth + 1,
            ExitedFrom::South,
            start_y + 1,
            start_x,
            input_map,
        );
        if first_loop && input_map[start_y][start_x].loop_flooded {
            input_map[start_y][start_x].start_exit_type = ExitedFrom::South;
            res = ExitedFrom::South;
            first_loop = false;
        }
    }
    if res == ExitedFrom::None {
        panic!("no loop found");
    }
    return res;
}

fn mark_main_loop(
    start_y: usize,
    start_x: usize,
    loop_dir: ExitedFrom,
    input_map: &mut Vec<Vec<Tile>>,
) {
    let mut first_loop = true;
    let depth: i64 = 0;
    input_map[start_y][start_x].depth = 0;
    // only follow the path given
    // if west:
    if loop_dir == ExitedFrom::West {
        flood(
            true,
            depth + 1,
            ExitedFrom::West,
            start_y,
            start_x - 1,
            input_map,
        );
        if first_loop && input_map[start_y][start_x].loop_flooded {
            first_loop = false;
        }
    }
    // if North
    else if loop_dir == ExitedFrom::North {
        flood(
            true,
            depth + 1,
            ExitedFrom::North,
            start_y - 1,
            start_x,
            input_map,
        );
        if first_loop && input_map[start_y][start_x].loop_flooded {
            first_loop = false;
        }
    }
    // if East
    else if start_x + 1 <= input_map[start_y].len() {
        flood(
            true,
            depth + 1,
            ExitedFrom::East,
            start_y,
            start_x + 1,
            input_map,
        );
        if first_loop && input_map[start_y][start_x].loop_flooded {
            first_loop = false;
        }
    }
    // if South
    else if start_y + 1 <= input_map.len() {
        flood(
            true,
            depth + 1,
            ExitedFrom::South,
            start_y + 1,
            start_x,
            input_map,
        );
        if first_loop && input_map[start_y][start_x].loop_flooded {
            first_loop = false;
        }
    } else {
        panic!("must provide a valid exit")
    }
}

fn expand_map(input_map: Vec<Vec<Tile>>) -> Vec<Vec<ExpandedTile>> {
    let mut expanded_map: Vec<Vec<ExpandedTile>> = Vec::default();
    for row in input_map.iter() {
        let mut expanded_row: Vec<ExpandedTile> = Vec::default();
        for top_big_tile in row {
            let tile_type = {
                if !top_big_tile.loop_flooded {
                    TileType::Ground
                } else {
                    match top_big_tile.tile_type {
                        TileType::Start => top_big_tile.start_tile_type,
                        TileType::Unknown => panic!("invalid tile type"),
                        x => x,
                    }
                }
            };
            match tile_type {
                TileType::Vertical => {
                    expanded_row.push(ExpandedTile {
                        tile_type: ExpandedTileType::Nothing,
                        flooded: false,
                        flooding: false,
                    });
                    expanded_row.push(ExpandedTile {
                        tile_type: ExpandedTileType::Pipe,
                        flooded: false,
                        flooding: false,
                    });
                    expanded_row.push(ExpandedTile {
                        tile_type: ExpandedTileType::Nothing,
                        flooded: false,
                        flooding: false,
                    });
                }
                TileType::Horizontal => {
                    expanded_row.push(ExpandedTile {
                        tile_type: ExpandedTileType::Nothing,
                        flooded: false,
                        flooding: false,
                    });
                    expanded_row.push(ExpandedTile {
                        tile_type: ExpandedTileType::Nothing,
                        flooded: false,
                        flooding: false,
                    });
                    expanded_row.push(ExpandedTile {
                        tile_type: ExpandedTileType::Nothing,
                        flooded: false,
                        flooding: false,
                    });
                }
                TileType::NorthEast90 => {
                    expanded_row.push(ExpandedTile {
                        tile_type: ExpandedTileType::Nothing,
                        flooded: false,
                        flooding: false,
                    });
                    expanded_row.push(ExpandedTile {
                        tile_type: ExpandedTileType::Pipe,
                        flooded: false,
                        flooding: false,
                    });
                    expanded_row.push(ExpandedTile {
                        tile_type: ExpandedTileType::Nothing,
                        flooded: false,
                        flooding: false,
                    });
                }
                TileType::NorthWest90 => {
                    expanded_row.push(ExpandedTile {
                        tile_type: ExpandedTileType::Nothing,
                        flooded: false,
                        flooding: false,
                    });
                    expanded_row.push(ExpandedTile {
                        tile_type: ExpandedTileType::Pipe,
                        flooded: false,
                        flooding: false,
                    });
                    expanded_row.push(ExpandedTile {
                        tile_type: ExpandedTileType::Nothing,
                        flooded: false,
                        flooding: false,
                    });
                }
                TileType::SouthWest90 => {
                    expanded_row.push(ExpandedTile {
                        tile_type: ExpandedTileType::Nothing,
                        flooded: false,
                        flooding: false,
                    });
                    expanded_row.push(ExpandedTile {
                        tile_type: ExpandedTileType::Nothing,
                        flooded: false,
                        flooding: false,
                    });
                    expanded_row.push(ExpandedTile {
                        tile_type: ExpandedTileType::Nothing,
                        flooded: false,
                        flooding: false,
                    });
                }
                TileType::SouthEast90 => {
                    expanded_row.push(ExpandedTile {
                        tile_type: ExpandedTileType::Nothing,
                        flooded: false,
                        flooding: false,
                    });
                    expanded_row.push(ExpandedTile {
                        tile_type: ExpandedTileType::Nothing,
                        flooded: false,
                        flooding: false,
                    });
                    expanded_row.push(ExpandedTile {
                        tile_type: ExpandedTileType::Nothing,
                        flooded: false,
                        flooding: false,
                    });
                }
                TileType::Ground => {
                    expanded_row.push(ExpandedTile {
                        tile_type: ExpandedTileType::Nothing,
                        flooded: false,
                        flooding: false,
                    });
                    expanded_row.push(ExpandedTile {
                        tile_type: ExpandedTileType::Nothing,
                        flooded: false,
                        flooding: false,
                    });
                    expanded_row.push(ExpandedTile {
                        tile_type: ExpandedTileType::Nothing,
                        flooded: false,
                        flooding: false,
                    });
                }
                _ => panic!("invalid tile type"),
            }
        }
        expanded_map.push(expanded_row);
        let mut expanded_row: Vec<ExpandedTile> = Vec::default();
        for middle_big_tile in row {
            let tile_type = {
                if !middle_big_tile.loop_flooded {
                    TileType::Ground
                } else {
                    match middle_big_tile.tile_type {
                        TileType::Start => middle_big_tile.start_tile_type,
                        TileType::Unknown => panic!("invalid tile type"),
                        x => x,
                    }
                }
            };
            match tile_type {
                TileType::Vertical => {
                    expanded_row.push(ExpandedTile {
                        tile_type: ExpandedTileType::Nothing,
                        flooded: false,
                        flooding: false,
                    });
                    expanded_row.push(ExpandedTile {
                        tile_type: ExpandedTileType::Pipe,
                        flooded: false,
                        flooding: false,
                    });
                    expanded_row.push(ExpandedTile {
                        tile_type: ExpandedTileType::Nothing,
                        flooded: false,
                        flooding: false,
                    });
                }
                TileType::Horizontal => {
                    expanded_row.push(ExpandedTile {
                        tile_type: ExpandedTileType::Pipe,
                        flooded: false,
                        flooding: false,
                    });
                    expanded_row.push(ExpandedTile {
                        tile_type: ExpandedTileType::Pipe,
                        flooded: false,
                        flooding: false,
                    });
                    expanded_row.push(ExpandedTile {
                        tile_type: ExpandedTileType::Pipe,
                        flooded: false,
                        flooding: false,
                    });
                }
                TileType::NorthEast90 => {
                    expanded_row.push(ExpandedTile {
                        tile_type: ExpandedTileType::Nothing,
                        flooded: false,
                        flooding: false,
                    });
                    expanded_row.push(ExpandedTile {
                        tile_type: ExpandedTileType::Pipe,
                        flooded: false,
                        flooding: false,
                    });
                    expanded_row.push(ExpandedTile {
                        tile_type: ExpandedTileType::Pipe,
                        flooded: false,
                        flooding: false,
                    });
                }
                TileType::NorthWest90 => {
                    expanded_row.push(ExpandedTile {
                        tile_type: ExpandedTileType::Pipe,
                        flooded: false,
                        flooding: false,
                    });
                    expanded_row.push(ExpandedTile {
                        tile_type: ExpandedTileType::Pipe,
                        flooded: false,
                        flooding: false,
                    });
                    expanded_row.push(ExpandedTile {
                        tile_type: ExpandedTileType::Nothing,
                        flooded: false,
                        flooding: false,
                    });
                }
                TileType::SouthWest90 => {
                    expanded_row.push(ExpandedTile {
                        tile_type: ExpandedTileType::Pipe,
                        flooded: false,
                        flooding: false,
                    });
                    expanded_row.push(ExpandedTile {
                        tile_type: ExpandedTileType::Pipe,
                        flooded: false,
                        flooding: false,
                    });
                    expanded_row.push(ExpandedTile {
                        tile_type: ExpandedTileType::Nothing,
                        flooded: false,
                        flooding: false,
                    });
                }
                TileType::SouthEast90 => {
                    expanded_row.push(ExpandedTile {
                        tile_type: ExpandedTileType::Nothing,
                        flooded: false,
                        flooding: false,
                    });
                    expanded_row.push(ExpandedTile {
                        tile_type: ExpandedTileType::Pipe,
                        flooded: false,
                        flooding: false,
                    });
                    expanded_row.push(ExpandedTile {
                        tile_type: ExpandedTileType::Pipe,
                        flooded: false,
                        flooding: false,
                    });
                }
                TileType::Ground => {
                    expanded_row.push(ExpandedTile {
                        tile_type: ExpandedTileType::Nothing,
                        flooded: false,
                        flooding: false,
                    });
                    expanded_row.push(ExpandedTile {
                        tile_type: ExpandedTileType::Nothing,
                        flooded: false,
                        flooding: false,
                    });
                    expanded_row.push(ExpandedTile {
                        tile_type: ExpandedTileType::Nothing,
                        flooded: false,
                        flooding: false,
                    });
                }
                _ => panic!("invalid tile type"),
            }
        }
        expanded_map.push(expanded_row);
        let mut expanded_row: Vec<ExpandedTile> = Vec::default();
        for bottom_big_tile in row {
            let tile_type = {
                if !bottom_big_tile.loop_flooded {
                    TileType::Ground
                } else {
                    match bottom_big_tile.tile_type {
                        TileType::Start => bottom_big_tile.start_tile_type,
                        TileType::Unknown => panic!("invalid tile type"),
                        x => x,
                    }
                }
            };
            match tile_type {
                TileType::Vertical => {
                    expanded_row.push(ExpandedTile {
                        tile_type: ExpandedTileType::Nothing,
                        flooded: false,
                        flooding: false,
                    });
                    expanded_row.push(ExpandedTile {
                        tile_type: ExpandedTileType::Pipe,
                        flooded: false,
                        flooding: false,
                    });
                    expanded_row.push(ExpandedTile {
                        tile_type: ExpandedTileType::Nothing,
                        flooded: false,
                        flooding: false,
                    });
                }
                TileType::Horizontal => {
                    expanded_row.push(ExpandedTile {
                        tile_type: ExpandedTileType::Nothing,
                        flooded: false,
                        flooding: false,
                    });
                    expanded_row.push(ExpandedTile {
                        tile_type: ExpandedTileType::Nothing,
                        flooded: false,
                        flooding: false,
                    });
                    expanded_row.push(ExpandedTile {
                        tile_type: ExpandedTileType::Nothing,
                        flooded: false,
                        flooding: false,
                    });
                }
                TileType::NorthEast90 => {
                    expanded_row.push(ExpandedTile {
                        tile_type: ExpandedTileType::Nothing,
                        flooded: false,
                        flooding: false,
                    });
                    expanded_row.push(ExpandedTile {
                        tile_type: ExpandedTileType::Nothing,
                        flooded: false,
                        flooding: false,
                    });
                    expanded_row.push(ExpandedTile {
                        tile_type: ExpandedTileType::Nothing,
                        flooded: false,
                        flooding: false,
                    });
                }
                TileType::NorthWest90 => {
                    expanded_row.push(ExpandedTile {
                        tile_type: ExpandedTileType::Nothing,
                        flooded: false,
                        flooding: false,
                    });
                    expanded_row.push(ExpandedTile {
                        tile_type: ExpandedTileType::Nothing,
                        flooded: false,
                        flooding: false,
                    });
                    expanded_row.push(ExpandedTile {
                        tile_type: ExpandedTileType::Nothing,
                        flooded: false,
                        flooding: false,
                    });
                }
                TileType::SouthWest90 => {
                    expanded_row.push(ExpandedTile {
                        tile_type: ExpandedTileType::Nothing,
                        flooded: false,
                        flooding: false,
                    });
                    expanded_row.push(ExpandedTile {
                        tile_type: ExpandedTileType::Pipe,
                        flooded: false,
                        flooding: false,
                    });
                    expanded_row.push(ExpandedTile {
                        tile_type: ExpandedTileType::Nothing,
                        flooded: false,
                        flooding: false,
                    });
                }
                TileType::SouthEast90 => {
                    expanded_row.push(ExpandedTile {
                        tile_type: ExpandedTileType::Nothing,
                        flooded: false,
                        flooding: false,
                    });
                    expanded_row.push(ExpandedTile {
                        tile_type: ExpandedTileType::Pipe,
                        flooded: false,
                        flooding: false,
                    });
                    expanded_row.push(ExpandedTile {
                        tile_type: ExpandedTileType::Nothing,
                        flooded: false,
                        flooding: false,
                    });
                }
                TileType::Ground => {
                    expanded_row.push(ExpandedTile {
                        tile_type: ExpandedTileType::Nothing,
                        flooded: false,
                        flooding: false,
                    });
                    expanded_row.push(ExpandedTile {
                        tile_type: ExpandedTileType::Nothing,
                        flooded: false,
                        flooding: false,
                    });
                    expanded_row.push(ExpandedTile {
                        tile_type: ExpandedTileType::Nothing,
                        flooded: false,
                        flooding: false,
                    });
                }
                _ => panic!("invalid tile type"),
            }
        }
        expanded_map.push(expanded_row);
    }
    expanded_map
}

pub fn process_part1(input: &str) -> String {
    let (start_y, start_x, mut pipes) = parse_map(input);
    let mut furthest: i64 = 0;
    flood_from_start(start_y, start_x, &mut pipes);
    for row in pipes.iter() {
        for tile in row {
            if tile.depth == i64::MAX {
                continue;
            } else if tile.depth > furthest && !tile.dead_end {
                furthest = tile.depth;
            }
        }
    }
    furthest.to_string()
}

fn any_tile_flooding(expanded_map: &Vec<Vec<ExpandedTile>>) -> bool {
    for row in expanded_map {
        for tile in row {
            if tile.flooding {
                return true;
            } else {
                continue;
            }
        }
    }
    return false;
}

fn non_recursive_expanded_flood(
    input_y_pos: usize,
    input_x_pos: usize,
    expanded_map: &mut Vec<Vec<ExpandedTile>>,
) {
    expanded_map[input_y_pos][input_x_pos].flooding = true;
    while any_tile_flooding(&expanded_map) {
        for y_pos in 0..expanded_map.len() {
            for x_pos in 0..expanded_map[y_pos].len() {
                if expanded_map[y_pos][x_pos].flooding {
                    // check west
                    if x_pos != 0
                        && expanded_map[y_pos][x_pos - 1].tile_type != ExpandedTileType::Pipe
                        && !expanded_map[y_pos][x_pos - 1].flooded
                        && !expanded_map[y_pos][x_pos - 1].flooding
                    {
                        expanded_map[y_pos][x_pos - 1].flooding = true;
                    }
                    // check north
                    if y_pos != 0
                        && expanded_map[y_pos - 1][x_pos].tile_type != ExpandedTileType::Pipe
                        && !expanded_map[y_pos - 1][x_pos].flooded
                        && !expanded_map[y_pos - 1][x_pos].flooding
                    {
                        expanded_map[y_pos - 1][x_pos].flooding = true;
                    }
                    // check east
                    if x_pos + 1 < expanded_map[y_pos].len()
                        && expanded_map[y_pos][x_pos + 1].tile_type != ExpandedTileType::Pipe
                        && !expanded_map[y_pos][x_pos + 1].flooded
                        && !expanded_map[y_pos][x_pos + 1].flooding
                    {
                        expanded_map[y_pos][x_pos + 1].flooding = true;
                    }
                    // check south
                    if y_pos + 1 < expanded_map.len()
                        && expanded_map[y_pos + 1][x_pos].tile_type != ExpandedTileType::Pipe
                        && !expanded_map[y_pos + 1][x_pos].flooded
                        && !expanded_map[y_pos + 1][x_pos].flooding
                    {
                        expanded_map[y_pos + 1][x_pos].flooding = true;
                    }
                    expanded_map[y_pos][x_pos].flooding = false;
                    expanded_map[y_pos][x_pos].flooded = true;
                }
            }
        }
    }
}

fn collapse_map(expanded_map: &Vec<Vec<ExpandedTile>>) -> i64 {
    let mut count = 0;
    for y in (0..expanded_map.len()).step_by(3) {
        for x in (0..expanded_map[y].len()).step_by(3) {
            if (expanded_map[y][x].tile_type == ExpandedTileType::Nothing
                && !expanded_map[y][x].flooded)
                && (expanded_map[y][x + 1].tile_type == ExpandedTileType::Nothing
                    && !expanded_map[y][x + 1].flooded)
                && (expanded_map[y][x + 2].tile_type == ExpandedTileType::Nothing
                    && !expanded_map[y][x + 2].flooded)
                && (expanded_map[y + 1][x].tile_type == ExpandedTileType::Nothing
                    && !expanded_map[y + 1][x].flooded)
                && (expanded_map[y + 1][x + 1].tile_type == ExpandedTileType::Nothing
                    && !expanded_map[y + 1][x + 1].flooded)
                && (expanded_map[y + 1][x + 2].tile_type == ExpandedTileType::Nothing
                    && !expanded_map[y + 1][x + 2].flooded)
                && (expanded_map[y + 2][x].tile_type == ExpandedTileType::Nothing
                    && !expanded_map[y + 2][x].flooded)
                && (expanded_map[y + 2][x + 1].tile_type == ExpandedTileType::Nothing
                    && !expanded_map[y + 2][x + 1].flooded)
                && (expanded_map[y + 2][x + 2].tile_type == ExpandedTileType::Nothing
                    && !expanded_map[y + 2][x + 2].flooded)
            {
                count += 1;
            }
        }
    }
    count
}

pub fn process_part2(input: &str) -> String {
    let (start_y, start_x, mut pipes) = parse_map(input);
    let loop_dir = flood_from_start(start_y, start_x, &mut pipes);
    mark_main_loop(start_y, start_x, loop_dir, &mut pipes);
    // expand pipe map from 1x1 to 3x3
    let mut expanded_pipes: Vec<Vec<ExpandedTile>> = expand_map(pipes);
    // find all unenclosed point on edges
    // top edge
    for x in 0..expanded_pipes[0].len() {
        if expanded_pipes[0][x].tile_type == ExpandedTileType::Nothing
            && !expanded_pipes[0][x].flooded
        {
            non_recursive_expanded_flood(0, x, &mut expanded_pipes);
        }
    }
    // left edge
    for y in 0..expanded_pipes.len() {
        if expanded_pipes[y][0].tile_type == ExpandedTileType::Nothing
            && !expanded_pipes[y][0].flooded
        {
            non_recursive_expanded_flood(y, 0, &mut expanded_pipes);
        }
    }
    // right edge
    for y in 0..expanded_pipes.len() {
        if expanded_pipes[y]
            .last()
            .expect("map can't be empty")
            .tile_type
            == ExpandedTileType::Nothing
            && !expanded_pipes[y]
                .last()
                .expect("map can't be empty")
                .flooded
        {
            non_recursive_expanded_flood(y, expanded_pipes[y].len() - 1, &mut expanded_pipes);
        }
    }
    // bottom edge
    for x in 0..expanded_pipes.last().expect("map can't be empty").len() {
        if expanded_pipes.last().expect("map can't be empty")[x].tile_type
            == ExpandedTileType::Nothing
            && !expanded_pipes.last().expect("map can't be empty")[x].flooded
        {
            non_recursive_expanded_flood(expanded_pipes.len() - 1, x, &mut expanded_pipes);
        }
    }
    collapse_map(&expanded_pipes).to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    // part 1
    #[test]
    fn test_file1() {
        let file1 = fs::read_to_string("./test-input-1.txt").unwrap();
        assert_eq!(process_part1(file1.as_str()), "4");
    }
    #[test]
    fn test_file2() {
        let file2 = fs::read_to_string("./test-input-2.txt").unwrap();
        assert_eq!(process_part1(file2.as_str()), "4");
    }
    // part 2
    #[test]
    fn test_file3() {
        let file3 = fs::read_to_string("./test-input-3.txt").unwrap();
        assert_eq!(process_part2(file3.as_str()), "4");
    }
    #[test]
    fn test_file4() {
        let file4 = fs::read_to_string("./test-input-4.txt").unwrap();
        assert_eq!(process_part2(file4.as_str()), "4");
    }
    #[test]
    fn test_file5() {
        let file5 = fs::read_to_string("./test-input-5.txt").unwrap();
        assert_eq!(process_part2(file5.as_str()), "8");
    }
    #[test]
    fn test_file6() {
        let file6 = fs::read_to_string("./test-input-6.txt").unwrap();
        assert_eq!(process_part2(file6.as_str()), "10");
    }
    #[test]
    fn test_file7() {
        let file6 = fs::read_to_string("./test-input-7.txt").unwrap();
        assert_eq!(process_part2(file6.as_str()), "0");
    }
}
