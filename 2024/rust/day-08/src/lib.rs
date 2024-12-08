use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Coord {
    x: i64,
    y: i64,
}

#[derive(Debug, Clone)]
struct MapData {
    antenna_list: HashMap<char, Vec<Coord>>,
    width: i64,
    height: i64,
}

fn insert_antenna(antenna_list: &mut HashMap<char, Vec<Coord>>, x: usize, y: usize, c: char) {
    match antenna_list.get(&c) {
        Some(vec) => {
            let mut vec = vec.clone();
            vec.push(Coord {
                x: x.try_into().unwrap(),
                y: y.try_into().unwrap(),
            });
            antenna_list.insert(c, vec);
        }
        None => {
            antenna_list.insert(
                c,
                vec![Coord {
                    x: x.try_into().unwrap(),
                    y: y.try_into().unwrap(),
                }],
            );
        }
    }
}

fn parse(input: &str) -> MapData {
    //let mut map = HashMap::new();
    let mut antenna_list: HashMap<char, Vec<Coord>> = HashMap::new();
    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            match c {
                '.' => continue,
                _ => {
                    insert_antenna(&mut antenna_list, x, y, c);
                }
            }
        }
    }
    let height = input.lines().count().try_into().unwrap();
    let width = input
        .lines()
        .next()
        .expect("should have characters")
        .chars()
        .count()
        .try_into()
        .unwrap();
    MapData {
        //map,
        antenna_list,
        width,
        height,
    }
}

fn generate_coord_pairs(data: &MapData) -> Vec<(Coord, Coord)> {
    let mut pair_list: Vec<(Coord, Coord)> = Vec::new();
    for same_letter_list in data.antenna_list.values() {
        for i in 1..same_letter_list.len() {
            for j in 0..i {
                pair_list.push((same_letter_list[j], same_letter_list[i]));
            }
        }
    }
    pair_list
}

fn get_pair(antenna1: Coord, antenna2: Coord, magnitude: i64) -> (Coord, Coord) {
    let antenna_x_diff = (antenna1.x - antenna2.x) * magnitude;
    let antenna_y_diff = (antenna1.y - antenna2.y) * magnitude;
    let candidate1: Coord = Coord {
        x: antenna2.x - antenna_x_diff,
        y: antenna2.y - antenna_y_diff,
    };
    let candidate2: Coord = Coord {
        x: antenna1.x + antenna_x_diff,
        y: antenna1.y + antenna_y_diff,
    };
    (candidate1, candidate2)
}

fn generate_candidates(
    width: i64,
    height: i64,
    antenna_pair: (Coord, Coord),
    magnitude_limit: bool,
) -> Vec<Coord> {
    let antenna1 = antenna_pair.0;
    let antenna2 = antenna_pair.1;
    let mut candidates: Vec<Coord> = Vec::new();
    if magnitude_limit {
        let (candidate1, candidate2) = get_pair(antenna1, antenna2, 1);
        // println!(
        //     "for pair {antenna1:?}; {antenna2:?}:\n\
        // possible candidates: {candidate1:?}; {candidate2:?}"
        // );
        if candidate1.x < width && candidate1.x >= 0 && candidate1.y < height && candidate1.y >= 0 {
            candidates.push(candidate1);
        }
        if candidate2.x < width && candidate2.x >= 0 && candidate2.y < height && candidate2.y >= 0 {
            candidates.push(candidate2)
        }
        //println!("found candidates: {candidates:?}");
        candidates
    } else {
        let max_magnitude: i64 = width.max(height);
        for magnitude in (max_magnitude * -1)..max_magnitude {
            let (candidate1, candidate2) = get_pair(antenna1, antenna2, magnitude);
            if candidate1.x < width
                && candidate1.x >= 0
                && candidate1.y < height
                && candidate1.y >= 0
            {
                if candidates.contains(&candidate1) {
                    continue;
                } else {
                    candidates.push(candidate1);
                }
            }
            if candidate2.x < width
                && candidate2.x >= 0
                && candidate2.y < height
                && candidate2.y >= 0
            {
                if candidates.contains(&candidate2) {
                    continue;
                } else {
                    candidates.push(candidate2);
                }
            }
        }
        candidates
    }
}

