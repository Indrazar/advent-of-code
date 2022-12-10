use std::fmt::Debug;

#[derive(Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right
}

#[derive(Debug)]
struct Move {
    direction: Direction,
    size: usize,
}

struct GridData {
    tail_visit: bool,
}

impl Debug for GridData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.tail_visit {
            true => write!(f, "#"),
            false => write!(f, "."),
        }
    }
}

struct HeadTailGrid {
    grid: Vec<Vec<GridData>>,
    segments: usize,
    seg_x: Vec<usize>,
    seg_y: Vec<usize>,
    rows: usize,
    cols: usize
}

impl Debug for HeadTailGrid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut grid_output = String::new();
        for row in &self.grid {
            for col in row {
                grid_output += format!("{:?}", col).as_str();
            }
            grid_output.push('\n');
        }
        write!(f, "grid:\n{}\nsegments: {}\nseg_x: {:?}\nseg_y: {:?}\nrows: {}\ncols: {}", 
        grid_output, self.segments, self.seg_x, self.seg_y, self.rows, self.cols)
    }
}

impl HeadTailGrid {
    fn new(tails: u32) -> HeadTailGrid {
        let mut grid: Vec<Vec<GridData>> = Vec::new();
        let mut row: Vec<GridData> = Vec::new();
        let col: GridData = GridData { tail_visit: true };
        row.reserve(300);
        row.push(col);
        grid.reserve(500);
        grid.push(row);
        let mut seg_x = Vec::new();
        let mut seg_y = Vec::new();
        for _ in 0..tails+1 {
            seg_x.push(0);
            seg_y.push(0);
        }
        HeadTailGrid{ grid, segments: (tails as usize) +1, seg_x, seg_y, rows: 1, cols: 1 }
    }
    fn process_moves(&mut self, input: Vec<Move>) {
        for item in input {
            for _ in 0..item.size {
                match item.direction {
                    Direction::Up => self.head_up(),
                    Direction::Down => self.head_down(),
                    Direction::Left => self.head_left(),
                    Direction::Right => self.head_right(),
                }
            }
        }
    }
    fn count_tail_visits(&self) -> usize {
        self.grid
            .iter()
            .fold(0, |acc, item| {
                acc + item.iter().fold(0, |deep_acc, deep_item| { 
                    if deep_item.tail_visit {deep_acc+1} else {deep_acc} 
                })
            })
    }
    fn check_adjacency(&self, segment_num: usize) -> bool {
        if segment_num == 0 {
            panic!(); // why are you here go away
        }
        let s = segment_num;
        let h = segment_num - 1;
        // 9 horrible if blocks
        // x x x
        // x T x
        // x x x
        if self.seg_x[h] == self.seg_x[s] && self.seg_y[h] == self.seg_y[s] { //same
            return true
        } else if self.seg_x[h] == self.seg_x[s]+1 && self.seg_y[h] == self.seg_y[s] { //right
            return true
        } else if self.seg_x[s] != 0 && self.seg_x[h] == self.seg_x[s]-1 && self.seg_y[h] == self.seg_y[s] { //left
            return true
        } else if self.seg_x[h] == self.seg_x[s] && self.seg_y[h] == self.seg_y[s]+1 { //down
            return true
        } else if self.seg_y[s] != 0 && self.seg_x[h] == self.seg_x[s] && self.seg_y[h] == self.seg_y[s]-1 { //up
            return true
        } else if self.seg_x[s] != 0 && self.seg_y[s] != 0 && self.seg_x[h] == self.seg_x[s]-1 && self.seg_y[h] == self.seg_y[s]-1 { //up left
            return true
        } else if self.seg_y[s] != 0 && self.seg_x[h] == self.seg_x[s]+1 && self.seg_y[h] == self.seg_y[s]-1 { //up right
            return true
        } else if self.seg_x[h] == self.seg_x[s]+1 && self.seg_y[h] == self.seg_y[s]+1 { //down right
            return true
        } else if self.seg_x[s] != 0 && self.seg_x[h] == self.seg_x[s]-1 && self.seg_y[h] == self.seg_y[s]+1 { //down left
            return true
        } else { false }
    }
    fn update_tails(&mut self) {
        for s in 1..self.segments {
            if self.check_adjacency(s) { continue; }
            let h = s-1;
            // eight horrible if checks
            // tail moves based on head distance, catching up when needed
            if self.seg_x[s] == self.seg_x[h] ||  self.seg_y[s] == self.seg_y[h] {
                // no diag allowed
                if self.seg_y[s] != 0 { // check if we can go up
                    self.seg_y[s] -= 1;
                    if self.check_adjacency(s) { continue; }
                    else {self.seg_y[s] += 1;}
                }
                if self.seg_y[s] != self.rows-1 { // check if we can go down
                    self.seg_y[s] += 1;
                    if self.check_adjacency(s) { continue; }
                    else {self.seg_y[s] -= 1;}
                }
                if self.seg_x[s] != 0 { // check if we can go left
                    self.seg_x[s] -= 1;
                    if self.check_adjacency(s) { continue; }
                    else {self.seg_x[s] += 1}
                }
                if self.seg_x[s] != self.cols-1 { // check if we can go right
                    self.seg_x[s] += 1;
                    if self.check_adjacency(s) { continue; }
                    else {self.seg_y[s] -= 1;}
                }
            } else { // tail MUST move diag
            // up left
                if (self.seg_x[s] != 0) && (self.seg_y[s] != 0) { // check if we can go up+left
                    self.seg_x[s] -= 1;
                    self.seg_y[s] -= 1;
                    if self.check_adjacency(s) { continue; }
                    else {self.seg_x[s] += 1; self.seg_y[s] += 1;}
                }
                if (self.seg_x[s] != self.cols-1) && (self.seg_y[s] != 0) { // check if we can go up+right
                    self.seg_x[s] += 1;
                    self.seg_y[s] -= 1;
                    if self.check_adjacency(s) { continue; }
                    else {self.seg_x[s] -= 1; self.seg_y[s] += 1;}
                }
                if (self.seg_x[s] != self.cols-1) && (self.seg_y[s] != self.rows-1) { // check if we can go down+right
                    self.seg_x[s] += 1;
                    self.seg_y[s] += 1;
                    if self.check_adjacency(s) {
                        continue; 
                    }
                    else {self.seg_x[s] -= 1; self.seg_y[s] -= 1;}
                }
                if (self.seg_x[s] != 0) && (self.seg_y[s] != self.rows-1) { // check if we can go down+left
                    self.seg_x[s] -= 1;
                    self.seg_y[s] += 1;
                    if self.check_adjacency(s) {
                        continue; 
                    }
                    else {self.seg_x[s] += 1; self.seg_y[s] -= 1;}
                }
            }
            // could not resolve update
            panic!("could not find a valid place to move the tail");
        }
        self.grid[self.seg_y[self.segments-1]][self.seg_x[self.segments-1]].tail_visit = true; 
    }
    fn head_up(&mut self) {
        if self.seg_y[0] == 0 { // need new row at START
            // generate new row
            let mut row: Vec<GridData> = Vec::new();
            row.reserve(300);
            for _ in 0..self.cols {
                let col = GridData { tail_visit: false };
                row.push(col)
            }
            self.grid.insert(0, row); // insert row at the start
            //adjust coords since we are operating in quadrant 3
            for s in 0..self.segments {
                self.seg_y[s] += 1;
            }
            self.rows += 1;
        }
        // update head location
        self.seg_y[0] -= 1;
        self.update_tails();
    }
    fn head_down(&mut self) {
        if self.seg_y[0] == self.rows-1 { // need new row at END
            // generate new row
            let mut row: Vec<GridData> = Vec::new();
            row.reserve(300);
            for _ in 0..self.cols {
                let col = GridData { tail_visit: false };
                row.push(col)
            }
            self.grid.push(row); // insert row at the end
            // in quadrant 3 coords do not need to be updated for rows going deeper
            self.rows += 1;
        }
        // update head location
        self.seg_y[0] += 1;
        self.update_tails();
    }
    fn head_left(&mut self) {
        if self.seg_x[0] == 0 { // need new col on the front of each row
            // generate new col
            for row in &mut self.grid {
                let col = GridData { tail_visit: false };
                row.insert(0, col)
            }
            //adjust coords since we are operating in quadrant 3
            for s in 0..self.segments {
                self.seg_x[s] += 1;
            }
            self.cols += 1;
        }
        // update head location
        self.seg_x[0] -= 1;
        self.update_tails();
    }
    fn head_right(&mut self) {
        if self.seg_x[0] == self.cols-1 { // need new row at END
            // generate new col
            for row in &mut self.grid {
                let col = GridData { tail_visit: false };
                row.push(col)
            }
            // in quadrant 3 coords do not need to be updated for rows going deeper
            self.cols += 1;
        }
        // update head location
        self.seg_x[0] += 1;
        self.update_tails();
    }
}

