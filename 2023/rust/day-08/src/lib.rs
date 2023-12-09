use std::collections::BTreeMap;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Step {
    Left,
    Right,
}

impl TryFrom<char> for Step {
    type Error = ();

    fn try_from(input: char) -> Result<Self, Self::Error> {
        match input {
            'L' => Ok(Step::Left),
            'R' => Ok(Step::Right),
            _ => Err(()),
        }
    }
}

#[derive(Debug, Clone)]
struct Node {
    left: String,
    right: String,
}

pub fn gcd(mut n: u64, mut m: u64) -> u64 {
    if n == 0 {
        return m;
    } else if m == 0 {
        return n;
    }
    let k = {
        let k_n = n.trailing_zeros();
        let k_m = m.trailing_zeros();
        n >>= k_n;
        m >>= k_m;
        core::cmp::min(k_n, k_m)
    };
    loop {
        if n > m {
            core::mem::swap(&mut n, &mut m);
        }
        m -= n;
        if m == 0 {
            return n << k;
        }
        m >>= m.trailing_zeros();
    }
}

fn lcm(n: u64, m: u64) -> u64 {
    n * m / gcd(n, m)
}

pub fn process_part1(input: &str) -> String {
    let mut lines_itr = input.lines();
    let steps: Vec<Step> = lines_itr
        .next()
        .expect("should be a set of steps")
        .chars()
        .map(|i| {
            i.try_into()
                .expect(format!("{i} should be an R or L").as_str())
        })
        .collect();
    let nodes: BTreeMap<String, Node> = lines_itr
        .filter_map(|line| {
            let node_name = match line.split(" = ").nth(0) {
                Some(name) => name.trim(),
                None => return None,
            };
            let left_node = match line.split(&['(', ','][..]).nth(1) {
                Some(name) => name.trim(),
                None => return None,
            };
            let right_node = match line.split(&[',', ')'][..]).nth(1) {
                Some(name) => name.trim(),
                None => return None,
            };
            Some((
                node_name.to_string(),
                Node {
                    left: left_node.to_string(),
                    right: right_node.to_string(),
                },
            ))
        })
        .collect();
    let mut step_count: u64 = 0;
    let mut current_node: String = String::from("AAA");
    let mut direction_iter = steps.iter();
    loop {
        match direction_iter.next() {
            Some(Step::Left) => {
                current_node = nodes
                    .get(&current_node)
                    .expect(format!("{current_node} not in list").as_str())
                    .left
                    .clone();
            }
            Some(Step::Right) => {
                current_node = nodes
                    .get(&current_node)
                    .expect(format!("{current_node} not in list").as_str())
                    .right
                    .clone();
            }
            None => {
                direction_iter = steps.iter();
                continue;
            }
        }
        step_count += 1;
        if current_node == String::from("ZZZ") {
            break;
        }
    }
    step_count.to_string()
}

pub fn process_part2(input: &str) -> String {
    let mut lines_itr = input.lines();
    let steps: Vec<Step> = lines_itr
        .next()
        .expect("should be a set of steps")
        .chars()
        .map(|i| {
            i.try_into()
                .expect(format!("{i} should be an R or L").as_str())
        })
        .collect();
    let nodes: BTreeMap<String, Node> = lines_itr
        .filter_map(|line| {
            let node_name = match line.split(" = ").nth(0) {
                Some(name) => name.trim(),
                None => return None,
            };
            let left_node = match line.split(&['(', ','][..]).nth(1) {
                Some(name) => name.trim(),
                None => return None,
            };
            let right_node = match line.split(&[',', ')'][..]).nth(1) {
                Some(name) => name.trim(),
                None => return None,
            };
            Some((
                node_name.to_string(),
                Node {
                    left: left_node.to_string(),
                    right: right_node.to_string(),
                },
            ))
        })
        .collect();
    let mut step_counts: Vec<u64> = Vec::default();
    let mut current_nodes: Vec<String> = Vec::default();
    for node_name in nodes.keys() {
        if node_name.ends_with('A') {
            current_nodes.push(node_name.clone())
        }
    }
    for current_node in current_nodes.iter_mut() {
        let mut step_count = 0;
        let mut direction_iter = steps.iter();
        loop {
            match direction_iter.next() {
                Some(Step::Left) => {
                    *current_node = nodes
                        .get(current_node)
                        .expect(format!("{current_node} not in list").as_str())
                        .left
                        .clone();
                }
                Some(Step::Right) => {
                    *current_node = nodes
                        .get(current_node)
                        .expect(format!("{current_node} not in list").as_str())
                        .right
                        .clone();
                }
                None => {
                    direction_iter = steps.iter();
                    continue;
                }
            }
            step_count += 1;
            if current_node.ends_with('Z') {
                break;
            }
        }
        step_counts.push(step_count)
    }
    let mut res = 1;
    for step_count in step_counts.iter() {
        res = lcm(res, *step_count)
    }
    res.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_input() {
        let file1 = fs::read_to_string("./test-input-1.txt").unwrap();
        let file2 = fs::read_to_string("./test-input-2.txt").unwrap();
        let file3 = fs::read_to_string("./test-input-3.txt").unwrap();
        assert_eq!(process_part1(file1.as_str()), "2");
        assert_eq!(process_part1(file2.as_str()), "6");
        assert_eq!(process_part2(file3.as_str()), "6");
    }
}
