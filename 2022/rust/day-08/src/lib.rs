fn parse_grid(input: &str) -> Vec<Vec<u32>> {
    let grid: Vec<Vec<u32>> = input
        .lines()
        .fold(Vec::new(), |mut row, line| {
            row.push(
                line.chars()
                    .fold(Vec::new(), |mut col: Vec<u32>, item: char| {
                        col.push(item.to_digit(10).unwrap()); // assume valid input
                        col
                    }),
            );
            row
        });
    grid
}

fn check_inward(grid: Vec<Vec<u32>>) -> u32 {
    let mut down_visible_grid: Vec<Vec<bool>> = Vec::new();
    for _ in 0..grid.len() {
        let mut row: Vec<bool> = Vec::new();
        for _ in 0..grid[0].len() {
            row.push(false);
        }
        down_visible_grid.push(row);
    }
    let mut right_visible_grid = down_visible_grid.clone();
    let mut up_visible_grid = down_visible_grid.clone();
    let mut left_visible_grid = down_visible_grid.clone();
    let mut visible_grid = down_visible_grid.clone();
    //cast down rays
    for y in 0..grid.len() {
        for x in 0..grid[0].len() {
            if y == 0 {
                down_visible_grid[y][x] = true; // edge
            } else if down_visible_grid[y-1][x] && grid[y-1][x] < grid[y][x] { //visible and smaller
                down_visible_grid[y][x] = true;
            } else if grid[y-1][x] >= grid[y][x] { // if neghbor is taller or same we are not visible 
                continue; // already set false
            }
            else if grid[y-1][x] < grid[y][x] { //neighbor is not visible and smaller
                let mut is_tallest_so_far = true; // check are we tallest tree so far
                for look_y in (0..(y-1)).rev() { // check all trees so far
                    if grid[look_y][x] >= grid[y][x] { //if any are same or taller we stop
                        is_tallest_so_far = false;
                        break;
                    }                
                }
                down_visible_grid[y][x] = is_tallest_so_far;
            }
        } 
    }
    //cast right rays
    for y in 0..grid.len() {
        for x in 0..grid[0].len() {
            if x == 0 {
                right_visible_grid[y][x] = true; // edge
            } else if right_visible_grid[y][x-1] && grid[y][x-1] < grid[y][x] { //visible and smaller
                right_visible_grid[y][x] = true;
            } else if grid[y][x-1] >= grid[y][x] { // if neghbor is taller or same we are not visible 
                continue; // already set false
            }
            else if grid[y][x-1] < grid[y][x] { //neighbor is not visible and smaller
                let mut is_tallest_so_far = true; // check are we tallest tree so far
                for look_x in (0..(x-1)).rev() { // check all trees so far
                    if grid[y][look_x] >= grid[y][x] { //if any are same or taller we stop
                        is_tallest_so_far = false;
                        break;
                    }                
                }
                right_visible_grid[y][x] = is_tallest_so_far;
            }
        } 
    }
    //cast up rays
    for y in (0..grid.len()).rev() {
        for x in 0..grid[0].len() {
            if y == grid.len()-1 {
                up_visible_grid[y][x] = true; // edge
            } else if up_visible_grid[y+1][x] && grid[y+1][x] < grid[y][x] { //visible and smaller
                up_visible_grid[y][x] = true;
            } else if grid[y+1][x] >= grid[y][x] { // if neghbor is taller or same we are not visible 
                continue; // already set false
            }
            else if grid[y+1][x] < grid[y][x] { //neighbor is not visible and smaller
                let mut is_tallest_so_far = true; // check are we tallest tree so far
                for look_y in (y+1)..grid.len() { // check all trees so far
                    if grid[look_y][x] >= grid[y][x] { //if any are same or taller we stop
                        is_tallest_so_far = false;
                        break;
                    }                
                }
                up_visible_grid[y][x] = is_tallest_so_far;
            }
        } 
    }
    //cast left rays
    for y in 0..grid.len() {
        for x in (0..grid[0].len()).rev() {
            if x == grid[0].len()-1 {
                left_visible_grid[y][x] = true; // edge
            } else if left_visible_grid[y][x+1] && grid[y][x+1] < grid[y][x] { //visible and smaller
                left_visible_grid[y][x] = true;
            } else if grid[y][x+1] >= grid[y][x] { // if neghbor is taller or same we are not visible 
                continue; // already set false
            }
            else if grid[y][x+1] < grid[y][x] { //neighbor is not visible and smaller
                let mut is_tallest_so_far = true; // check are we tallest tree so far
                for look_x in (x+1)..grid[0].len() { // check all trees so far
                    if grid[y][look_x] >= grid[y][x] { //if any are same or taller we stop
                        is_tallest_so_far = false;
                        break;
                    }                
                }
                left_visible_grid[y][x] = is_tallest_so_far;
            }
        } 
    }
    //logical or all grids
    for y in 0..grid.len() {
        for x in 0..grid[0].len() {
            visible_grid[y][x] = down_visible_grid[y][x] || up_visible_grid[y][x] || left_visible_grid[y][x] || right_visible_grid[y][x];
        }
    }
    visible_grid
        .iter()
        .fold(0, |acc, item| {
            acc + item.iter().fold(0, |deep_acc, deep_item| { 
                if *deep_item {deep_acc+1} else {deep_acc} 
            })
        })
}

