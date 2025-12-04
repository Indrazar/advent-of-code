enum Square {
    Roll,
    Empty,
}

struct Coord {
    row: usize,
    col: usize,
}

fn check_neighbors(map: &Vec<Vec<Square>>, row: usize, col: usize) -> usize {
    let mut filled = 0;
    //up left
    if row > 0 && col > 0 {
        match map[row - 1][col - 1] {
            Square::Roll => filled += 1,
            Square::Empty => {}
        }
    }
    //up
    if row > 0 {
        match map[row - 1][col] {
            Square::Roll => filled += 1,
            Square::Empty => {}
        }
    }
    //up right
    if row > 0 && col + 1 < map[0].len() {
        match map[row - 1][col + 1] {
            Square::Roll => filled += 1,
            Square::Empty => {}
        }
    }
    //left
    if col > 0 {
        match map[row][col - 1] {
            Square::Roll => filled += 1,
            Square::Empty => {}
        }
    }
    //right
    if col + 1 < map[0].len() {
        match map[row][col + 1] {
            Square::Roll => filled += 1,
            Square::Empty => {}
        }
    }
    //down left
    if row + 1 < map.len() && col > 0 {
        match map[row + 1][col - 1] {
            Square::Roll => filled += 1,
            Square::Empty => {}
        }
    }
    //down
    if row + 1 < map.len() {
        match map[row + 1][col] {
            Square::Roll => filled += 1,
            Square::Empty => {}
        }
    }
    //down right
    if row + 1 < map.len() && col + 1 < map[0].len() {
        match map[row + 1][col + 1] {
            Square::Roll => filled += 1,
            Square::Empty => {}
        }
    }
    filled
}

pub fn process_part1(input: &str) -> String {
    let mut accessible = 0;
    let mut map: Vec<Vec<Square>> = Vec::new();
    for line in input.lines() {
        let mut map_row: Vec<Square> = Vec::new();
        for char in line.chars() {
            match char {
                '.' => map_row.push(Square::Empty),
                '@' => map_row.push(Square::Roll),
                _ => panic!("invalid character {char}"),
            }
        }
        map.push(map_row);
    }
    for (row, line) in map.iter().enumerate() {
        for (col, square) in line.iter().enumerate() {
            match square {
                Square::Empty => {}
                Square::Roll => {
                    if check_neighbors(&map, row, col) < 4 {
                        accessible += 1;
                    }
                }
            }
        }
    }
    accessible.to_string()
}

pub fn process_part2(input: &str) -> String {
    let mut removals = 0;
    let mut map: Vec<Vec<Square>> = Vec::new();
    for line in input.lines() {
        let mut map_row: Vec<Square> = Vec::new();
        for char in line.chars() {
            match char {
                '.' => map_row.push(Square::Empty),
                '@' => map_row.push(Square::Roll),
                _ => panic!("invalid character {char}"),
            }
        }
        map.push(map_row);
    }
    let mut removals_this_loop = 1;
    while removals_this_loop > 0 {
        removals_this_loop = 0;
        let mut removal_list: Vec<Coord> = Vec::new();
        for (row, line) in map.iter().enumerate() {
            for (col, square) in line.iter().enumerate() {
                match square {
                    Square::Empty => {}
                    Square::Roll => {
                        if check_neighbors(&map, row, col) < 4 {
                            removal_list.push(Coord { row, col });
                            removals += 1;
                            removals_this_loop += 1;
                        }
                    }
                }
            }
        }
        for spot in removal_list {
            map[spot.row][spot.col] = Square::Empty;
        }
    }
    removals.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input() {
        let file = include_str!("../test-input-1.txt");
        assert_eq!(process_part1(file), "13");
        assert_eq!(process_part2(file), "43");
    }
}