fn parse_steps(input: &str) -> Vec<Move> {
    let mut result = Vec::new();
    for line in input.lines() {
        let direction = match line.chars().next() {
            Some('U') => Direction::Up,
            Some('D') => Direction::Down,
            Some('L') => Direction::Left,
            Some('R') => Direction::Right,
            Some(_) => todo!(), // invalid input
            None => todo!(), // invalid input
        };
        let size = match line.split_whitespace().nth(1) {
            Some(number) => number.parse::<u32>().expect("invalid input, expected number"),
            None => todo!(), // invalid input
        };
        result.push(Move { direction, size: size as usize })
    }
    result
}

pub fn process_part1(input: &str) -> String {
    let moves = parse_steps(input);
    let mut grid = HeadTailGrid::new(1);
    grid.process_moves(moves);
    dbg!(&grid);
    grid.count_tail_visits().to_string()
}

pub fn process_part2(input: &str) -> String {
    let moves = parse_steps(input);
    let mut grid = HeadTailGrid::new(9);
    grid.process_moves(moves);
    dbg!(&grid);
    grid.count_tail_visits().to_string()
}


#[cfg(test)]
mod tests {
    use std::fs;
    use super::*;

    #[test]
    fn test_input() {
        let file = fs::read_to_string("./test-input-1.txt").unwrap();
        assert_eq!(process_part1(file.as_str()), "13");
        assert_eq!(process_part2(file.as_str()), "1");
        let file2 = fs::read_to_string("./test-input-2.txt").unwrap();
        assert_eq!(process_part2(file2.as_str()), "36");
    }
}
