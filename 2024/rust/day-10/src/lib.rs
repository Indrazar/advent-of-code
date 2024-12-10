use std::collections::BTreeMap;
use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Coord {
    x: usize,
    y: usize,
}

#[derive(Debug, Clone)]
struct MapData {
    map: HashMap<Coord, usize>,
    heights: HashMap<usize, Vec<Coord>>,
}

fn flood(
    input: Coord,
    prev_height: Option<usize>,
    data: &MapData,
    flood_map: Option<HashMap<Coord, bool>>,
) -> HashMap<Coord, bool> {
    let mut flood_map: HashMap<Coord, bool> = match flood_map {
        Some(f_m) => f_m,
        None => HashMap::new(),
    };
    // check current square
    let tile_height = match data.map.get(&input) {
        Some(tile_height) => {
            if prev_height.is_none() || (*tile_height == prev_height.unwrap_or(0) + 1) {
                // valid next step, continue flooding
                //println!(
                //    "flood: {:?} to ({},{}) height: {}",
                //    prev_height, input.x, input.y, tile_height
                //);
                flood_map.insert(input, true);
                *tile_height
            } else {
                // invalid next step, do not continue flooding
                return flood_map;
            }
        }
        None => return flood_map,
    };
    // check upwards
    if input.y >= 1 {
        let up = Coord {
            x: input.x,
            y: input.y - 1,
        };

        flood_map = flood(up, Some(tile_height), data, Some(flood_map));
    }
    // check downwards
    let down = Coord {
        x: input.x,
        y: input.y + 1,
    };
    flood_map = flood(down, Some(tile_height), data, Some(flood_map));
    // check left
    if input.x >= 1 {
        let left = Coord {
            x: input.x - 1,
            y: input.y,
        };
        flood_map = flood(left, Some(tile_height), data, Some(flood_map));
    }
    // check right
    let right = Coord {
        x: input.x + 1,
        y: input.y,
    };
    flood_map = flood(right, Some(tile_height), data, Some(flood_map));
    flood_map
}

fn parse(input: &str) -> MapData {
    let mut map: HashMap<Coord, usize> = HashMap::new();
    let mut heights: HashMap<usize, Vec<Coord>> = HashMap::new();
    for (y, row) in input.lines().enumerate() {
        for (x, ch) in row.chars().enumerate() {
            let height: usize = ch.to_digit(10).expect("should be a digit") as usize;
            map.insert(Coord { x, y }, height);
            match heights.get_mut(&height) {
                Some(group) => {
                    group.push(Coord { x, y });
                }
                None => {
                    heights.insert(height, vec![Coord { x, y }]);
                }
            }
        }
    }
    MapData { map, heights }
}

pub fn process_part1(input: &str) -> String {
    let data = parse(input);
    let starting_points = match data.heights.get(&0) {
        Some(l) => l.clone(),
        None => panic!("we expect some starting points"),
    };
    let mut total: i64 = 0;
    for start in starting_points {
        let mut trail_score: i64 = 0;
        let flood_map = flood(start, None, &data, None);
        for ending_point in data.heights.get(&9).expect("we expect some ending points") {
            if *flood_map.get(ending_point).unwrap_or(&false) {
                trail_score += 1;
            }
        }
        //println!(
        //    "start point: ({},{}) has a score of {trail_score}",
        //    start.x, start.y
        //);
        total += trail_score;
    }
    total.to_string()
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
        assert_eq!(process_part1(file), "36");
        //assert_eq!(process_part2(file), "81");
    }
}
