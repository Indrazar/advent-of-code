use std::{fmt::Display, iter::zip};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Block {
    Empty,
    Data(usize),
}

impl Display for Block {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Block::Empty => {
                write!(f, ".")
            }
            Block::Data(val) => {
                write!(f, "{val}")
            }
        }
    }
}

struct Disk {
    data: Vec<Block>,
}

impl Disk {
    fn is_blockwise_left_packed(&self) -> bool {
        // assumes there will be at least one empty block at the end
        let mut at_least_one_data = false;
        let mut at_least_one_empty = false;
        for block in self.data.iter().rev() {
            match block {
                Block::Empty => {
                    at_least_one_empty = true;
                    if at_least_one_data {
                        return false;
                    }
                }
                Block::Data(_) => {
                    at_least_one_data = true;
                    if !at_least_one_empty {
                        return false;
                    }
                }
            }
        }
        true
    }
    fn leftmost_empty_location(&self) -> usize {
        for (i, val) in self.data.iter().enumerate() {
            match val {
                Block::Empty => return i,
                Block::Data(_) => continue,
            }
        }
        panic!("no empty location found in leftmost_empty_location");
    }
    fn rightmost_data_location(&self) -> usize {
        for (i, val) in self.data.iter().rev().enumerate() {
            match val {
                Block::Empty => continue,
                Block::Data(_) => return self.data.len() - 1 - i,
            }
        }
        panic!("no data found in rightmost_empty_location");
    }
    fn pack_one_block(&mut self) {
        // if self.is_left_packed() {
        //     panic!("already packed");
        // }
        let leftmost_empty_location: usize = self.leftmost_empty_location();
        let rightmost_data_location: usize = self.rightmost_data_location();
        //println!("swapping: {leftmost_empty_location} with {rightmost_data_location}");
        self.data
            .swap(leftmost_empty_location, rightmost_data_location);
    }
    fn calculate_checksum(&self) -> usize {
        let mut total = 0;
        for (i, block) in self.data.iter().enumerate() {
            match block {
                Block::Empty => {}
                Block::Data(id) => {
                    total += i * id;
                }
            }
        }
        total
    }
    fn get_highest_file_id(&self) -> usize {
        match self.data[self.rightmost_data_location()] {
            Block::Empty => {
                panic!("must have data here")
            }
            Block::Data(id) => id,
        }
    }
    fn start_and_end_of_file(&self, target_id: usize) -> (usize, usize) {
        let mut start: Option<usize> = None;
        let mut end: Option<usize> = None;
        for (loc, block) in self.data.iter().enumerate() {
            match block {
                Block::Empty => {
                    if start.is_some() {
                        end = Some(loc - 1);
                        break;
                    }
                }
                Block::Data(id) => {
                    if *id == target_id {
                        if start.is_none() {
                            start = Some(loc)
                        }
                    } else if start.is_some() {
                        end = Some(loc - 1);
                        break;
                    }
                }
            }
        }
        let start = start.expect("there should be a start");
        let end = end.unwrap_or(self.data.len() - 1);
        (start, end)
    }
    fn try_get_empty_blocks_of_size_before_loc(&self, size: usize, loc: usize) -> Option<usize> {
        let mut contiguous_empty_so_far = 0;
        let mut start_of_empty: Option<usize> = None;
        for i in 0..loc {
            match self.data[i] {
                Block::Empty => {
                    if start_of_empty.is_none() {
                        start_of_empty = Some(i);
                    }
                    contiguous_empty_so_far += 1;
                    if contiguous_empty_so_far == size {
                        return start_of_empty;
                    } else {
                        continue;
                    }
                }
                Block::Data(_) => {
                    contiguous_empty_so_far = 0;
                    start_of_empty = None;
                }
            }
        }
        None
    }
    fn attempt_move_file_by_id(&mut self, id: usize) {
        let (start_of_file, end_of_file) = self.start_and_end_of_file(id);
        let size_of_file = end_of_file - start_of_file + 1;
        //println!("moving file: {id}, starting at: {start_of_file} ending: {end_of_file}, size: {size_of_file}");
        match self.try_get_empty_blocks_of_size_before_loc(size_of_file, start_of_file) {
            Some(loc) => {
                let file_iter = start_of_file..=end_of_file;
                let empty_iter = loc..(loc + size_of_file);
                //if file_iter.clone().count() != empty_iter.clone().count() {
                //    panic!("must have same number of items")
                //}
                for (file_loc, empty_loc) in zip(file_iter, empty_iter) {
                    //println!("swapping {file_loc} with {empty_loc}");
                    self.data.swap(empty_loc, file_loc);
                }
            }
            None => {
                //println!("could not move file: {id}")
            }
        }
    }
    // this function can only be called once
    fn pack_by_file(&mut self) {
        let high_id = self.get_highest_file_id();
        for id in (0..=high_id).rev() {
            self.attempt_move_file_by_id(id);
        }
    }
}

impl Display for Disk {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut data_str = String::with_capacity(self.data.len());
        for block in &self.data {
            data_str += format!("{block}").as_str();
        }
        write!(f, "{data_str}")
    }
}

fn parse(input: &str) -> Disk {
    let mut data: Vec<Block> = Vec::with_capacity(input.len());
    let mut data_mode = true;
    let mut current_id: usize = 0;
    for ch in input.chars() {
        if !ch.is_ascii_digit() {
            panic!("invalid character in parse: {ch}")
        }
        let size: i64 = (ch as u8 - 48).into();
        //print!("{size}")
        if data_mode {
            for _ in 0..size {
                data.push(Block::Data(current_id));
            }
            current_id += 1;
        } else {
            for _ in 0..size {
                data.push(Block::Empty)
            }
        }
        data_mode = !data_mode;
    }
    //let disk = Disk { data };
    //println!("data:\n{disk}");
    //println!("is done: {}", disk.is_left_packed());
    //disk
    Disk { data }
}

pub fn process_part1(input: &str) -> String {
    let mut disk = parse(input);
    while !disk.is_blockwise_left_packed() {
        disk.pack_one_block();
    }
    //println!("{disk}");
    disk.calculate_checksum().to_string()
}

pub fn process_part2(input: &str) -> String {
    let mut disk = parse(input);
    //println!("{disk}");
    disk.pack_by_file();
    //println!("{disk}");
    disk.calculate_checksum().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input() {
        let file = include_str!("../test-input-1.txt");
        assert_eq!(process_part1(file), "1928");
        assert_eq!(process_part2(file), "2858");
    }
}
