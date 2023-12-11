#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Tile {
    Galaxy,
    Space,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Position {
    y_pos: usize,
    x_pos: usize,
}

impl TryFrom<char> for Tile {
    type Error = ();

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '#' => Ok(Tile::Galaxy),
            '.' => Ok(Tile::Space),
            _ => Err(()),
        }
    }
}

fn find_empty_rows(input: &Vec<Vec<Tile>>) -> Vec<usize> {
    let mut res: Vec<usize> = Vec::default();
    let mut empty_row = true;
    for y in 0..input.len() {
        for x in 0..input[y].len() {
            if input[y][x] == Tile::Space {
                continue;
            } else {
                empty_row = false;
                break;
            }
        }
        if empty_row {
            res.push(y);
        }
        empty_row = true;
    }
    res
}

fn find_empty_columns(input: &Vec<Vec<Tile>>) -> Vec<usize> {
    let mut res: Vec<usize> = Vec::default();
    let mut empty_column = true;
    for x in 0..input[0].len() {
        for y in 0..input.len() {
            if input[y][x] == Tile::Space {
                continue;
            } else {
                empty_column = false;
                break;
            }
        }
        if empty_column {
            res.push(x);
        }
        empty_column = true;
    }
    res
}

fn expand_rows(rows: Vec<usize>, grid: &mut Vec<Vec<Tile>>) {
    for row in rows.iter().rev() {
        let mut inserted_row: Vec<Tile> = Vec::default();
        for _ in 0..grid[*row].len() {
            inserted_row.push(Tile::Space)
        }
        grid.insert(*row, inserted_row)
    }
}

fn expand_columns(columns: Vec<usize>, grid: &mut Vec<Vec<Tile>>) {
    for column in columns.iter().rev() {
        for i in 0..grid.len() {
            grid[i].insert(*column, Tile::Space)
        }
    }
}

fn find_galaxy_positions(grid: &Vec<Vec<Tile>>) -> Vec<Position> {
    let mut list: Vec<Position> = Vec::default();
    for y in 0..grid.len() {
        for x in 0..grid[y].len() {
            if grid[y][x] == Tile::Galaxy {
                list.push(Position { y_pos: y, x_pos: x })
            }
        }
    }
    list
}

fn find_manhattan_distance(pos1: &Position, pos2: &Position) -> usize {
    let mut dist = 0;
    if pos1.y_pos > pos2.y_pos {
        dist += pos1.y_pos - pos2.y_pos;
    } else {
        dist += pos2.y_pos - pos1.y_pos;
    }
    if pos1.x_pos > pos2.x_pos {
        dist += pos1.x_pos - pos2.x_pos;
    } else {
        dist += pos2.x_pos - pos1.x_pos;
    }
    dist
}

fn sum_all_pairs(input: Vec<Position>) -> usize {
    let mut dist = 0;
    for p1 in 0..input.len() {
        for p2 in p1..input.len() {
            dist += find_manhattan_distance(&input[p1], &input[p2])
        }
    }
    dist
}

pub fn process_part1(input: &str) -> String {
    let mut grid: Vec<Vec<Tile>> = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|char| char.try_into().expect("should be a valid tile"))
                .collect::<Vec<Tile>>()
        })
        .collect::<Vec<Vec<Tile>>>();
    /*println!("before expansion");
    for row in grid.iter() {
        for tile in row {
            match tile {
                Tile::Galaxy => print!("#"),
                Tile::Space => print!("."),
            }
        }
        print!("\n")
    }*/
    expand_rows(find_empty_rows(&grid), &mut grid);
    expand_columns(find_empty_columns(&grid), &mut grid);
    /*println!("after expansion");
    for row in grid.iter() {
        for tile in row {
            match tile {
                Tile::Galaxy => print!("#"),
                Tile::Space => print!("."),
            }
        }
        print!("\n")
    }*/
    let positions: Vec<Position> = find_galaxy_positions(&grid);
    //println!("galaxy positions: {positions:?}");
    sum_all_pairs(positions).to_string()
}

fn find_expanded_manhattan_distance(
    pos1: &Position,
    pos2: &Position,
    expanded_columns: &Vec<usize>,
    expanded_rows: &Vec<usize>,
    expansion_rate: usize,
) -> usize {
    let mut dist = 0;
    if pos1.y_pos > pos2.y_pos {
        dist += pos1.y_pos - pos2.y_pos;
        for row in pos2.y_pos..pos1.y_pos {
            if expanded_rows.contains(&row) {
                dist += expansion_rate - 1;
            }
        }
    } else {
        dist += pos2.y_pos - pos1.y_pos;
        for row in pos1.y_pos..pos2.y_pos {
            if expanded_rows.contains(&row) {
                dist += expansion_rate - 1;
            }
        }
    }
    if pos1.x_pos > pos2.x_pos {
        dist += pos1.x_pos - pos2.x_pos;
        for column in pos2.x_pos..pos1.x_pos {
            if expanded_columns.contains(&column) {
                dist += expansion_rate - 1;
            }
        }
    } else {
        dist += pos2.x_pos - pos1.x_pos;
        for column in pos1.x_pos..pos2.x_pos {
            if expanded_columns.contains(&column) {
                dist += expansion_rate - 1;
            }
        }
    }
    dist
}

fn sum_all_pairs_with_expansion(
    input: &Vec<Position>,
    expanded_rows: &Vec<usize>,
    expanded_columns: &Vec<usize>,
    expansion_rate: usize,
) -> usize {
    let mut dist = 0;
    for p1 in 0..input.len() {
        for p2 in p1..input.len() {
            dist += find_expanded_manhattan_distance(
                &input[p1],
                &input[p2],
                expanded_columns,
                expanded_rows,
                expansion_rate,
            )
        }
    }
    dist
}

pub fn process_part2(input: &str, expansion_rate: usize) -> String {
    let grid: Vec<Vec<Tile>> = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|char| char.try_into().expect("should be a valid tile"))
                .collect::<Vec<Tile>>()
        })
        .collect::<Vec<Vec<Tile>>>();
    let expanded_rows = find_empty_rows(&grid);
    let expanded_columns = find_empty_columns(&grid);
    let positions: Vec<Position> = find_galaxy_positions(&grid);
    sum_all_pairs_with_expansion(
        &positions,
        &expanded_rows,
        &expanded_columns,
        expansion_rate,
    )
    .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_input() {
        let file = fs::read_to_string("./test-input-1.txt").unwrap();
        assert_eq!(process_part1(file.as_str()), "374");
        assert_eq!(process_part2(file.as_str(), 10), "1030");
        assert_eq!(process_part2(file.as_str(), 100), "8410");
    }
}
