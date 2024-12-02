#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum LevelKind {
    Safe,
    NotSafe,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Direction {
    Up,
    Down,
    Neither,
}

fn process_levels(levels: &Vec<i32>) -> LevelKind {
    if levels.len() < 2 {
        panic!("incorrectly formatted input")
    }
    let mut current_direction: Direction = Direction::Neither;
    let mut last_level: i32 = 0;
    for (i, level) in levels.iter().enumerate() {
        //println!("i: {i}, level: {level}, last_level: {last_level}");
        if i == 0 {
            last_level = *level
        } else if i == 1 {
            if (*level - last_level).abs() >= 4 || (*level - last_level).abs() == 0 {
                //println!("check differ failed, not safe");
                return LevelKind::NotSafe;
            }
            if *level < last_level {
                //println!("setting direction down");
                current_direction = Direction::Down
            } else if *level > last_level {
                //println!("setting direction up");
                current_direction = Direction::Up
            } else {
                //println!("they're equal? not safe");
                return LevelKind::NotSafe;
            }
            last_level = *level
        } else {
            if (*level - last_level).abs() >= 4 || (*level - last_level).abs() == 0 {
                //println!("check differ failed, not safe");
                return LevelKind::NotSafe;
            }
            if *level < last_level && current_direction == Direction::Up {
                //println!("was going up, now down? not safe");
                return LevelKind::NotSafe;
            } else if *level > last_level && current_direction == Direction::Down {
                //println!("was going down, now up? not safe");
                return LevelKind::NotSafe;
            }
            last_level = *level
        }
    }
    //println!("safe!");
    LevelKind::Safe
}

pub fn process_part1(input: &str) -> String {
    let mut safe_reports: i32 = 0;
    input.lines().for_each(|line| {
        let levels: Vec<i32> = line
            .split_whitespace()
            .flat_map(|x| x.parse::<i32>())
            .collect();
        match process_levels(&levels) {
            LevelKind::NotSafe => (),
            LevelKind::Safe => safe_reports += 1,
        }
    });
    format!("{safe_reports}")
}

fn process_updated_levels(levels: &Vec<i32>) -> LevelKind {
    if process_levels(levels) == LevelKind::Safe {
        return LevelKind::Safe;
    }
    for i in 0..levels.len() {
        let mut new_levels = levels.clone();
        new_levels.remove(i);
        if process_levels(&new_levels) == LevelKind::Safe {
            return LevelKind::Safe;
        }
    }
    LevelKind::NotSafe
}

pub fn process_part2(input: &str) -> String {
    let mut safe_reports: i32 = 0;
    input.lines().for_each(|line| {
        let levels: Vec<i32> = line
            .split_whitespace()
            .flat_map(|x| x.parse::<i32>())
            .collect();
        match process_updated_levels(&levels) {
            LevelKind::NotSafe => (),
            LevelKind::Safe => safe_reports += 1,
        }
    });
    format!("{safe_reports}")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input() {
        let file = include_str!("../test-input-1.txt");
        assert_eq!(process_part1(file), "2");
        assert_eq!(process_part2(file), "4");
    }
}
