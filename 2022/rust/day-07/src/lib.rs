use std::collections::BTreeMap;

fn parse_tree(input: &str) -> BTreeMap<Vec<String>, u64> {
    let mut tree: BTreeMap<Vec<String>, u64> = BTreeMap::new();
    let mut cwd: Vec<String> = Vec::new();
    for line in input.lines() { //.enumerate() {
        match line.split_whitespace().next() {
            Some("dir") => { continue; } // the names will propagate from cd
            Some("$") => {
                match line.split_whitespace().nth(1) {
                    Some("ls") => continue, // don't care about the command itself
                    Some("cd") => {
                        match line.split_whitespace().nth(2) {
                            Some("..") => { cwd.pop(); } // go up a directory
                            Some("/") => cwd.push("/".to_string()), //no double forward slashes
                            Some(name) => { 
                                cwd.push(name.to_string() + "/");// given a directory to go deeper
                                // collect cwd and push to list of all dirs
                            },
                            None => todo!(), // cd\n is bad input
                        }
                    }
                    Some(_) => todo!(), // unknown command
                    None => todo!(), // $ \n is bad input
                }
            }
            None => {continue;} // continue on empty line
            Some(num) => {
                // must be a number
                let size_result = num.parse::<u64>();
                let size = match size_result {
                    Ok(size) => {size},
                    Err(_) => todo!(), // should never get here
                };
                match line.split_whitespace().nth(1) {
                    Some(name) => {
                        let mut location = cwd.clone();
                        location.push(name.to_string());
                        tree.insert(location, size); // append name with size
                    },
                    None => todo!(), // number followed by nothing is bad input
                }
            }
        }
    }
    tree
}

fn parse_size(input: BTreeMap<Vec<String>, u64>) -> BTreeMap<Vec<String>, u64> {
    let mut size_tree: BTreeMap<Vec<String>, u64> = BTreeMap::new();
    for key in input.keys() {
        if '/' == key.last().unwrap().chars().last().unwrap(){
            continue;
        } else { // file
            let mut new_key = key.clone();
            for _ in 0..key.len()-1 { // update this and all outer dirs
                new_key.pop();
                if size_tree.contains_key(&new_key) {
                    let initial = size_tree[&new_key];
                    size_tree.insert(new_key.clone(), input[key] + initial);
                } else {
                    size_tree.insert(new_key.clone(), input[key]);
                }
            }
        }
    }
    size_tree
}

pub fn process_part1(input: &str) -> String {
    let file_tree = parse_tree(input);
    let size_tree = parse_size(file_tree);
    let result = size_tree
        .values()
        .fold(0, |sum, value| {
            if (*value) <= 100000 {
                sum + (*value)
            }
            else { sum }
        });
    result.to_string()
}

pub fn process_part2(input: &str) -> String {
    let file_tree = parse_tree(input);
    let size_tree = parse_size(file_tree);
    let root = vec!["/".to_string()];
    let needed_space = 30000000 - (70000000 - size_tree[&root]);
    let result = size_tree
        .values()
        .fold(size_tree[&root], |acc, value| {
            if *value >= needed_space && *value < acc {
                *value
            }
            else { acc }
        });
    result.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_input() {
        let file = fs::read_to_string("./test-input-1.txt").unwrap();
        assert_eq!(process_part1(file.as_str()), "95437");
        assert_eq!(process_part2(file.as_str()), "24933642");
    }
}
