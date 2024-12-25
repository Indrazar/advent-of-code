#[derive(Debug, Clone, Copy)]
enum KeyLock {
    Lock([i8; 5]),
    Key([i8; 5]),
}

impl KeyLock {
    fn inner(&self) -> [i8; 5] {
        match self {
            KeyLock::Lock(i) => *i,
            KeyLock::Key(i) => *i,
        }
    }
    fn is_lock(&self) -> bool {
        match self {
            KeyLock::Lock(_) => true,
            KeyLock::Key(_) => false,
        }
    }
}

fn parse(input: &str) -> Vec<KeyLock> {
    let mut res: Vec<KeyLock> = Vec::new();
    for (c, split) in input.split("\n\n").enumerate() {
        let mut is_lock = false;
        let mut stack: [i8; 5] = [0, 0, 0, 0, 0];
        for (y, line) in split.lines().enumerate() {
            if y == 0 && line == "#####" {
                is_lock = true;
            } else if y == 6 && line == "#####" {
            } else {
                for (x, ch) in line.chars().enumerate() {
                    match ch {
                        '#' => {
                            stack[x] += 1;
                        }
                        '.' => {}
                        unknown => {
                            panic!("unknown character in parse: {unknown}")
                        }
                    }
                }
            }
        }

        match is_lock {
            true => {
                res.push(KeyLock::Lock(stack));
            }
            false => res.push(KeyLock::Key(stack)),
        }
    }

    res
}

pub fn process_part1(input: &str) -> String {
    let keys_locks = parse(input);
    let keys: Vec<KeyLock> = keys_locks
        .iter()
        .filter(|x| !x.is_lock())
        .copied()
        .collect();
    let locks: Vec<KeyLock> = keys_locks.iter().filter(|x| x.is_lock()).copied().collect();
    let mut doesnt_fit = 0;
    for lock in locks.iter() {
        for key in keys.iter() {
            for i in 0..5 {
                //println!("comparing {:?} to {:?}", lock.inner(), key.inner());
                if (lock.inner()[i] + key.inner()[i]) > 5 {
                    // does not fit
                    doesnt_fit += 1;
                    break;
                } else {
                    // does fit
                }
            }
        }
    }
    let fit_count = (keys.len() * locks.len()) - doesnt_fit;
    fit_count.to_string()
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
        assert_eq!(process_part1(file), "3");
    }
}