fn check_outward(grid: Vec<Vec<u32>>) -> u32 {
    let mut down_range_grid: Vec<Vec<u32>> = Vec::new();
    for _ in 0..grid.len() {
        let mut row: Vec<u32> = Vec::new();
        for _ in 0..grid[0].len() {
            row.push(0);
        }
        down_range_grid.push(row);
    }
    let mut right_range_grid = down_range_grid.clone();
    let mut up_range_grid = down_range_grid.clone();
    let mut left_range_grid = down_range_grid.clone();
    let mut range_grid = down_range_grid.clone();

    //cast down rays
    for y in 0..grid.len() {
        for x in 0..grid[0].len() {
            if y == grid.len()-1 {
                down_range_grid[y][x] = 0; // edge
            } else {
                let mut loop_count = 0;
                for look_y in y+1..grid.len() {
                    loop_count += 1;
                    if grid[look_y][x] >= grid[y][x] {
                        break;
                    }
                }
                down_range_grid[y][x] = loop_count;
            }
        }
    } 
    //cast right rays
    for y in 0..grid.len() {
        for x in 0..grid[0].len() {
            if x == grid[0].len()-1 {
                right_range_grid[y][x] = 0; // edge
            } else {
                let mut loop_count = 0;
                for look_x in x+1..grid.len() {
                    loop_count += 1;
                    if grid[y][look_x] >= grid[y][x] {
                        break;
                    }
                }
                right_range_grid[y][x] = loop_count;
            }
        }
    }
    //cast up rays
    for y in 0..grid.len() {
        for x in 0..grid[0].len() {
            if y == 0 {
                up_range_grid[y][x] = 0; // edge
            } else {
                let mut loop_count = 0;
                for look_y in (0..y).rev() {
                    loop_count += 1;
                    if grid[look_y][x] >= grid[y][x] {
                        break;
                    }
                }
                up_range_grid[y][x] = loop_count;
            }
        }
    }
    //cast left rays
    for y in 0..grid.len() {
        for x in 0..grid[0].len() {
            if x == 0 {
                left_range_grid[y][x] = 0; // edge
            } else {
                let mut loop_count = 0;
                for look_x in (0..x).rev() {
                    loop_count += 1;
                    if grid[y][look_x] >= grid[y][x] {
                        break;
                    }
                }
                left_range_grid[y][x] = loop_count;
            }
        }
    }   
    //multiply all grids
    let mut max: u32 = 0;
    for y in 0..grid.len() {
        for x in 0..grid[0].len() {
            range_grid[y][x] = down_range_grid[y][x] * up_range_grid[y][x] * left_range_grid[y][x] * right_range_grid[y][x];
            if range_grid[y][x] > max {
                max = range_grid[y][x];
            }
        }
    }
    max
}

pub fn process_part1(input: &str) -> String {
    let grid = parse_grid(input);
    let count = check_inward(grid);
    count.to_string()
}

pub fn process_part2(input: &str) -> String {
    let grid = parse_grid(input);
    check_outward(grid).to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_input() {
        let file = fs::read_to_string("./test-input-1.txt").unwrap();
        assert_eq!(process_part1(file.as_str()), "21");
        assert_eq!(process_part2(file.as_str()), "8")
    }
}