fn print_map(height: i64, width: i64, input: &Vec<Coord>) {
    println!("map:");
    for y in 0..height {
        for x in 0..width {
            let mut count = 0;
            for val in input {
                if *val == (Coord { x, y }) {
                    count += 1;
                }
            }
            if count == 0 {
                print!(".");
            } else {
                print!("{count}");
            }
        }
        print!("\n");
    }
}

pub fn process_part1(input: &str) -> String {
    let data = parse(input);
    let pairs = generate_coord_pairs(&data);
    //println!("{pairs:?}, length: {}", pairs.len());
    let mut total = Vec::new();
    for pair in pairs {
        let new_values = generate_candidates(data.width, data.height, pair, true);
        for value in new_values {
            if total.contains(&value) {
                continue;
            } else {
                total.push(value);
            }
        }
    }
    //print_map(data.height, data.width, &total);
    total.len().to_string()
}

pub fn process_part2(input: &str) -> String {
    let data = parse(input);
    let pairs = generate_coord_pairs(&data);
    //println!("{pairs:?}, length: {}", pairs.len());
    let mut total = Vec::new();
    for pair in pairs {
        let new_values = generate_candidates(data.width, data.height, pair, false);
        for value in new_values {
            if total.contains(&value) {
                continue;
            } else {
                total.push(value);
            }
        }
    }
    //print_map(data.height, data.width, &total);
    total.len().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input() {
        let file = include_str!("../test-input-1.txt");
        assert_eq!(process_part1(file), "14");
        assert_eq!(process_part2(file), "34");
    }
    #[test]
    fn candidate_test1() {
        let vec = generate_candidates(4, 3, (Coord { x: 1, y: 1 }, Coord { x: 2, y: 1 }), true);
        assert_eq!(vec, vec![Coord { x: 3, y: 1 }, Coord { x: 0, y: 1 }]);
        let vec = generate_candidates(4, 3, (Coord { x: 2, y: 1 }, Coord { x: 1, y: 1 }), true);
        assert_eq!(vec, vec![Coord { x: 0, y: 1 }, Coord { x: 3, y: 1 }]);
    }
    #[test]
    fn candidate_test2() {
        let vec = generate_candidates(3, 4, (Coord { x: 1, y: 1 }, Coord { x: 1, y: 2 }), true);
        assert_eq!(vec, vec![Coord { x: 1, y: 3 }, Coord { x: 1, y: 0 }]);
        let vec = generate_candidates(4, 3, (Coord { x: 2, y: 1 }, Coord { x: 1, y: 1 }), true);
        assert_eq!(vec, vec![Coord { x: 0, y: 1 }, Coord { x: 3, y: 1 }]);
    }
    #[test]
    fn test_2() {
        let file = include_str!("../test-input-2.txt");
        assert_eq!(process_part1(file), "2");
    }
    #[test]
    fn test_3() {
        let file = include_str!("../test-input-3.txt");
        assert_eq!(process_part1(file), "2");
    }
    #[test]
    fn test_4() {
        let file = include_str!("../test-input-4.txt");
        assert_eq!(process_part1(file), "2");
    }
    #[test]
    fn test_5() {
        let file = include_str!("../test-input-5.txt");
        assert_eq!(process_part1(file), "2");
    }
    #[test]
    fn test_6() {
        let file = include_str!("../test-input-6.txt");
        assert_eq!(process_part1(file), "1");
    }
    #[test]
    fn test_7() {
        let file = include_str!("../test-input-7.txt");
        assert_eq!(process_part1(file), "1");
    }
    #[test]
    fn test_8() {
        let file = include_str!("../test-input-8.txt");
        assert_eq!(process_part1(file), "1");
    }
    #[test]
    fn test_9() {
        let file = include_str!("../test-input-9.txt");
        assert_eq!(process_part1(file), "1");
    }
    #[test]
    fn test_10() {
        let file = include_str!("../test-input-10.txt");
        assert_eq!(process_part1(file), "1");
    }
}
