fn left(distance: i32, start: &mut i32) {
    let res = (*start - distance) % 100;
    if res < 0 {
        *start = res + 100
    } else {
        *start = res
    }
    //println!("L{distance}: {}", *start);
}

fn right(distance: i32, start: &mut i32) {
    *start = (*start + distance) % 100;
    //println!("R{distance}: {}", *start);
}

pub fn process_part1(input: &str) -> String {
    let mut zero_count = 0;
    let mut value = 50;
    for line in input.lines() {
        let (direction, count) = line.split_at(1);
        match direction {
            "L" => left(count.parse::<i32>().unwrap(), &mut value),
            "R" => right(count.parse::<i32>().unwrap(), &mut value),
            _ => panic!("this isn't a valid letter"),
        }
        if value == 0 {
            zero_count += 1;
        }
    }
    zero_count.to_string()
}

struct Dial {
    zero_passes: i32,
    reading: i32,
}

impl Dial {
    fn single_left(&mut self) {
        match self.reading {
            0 => self.reading = 99,
            1 => {
                self.reading = 0;
                self.zero_passes += 1;
            }
            2..=99 => {
                self.reading -= 1;
            }
            _ => {
                panic!("dial reading is invalid: {}", self.reading)
            }
        }
    }
    fn single_right(&mut self) {
        match self.reading {
            99 => {
                self.reading = 0;
                self.zero_passes += 1;
            }
            0..=98 => {
                self.reading += 1;
            }
            _ => {
                panic!("dial reading is invalid: {}", self.reading)
            }
        }
    }
    pub fn left_slow(&mut self, distance: i32) {
        for _ in 1..=distance {
            self.single_left();
        }
    }
    pub fn right_slow(&mut self, distance: i32) {
        for _ in 1..=distance {
            self.single_right();
        }
    }
    pub fn left_faster(&mut self, distance: i32) {
        let start = self.reading;
        if start > 0 {
            self.reading -= distance;
            while self.reading < 0 {
                self.reading += 100;
                self.zero_passes += 1;
            }
        } else if start == 0 {
            let mut first = true;
            self.reading -= distance;
            while self.reading < 0 {
                self.reading += 100;
                if first {
                    first = false;
                } else {
                    self.zero_passes += 1
                }
            }
        } else {
            panic!("dial reading is invalid: {}", self.reading)
        }
        if self.reading == 0 {
            self.zero_passes += 1;
        }
    }
    pub fn right_faster(&mut self, distance: i32) {
        let start = self.reading;
        if start > 99 || start < 0 {
            panic!("dial reading is invalid {}", self.reading)
        }
        self.reading += distance;
        while self.reading > 99 {
            self.reading -= 100;
            self.zero_passes += 1;
        }
    }
}

pub fn process_part2(input: &str) -> String {
    let mut safe: Dial = Dial {
        zero_passes: 0,
        reading: 50,
    };
    for line in input.lines() {
        let (direction, count) = line.split_at(1);
        match direction {
            "L" => safe.left_faster(count.parse::<i32>().unwrap()),
            "R" => safe.right_faster(count.parse::<i32>().unwrap()),
            _ => panic!("this isn't a valid letter"),
        }
    }
    safe.zero_passes.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input() {
        let file = include_str!("../test-input-1.txt");
        assert_eq!(process_part1(file), "3");
        assert_eq!(process_part2(file), "6");
        let file = include_str!("../input.txt");
        assert_eq!(process_part2(file), "5923");
    }
}
