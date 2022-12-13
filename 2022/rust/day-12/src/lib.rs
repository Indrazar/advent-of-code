
#[derive(Debug, PartialEq, Eq)]
enum Plot {
    Start,
    End,
    Normal(u32),  
}

#[derive(Debug, PartialEq, Eq)]
enum Distance {
    Unvisited,
    Visited(u32),    
}

#[derive(Debug)]
enum Updated {
    No,
    Yes
}

fn parse_input(input: &str) -> Vec<Vec<(Plot, Distance, Updated)>> {
    input
        .lines()
        .map(|line|{
            line
            .chars()
            .map(|letter| {
                if letter == 'S' {
                    (Plot::Start, Distance::Visited(0), Updated::Yes)
                } else if letter == 'E' {
                    (Plot::End, Distance::Unvisited, Updated::No)
                } else if (letter as u32) >= ('a' as u32) || (letter as u32) <= ('z' as u32) {
                    (Plot::Normal(letter as u32 - 'a' as u32 + 1), Distance::Unvisited, Updated::No)
                } else {
                    panic!("invalid input map");
                }})
            .collect::<Vec<(Plot, Distance, Updated)>>()
        })
        .collect::<Vec<Vec<(Plot, Distance, Updated)>>>()
}

fn parse_input2(input: &str) -> Vec<Vec<(Plot, Distance, Updated)>> {
    input
        .lines()
        .map(|line|{
            line
            .chars()
            .map(|letter| {
                if letter == 'S' || letter == 'a' {
                    (Plot::Start, Distance::Visited(0), Updated::Yes)
                } else if letter == 'E' {
                    (Plot::End, Distance::Unvisited, Updated::No)
                } else if (letter as u32) >= ('a' as u32) || (letter as u32) <= ('z' as u32) {
                    (Plot::Normal(letter as u32 - 'a' as u32 + 1), Distance::Unvisited, Updated::No)
                } else {
                    panic!("invalid input map");
                }})
            .collect::<Vec<(Plot, Distance, Updated)>>()
        })
        .collect::<Vec<Vec<(Plot, Distance, Updated)>>>()
}

fn grab_end_coords(input: &Vec<Vec<(Plot, Distance, Updated)>>) -> Option<(usize, usize)> {
    let mut end_y: usize = 0;
    let mut end_x: usize = 0;
    let mut found = false;
    for y in 0..input.len() {
        for x in 0..input[0].len() {
            if input[y][x].0 == Plot::End {
                found = true;
                end_y = y;
                end_x = x;
            }
        }
    }
    if found {
        Some((end_y, end_x))
    } else {
        None
    }
}

fn get_height(input: &Plot) -> u32 {
    match input {
        Plot::Start => 1,
        Plot::End => 26,
        Plot::Normal(x) => *x,
    }
}

fn update_neighbors(input: &mut Vec<Vec<(Plot, Distance, Updated)>>, y: usize, x: usize) {
    let new_dist = match input[y][x].1 {
        Distance::Unvisited => panic!("invalid lookup at y{}, x{}", y, x),
        Distance::Visited(dist) => dist+1,
    };
    let height = get_height(&input[y][x].0);
    //up
    if y != 0 && get_height(&input[y-1][x].0)-1 <= height {
        match input[y-1][x].1 {
            Distance::Unvisited => {
                input[y-1][x].2 = Updated::Yes;
                input[y-1][x].1 = Distance::Visited(new_dist);
            },
            Distance::Visited(dist) => {
                if dist > new_dist {
                    input[y-1][x].2 = Updated::Yes;
                    input[y-1][x].1 = Distance::Visited(new_dist);                    
                }
            },
        }
    }
    //down
    if y+1 < input.len() && get_height(&input[y+1][x].0)-1 <= height {
        match input[y+1][x].1 {
            Distance::Unvisited => {
                input[y+1][x].2 = Updated::Yes;
                input[y+1][x].1 = Distance::Visited(new_dist);
            },
            Distance::Visited(dist) => {
                if dist > new_dist {
                    input[y+1][x].2 = Updated::Yes;
                    input[y+1][x].1 = Distance::Visited(new_dist);
                }
            },
        }
    }
    //left
    if x != 0 && get_height(&input[y][x-1].0)-1 <= height {
        match input[y][x-1].1 {
            Distance::Unvisited => {                    
                input[y][x-1].2 = Updated::Yes;
                input[y][x-1].1 = Distance::Visited(new_dist);
            },
            Distance::Visited(dist) => {
                if dist > new_dist {
                    input[y][x-1].2 = Updated::Yes;
                    input[y][x-1].1 = Distance::Visited(new_dist);
                }
            },
        }
    }
    //right
    if x+1 < input[0].len() && get_height(&input[y][x+1].0)-1 <= height {
        match input[y][x+1].1 {
            Distance::Unvisited => {
                input[y][x+1].2 = Updated::Yes;
                input[y][x+1].1 = Distance::Visited(new_dist);
            },
            Distance::Visited(dist) => {
                if dist > new_dist {
                    input[y][x+1].2 = Updated::Yes;
                    input[y][x+1].1 = Distance::Visited(new_dist);
                }
            },
        }
    }
}

fn update_map(input: &mut Vec<Vec<(Plot, Distance, Updated)>>) -> bool {
    // find updated
    let mut updates: Vec<(usize, usize)> = Vec::new();
    for y in 0..input.len() {
        for x in 0..input[0].len() {
            match input[y][x].2 {
                Updated::No => continue,
                Updated::Yes => { 
                    updates.push((y,x));
                    input[y][x].2 = Updated::No
                },
            }
        }
    }
    if updates.len() == 0 {
        return false
    }
    for (y, x) in updates {
        update_neighbors(input, y, x)
    }
    true
}

pub fn process_part1(input: &str) -> String {
    let mut map = parse_input(input);
    let (end_y, end_x) = match grab_end_coords(&map) {
        Some((y, x)) => (y, x),
        None => panic!("invalid input"),
    };
    let mut new_nodes = true;
    while map[end_y][end_x].1 == Distance::Unvisited && new_nodes {
        new_nodes = update_map(&mut map);
    }
    match map[end_y][end_x].1 {
        Distance::Unvisited => {
            panic!("no valid route")
        },
        Distance::Visited(dist) => dist,
    }.to_string()
}

pub fn process_part2(input: &str) -> String {
    let mut map = parse_input2(input);
    let (end_y, end_x) = match grab_end_coords(&map) {
        Some((y, x)) => (y, x),
        None => panic!("invalid input"),
    };
    let mut new_nodes = true;
    while map[end_y][end_x].1 == Distance::Unvisited && new_nodes {
        new_nodes = update_map(&mut map);
    }
    match map[end_y][end_x].1 {
        Distance::Unvisited => {
            panic!("no valid route")
        },
        Distance::Visited(dist) => dist,
    }.to_string()
}


#[cfg(test)]
mod tests {
    use std::fs;
    use super::*;

    #[test]
    fn test_input() {
        let file = fs::read_to_string("./test-input-1.txt").unwrap();
        assert_eq!(process_part1(file.as_str()), "31");
        assert_eq!(process_part2(file.as_str()), "29")
    }
}
