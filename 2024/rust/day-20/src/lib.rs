use std::collections::{HashMap, VecDeque};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Tile {
    Start,
    End,
    Track,
    Wall,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Coord {
    x: usize,
    y: usize,
}

fn parse(input: &str) -> (Coord, Coord, Vec<Vec<Tile>>) {
    let mut map: Vec<Vec<Tile>> = Vec::new();
    let mut start: Coord = Coord { x: 0, y: 0 };
    let mut end: Coord = Coord { x: 0, y: 0 };
    for (y, line) in input.lines().enumerate() {
        let mut map_line: Vec<Tile> = Vec::new();
        for (x, ch) in line.chars().enumerate() {
            let tile = match ch {
                'S' => {
                    start = Coord { x, y };
                    Tile::Start
                }
                'E' => {
                    end = Coord { x, y };
                    Tile::End
                }
                '.' => Tile::Track,
                '#' => Tile::Wall,
                unknown => panic!("unknown character in parse: {unknown}"),
            };
            map_line.push(tile);
        }
        map.push(map_line);
    }
    (start, end, map)
}

fn print_map(input: &Vec<Vec<Tile>>) {
    println!("current map:");
    for y in input.iter() {
        for x in y.iter() {
            match x {
                Tile::Start => {
                    print!("S");
                }
                Tile::End => {
                    print!("E");
                }
                Tile::Track => {
                    print!(".");
                }
                Tile::Wall => {
                    print!("#");
                }
            }
        }
        println!()
    }
}

fn flood_fill_with_count(
    map: &Vec<Vec<Tile>>,
    position_queue: &mut VecDeque<(Coord, usize)>,
    flood_map: &mut HashMap<Coord, usize>,
) {
    let mut position_cost_opt = position_queue.pop_front();
    while position_cost_opt.is_some() {
        let (position, current_cost) = position_cost_opt.unwrap();
        if map[position.y][position.x] == Tile::Wall {
            // cannot move here

            position_cost_opt = position_queue.pop_front();
            continue;
        }
        match flood_map.get_mut(&position) {
            Some(old_cost) => {
                if current_cost < *old_cost {
                    // println!(
                    //     "getting to ({},{}) is cheaper: {current_cost}",
                    //     position.x, position.y
                    // );
                    *old_cost = current_cost;
                } else {
                    // already got here cheaper
                    position_cost_opt = position_queue.pop_front();
                    continue;
                }
            }
            None => {
                flood_map.insert(position, current_cost);
            }
        }
        // try up
        if position.y > 0 {
            let new_position = Coord {
                x: position.x,
                y: position.y - 1,
            };
            // println!(
            //     "adding ({},{}) to queue to be checked",
            //     position.x, position.y
            // );
            position_queue.push_back((new_position, current_cost + 1));
        }
        // try down
        if position.y < map.len() - 1 {
            let new_position = Coord {
                x: position.x,
                y: position.y + 1,
            };
            // println!(
            //     "adding ({},{}) to queue to be checked",
            //     position.x, position.y
            // );
            position_queue.push_back((new_position, current_cost + 1));
        }
        // try left
        if position.x > 0 {
            let new_position = Coord {
                x: position.x - 1,
                y: position.y,
            };
            // println!(
            //     "adding ({},{}) to queue to be checked",
            //     position.x, position.y
            // );
            position_queue.push_back((new_position, current_cost + 1));
        }
        // try right
        if position.x < map.len() - 1 {
            let new_position = Coord {
                x: position.x + 1,
                y: position.y,
            };
            // println!(
            //     "adding ({},{}) to queue to be checked",
            //     position.x, position.y
            // );
            position_queue.push_back((new_position, current_cost + 1));
        }
        //println!("getting next location to check");
        position_cost_opt = position_queue.pop_front();
    }
}

fn reverse_flood(
    map: &HashMap<Coord, usize>,
    position_queue: &mut VecDeque<(Coord, usize)>,
    track: &mut Vec<Coord>,
) {
    let mut position_cost_opt = position_queue.pop_front();
    while position_cost_opt.is_some() {
        let (position, current_cost) = position_cost_opt.unwrap();
        match map.get(&position) {
            Some(cost) => {
                if current_cost == *cost {
                    // correct
                    if !track.contains(&position) {
                        track.push(position);
                    }
                    if *cost == 0 {
                        // we're done
                        position_cost_opt = position_queue.pop_front();
                        continue;
                    }
                } else {
                    // wrong, stop
                    position_cost_opt = position_queue.pop_front();
                    continue;
                }
            }
            None => {
                position_cost_opt = position_queue.pop_front();
                continue;
            }
        }
        // try up
        if position.y > 0 {
            let new_position = Coord {
                x: position.x,
                y: position.y - 1,
            };
            position_queue.push_back((new_position, current_cost - 1));
        }
        // try down
        if position.y < map.len() - 1 {
            let new_position = Coord {
                x: position.x,
                y: position.y + 1,
            };
            position_queue.push_back((new_position, current_cost - 1));
        }
        // try left
        if position.x > 0 {
            let new_position = Coord {
                x: position.x - 1,
                y: position.y,
            };
            position_queue.push_back((new_position, current_cost - 1));
        }
        // try right
        if position.x < map.len() - 1 {
            let new_position = Coord {
                x: position.x + 1,
                y: position.y,
            };
            position_queue.push_back((new_position, current_cost - 1));
        }
        position_cost_opt = position_queue.pop_front();
    }
}

fn generate_list_of_adjacent_walls(map: &Vec<Vec<Tile>>, middle_point: Coord) -> Vec<Coord> {
    let mut tile_list: Vec<Coord> = Vec::with_capacity(8);
    if middle_point.y > 0 {
        match map[middle_point.y - 1][middle_point.x] {
            Tile::Wall => {
                tile_list.push(Coord {
                    x: middle_point.x,
                    y: middle_point.y - 1,
                });
            }
            _ => {}
        }
    }
    // try down
    if middle_point.y < map.len() - 1 {
        match map[middle_point.y + 1][middle_point.x] {
            Tile::Wall => {
                tile_list.push(Coord {
                    x: middle_point.x,
                    y: middle_point.y + 1,
                });
            }
            _ => {}
        }
    }
    // try left
    if middle_point.x > 0 {
        match map[middle_point.y][middle_point.x - 1] {
            Tile::Wall => {
                tile_list.push(Coord {
                    x: middle_point.x - 1,
                    y: middle_point.y,
                });
            }
            _ => {}
        }
    }
    // try right
    if middle_point.x < map.len() - 1 {
        match map[middle_point.y][middle_point.x + 1] {
            Tile::Wall => {
                tile_list.push(Coord {
                    x: middle_point.x + 1,
                    y: middle_point.y,
                });
            }
            _ => {}
        }
    }
    tile_list
}

pub fn process_part1(input: &str) -> String {
    let (start, end, map) = parse(input);
    //print_map(&map);
    let mut flood_map = HashMap::new();
    let mut queue = VecDeque::new();
    queue.push_back((start, 0));
    flood_fill_with_count(&map, &mut queue, &mut flood_map);
    let end_cost = match flood_map.get(&end) {
        Some(cost) => *cost,
        None => panic!("end could not be reached by floodfill"),
    };
    //println!("floodfill done");
    let mut normal_track: Vec<Coord> = Vec::new();
    let mut rev_queue = VecDeque::new();
    rev_queue.push_back((end, end_cost));
    reverse_flood(&flood_map, &mut rev_queue, &mut normal_track);
    //println!("track found");
    let mut wall_list: Vec<Coord> = Vec::new();
    for tile in normal_track {
        for adj in generate_list_of_adjacent_walls(&map, tile).iter() {
            if !wall_list.contains(adj) {
                wall_list.push(*adj);
            }
        }
    }
    //println!("walls found");
    let mut cheat_list: Vec<(Coord, usize)> = Vec::new();
    for wall in wall_list {
        let mut new_map = map.clone();
        new_map[wall.y][wall.x] = Tile::Track;
        let mut new_flood_map = HashMap::new();
        let mut new_queue = VecDeque::new();
        new_queue.push_back((start, 0));
        flood_fill_with_count(&new_map, &mut new_queue, &mut new_flood_map);
        //println!("wall ({},{}) processed", wall.x, wall.y);
        let new_end_cost = match new_flood_map.get(&end) {
            Some(cost) => *cost,
            None => panic!("end could not be reached by floodfill"),
        };
        if new_end_cost < end_cost {
            cheat_list.push((wall, new_end_cost));
        }
    }
    //println!("all cheats found");
    let mut best_cheat: usize = 0;
    let mut cheat_sort: HashMap<usize, usize> = HashMap::new();
    for cheat in cheat_list {
        let _wall_position = cheat.0;
        let save = end_cost - cheat.1;
        if save > best_cheat {
            best_cheat = save;
        }
        match cheat_sort.get_mut(&save) {
            Some(count) => {
                *count += 1;
            }
            None => {
                cheat_sort.insert(save, 1);
            }
        }
    }
    //println!("all cheats counted");
    let mut output_string = String::new();
    let mut at_least_100: usize = 0;
    for i in 1..=best_cheat {
        match cheat_sort.get(&i) {
            Some(count) => {
                if i >= 100 {
                    at_least_100 += *count;
                }
                if *count == 1 {
                    output_string =
                        format!("{output_string}\nThere is one cheat that saves {i} picoseconds.");
                } else {
                    output_string = format!(
                        "{output_string}\nThere are {count} cheats that save {i} picoseconds."
                    );
                }
            }
            None => {}
        }
    }
    output_string = format!("{output_string}\n{at_least_100}");
    //println!("{output_string}");
    output_string.to_string()
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
        assert_eq!(
            process_part1(file),
            "
There are 14 cheats that save 2 picoseconds.
There are 14 cheats that save 4 picoseconds.
There are 2 cheats that save 6 picoseconds.
There are 4 cheats that save 8 picoseconds.
There are 2 cheats that save 10 picoseconds.
There are 3 cheats that save 12 picoseconds.
There is one cheat that saves 20 picoseconds.
There is one cheat that saves 36 picoseconds.
There is one cheat that saves 38 picoseconds.
There is one cheat that saves 40 picoseconds.
There is one cheat that saves 64 picoseconds.
0"
        );
    }
}
