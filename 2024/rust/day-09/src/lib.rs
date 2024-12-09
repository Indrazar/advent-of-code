use std::fmt::Display;

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
    fn is_left_packed(&self) -> bool {
        // assumes there will be at least one empty block at the end
        let mut at_least_one_data = false;
        let mut at_least_one_empty = false;
        for block in self.data.iter().rev() {
            match block {
                Block::Empty => {
                    at_least_one_empty = true;
                    if at_least_one_data {
                        return false;
                    } else {
                    }
                }
                Block::Data(_) => {
                    at_least_one_data = true;
                    if !at_least_one_empty {
                        return false;
                    } else {
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
    fn pack_one_step(&mut self) {
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
                Block::Empty => return total,
                Block::Data(id) => {
                    total += i * id;
                }
            }
        }
        total
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
    while !disk.is_left_packed() {
        disk.pack_one_step();
    }
    //println!("{disk}");
    disk.calculate_checksum().to_string()
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
        assert_eq!(process_part1(file), "1928");
    }
}
