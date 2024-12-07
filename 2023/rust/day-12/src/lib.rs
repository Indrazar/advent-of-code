use std::fmt::Display;

use itertools::{repeat_n, Itertools};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Tile {
    Unknown,
    Operational,
    Damaged,
}

impl Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let character = match self {
            Tile::Unknown => '?',
            Tile::Operational => '.',
            Tile::Damaged => '#',
        };
        write!(f, "{character}")
    }
}

impl TryFrom<char> for Tile {
    type Error = ();

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '?' => Ok(Tile::Unknown),
            '.' => Ok(Tile::Operational),
            '#' => Ok(Tile::Damaged),
            _ => Err(()),
        }
    }
}

fn possible_options(
    group: &Vec<Tile>,
    constraints: &Vec<u64>,
    unknowns: u64,
) -> impl Iterator<Item = Vec<Tile>> {
    repeat_n(
        [Tile::Operational, Tile::Damaged].into_iter(),
        unknowns as usize,
    )
    .multi_cartesian_product()
}

fn check_option(group: &Vec<Tile>, option: &Vec<Tile>, constraints: &Vec<u64>) -> bool {
    let mut option_iter = option.iter();
    let filled_group: Vec<Tile> = group
        .iter()
        .map(|tile| match tile {
            Tile::Unknown => option_iter
                .next()
                .expect("should have enough options for unknowns")
                .clone(),
            Tile::Operational => Tile::Operational,
            Tile::Damaged => Tile::Damaged,
        })
        .collect();
    let counts = filled_group
        .iter()
        .chunk_by(|t| **t == Tile::Damaged)
        .into_iter()
        .filter_map(|(is_hashes, group)| is_hashes.then_some(group.into_iter().count() as u64))
        .collect::<Vec<u64>>();
    constraints[..] == counts[..]
}

fn process_line(input: &str) -> u64 {
    let group: Vec<Tile> = input
        .split_ascii_whitespace()
        .nth(0)
        .expect("must have layout")
        .chars()
        .map(|symbol| symbol.try_into().expect("must be valid symbol"))
        .collect::<Vec<Tile>>();
    let constraints = input
        .split_ascii_whitespace()
        .nth(1)
        .expect("must have groups")
        .split(',')
        .map(|num| num.parse::<u64>().expect("must be a number"))
        .collect::<Vec<u64>>();
    // let mut unknowns = 0;
    // for x in group.iter() {
    //     match x {
    //         Tile::Unknown => unknowns += 1,
    //         _ => {}
    //     }
    // }
    // let options = possible_options(&group, &constraints, unknowns);
    // options
    //     .filter(|option| check_option(&group, option, &constraints))
    //     .count() as u64
    determine_variations(&group, &constraints)
}

fn determine_variations(tiles: &Vec<Tile>, constraints: &Vec<u64>) -> u64 {
    let mut constraints = constraints.clone();
    constraints.push(0);
    let max_run: usize = *constraints.iter().max().expect("there should be a max") as usize;
    let mut tiles = tiles.clone();
    tiles.push(Tile::Operational);

    let tile_count = tiles.len(); //n
    let constraint_count = constraints.len(); //m
    let mut dp: Vec<Vec<Vec<Option<u64>>>> =
        vec![vec![vec![None; max_run + 1usize]; constraint_count]; tile_count];
    for current_tile in 0..tile_count {
        let x = tiles[current_tile];
        let mut combinations_if_current_operational: Option<u64> = None;
        let mut combinations_if_current_is_damaged: Option<u64> = None;
        for current_run in 0..constraint_count {
            for current_damaged_count in 0..((constraints[current_run] + 1) as usize) {
                if current_tile == 0 {
                    if current_run != 0 {
                        dp[current_tile][current_run][current_damaged_count] = Some(0);
                        continue;
                    } else {
                        match x {
                            Tile::Damaged => {
                                if current_damaged_count != 1 {
                                    dp[current_tile][current_run][current_damaged_count] = Some(0);
                                    continue;
                                } else {
                                    dp[current_tile][current_run][current_damaged_count] = Some(1);
                                    continue;
                                }
                            }
                            Tile::Operational => {
                                if current_damaged_count != 0 {
                                    dp[current_tile][current_run][current_damaged_count] = Some(0);
                                    continue;
                                } else {
                                    dp[current_tile][current_run][current_damaged_count] = Some(1);
                                    continue;
                                }
                            }
                            Tile::Unknown => {
                                if current_damaged_count != 1 && current_damaged_count != 0 {
                                    dp[current_tile][current_run][current_damaged_count] = Some(0);
                                    continue;
                                } else {
                                    dp[current_tile][current_run][current_damaged_count] = Some(1);
                                    continue;
                                }
                            }
                        }
                    }
                }
                if current_damaged_count != 0 {
                    combinations_if_current_operational = Some(0);
                } else if current_run > 0 {
                    if current_damaged_count != 0 {
                        panic!("k must be 0")
                    } else {
                        combinations_if_current_operational = Some(
                            dp[current_tile - 1][current_run - 1]
                                [constraints[current_run - 1] as usize]
                                .expect("should have a value"),
                        );
                        combinations_if_current_operational = Some(
                            combinations_if_current_operational.expect("should have a value")
                                + dp[current_tile - 1][current_run][0]
                                    .expect("should have a value"),
                        );
                    }
                } else {
                    // current tile > 0, run=0, damaged=0
                    // only possibility is every unknown is operational
                    combinations_if_current_operational = if *(&tiles[0..current_tile]
                        .iter()
                        .map(|val| if *val == Tile::Damaged { 1 } else { 0 })
                        .sum::<u64>())
                        == 0
                    {
                        Some(1)
                    } else {
                        Some(0)
                    };
                }
                if current_damaged_count == 0 {
                    combinations_if_current_is_damaged = Some(0);
                } else {
                    combinations_if_current_is_damaged =
                        dp[current_tile - 1][current_run][current_damaged_count - 1];
                }

                match x {
                    Tile::Operational => {
                        dp[current_tile][current_run][current_damaged_count] =
                            Some(combinations_if_current_operational.expect("should have a value"));
                    }
                    Tile::Damaged => {
                        dp[current_tile][current_run][current_damaged_count] =
                            Some(combinations_if_current_is_damaged.expect("should have a value"));
                    }
                    Tile::Unknown => {
                        dp[current_tile][current_run][current_damaged_count] = Some(
                            combinations_if_current_operational.expect("should have a value")
                                + combinations_if_current_is_damaged.expect("should have a value"),
                        );
                    }
                }
            }
        }
    }
    // dp[last tile][last run][no damaged]
    // we added an extra no damaged at the end so it's the total combinations
    let ans = dp[tile_count - 1][dp[tile_count - 1].len() - 1][0];
    return ans.expect("this should have a value");
}

fn process_line2(input: &str) -> u64 {
    let small_group: Vec<Tile> = input
        .split_ascii_whitespace()
        .nth(0)
        .expect("must have layout")
        .chars()
        .map(|symbol| symbol.try_into().expect("must be valid symbol"))
        .collect::<Vec<Tile>>();
    let mut group: Vec<Tile> = Vec::default();
    // not 5 because this would add an extra incorrect unknown on the end
    for _ in 0..4 {
        group.append(&mut small_group.clone());
        group.push(Tile::Unknown)
    }
    // append the 5th with no extra unknown
    group.append(&mut small_group.clone());
    let small_constraints = input
        .split_ascii_whitespace()
        .nth(1)
        .expect("must have groups")
        .split(',')
        .map(|num| num.parse::<u64>().expect("must be a number"))
        .collect::<Vec<u64>>();
    let mut constraints: Vec<u64> = Vec::default();
    for _ in 0..5 {
        constraints.append(&mut small_constraints.clone())
    }
    let mut group_string = String::new();
    for tile in &group {
        match tile {
            Tile::Unknown => group_string.push('?'),
            Tile::Operational => group_string.push('.'),
            Tile::Damaged => group_string.push('#'),
        }
    }
    let mut constraint_iter = constraints.iter();
    let mut constraints_string = String::from(format!(
        "{}",
        constraint_iter.next().expect("there should be a value")
    ));
    for val in constraint_iter {
        constraints_string = format!("{constraints_string}, {val}");
    }
    let res = determine_variations(&group, &constraints);
    res
}

pub fn process_part1(input: &str) -> String {
    input
        .lines()
        .map(|line| process_line(line))
        .sum::<u64>()
        .to_string()
}

pub fn process_part2(input: &str) -> String {
    input
        .lines()
        .map(|line| process_line2(line))
        .sum::<u64>()
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input() {
        let file = include_str!("../test-input-1.txt");
        assert_eq!(process_part1(file), "21");
        assert_eq!(process_part2(file), "525152");
    }
    #[test]
    fn test_1() {
        assert_eq!(process_part1("???.### 1,1,3"), "1")
    }
    #[test]
    fn test_2() {
        assert_eq!(process_part1(".??..??...?##. 1,1,3"), "4")
    }
    #[test]
    fn test_3() {
        assert_eq!(process_part1("?#?#?#?#?#?#?#? 1,3,1,6"), "1")
    }
    #[test]
    fn test_4() {
        assert_eq!(process_part1("????.#...#... 4,1,1"), "1")
    }
    #[test]
    fn test_5() {
        assert_eq!(process_part1("????.######..#####. 1,6,5"), "4")
    }
    #[test]
    fn test_6() {
        assert_eq!(process_part1("?###???????? 3,2,1"), "10")
    }
    #[test]
    fn test_7() {
        assert_eq!(process_part2("???.### 1,1,3"), "1")
    }
    #[test]
    fn test_8() {
        assert_eq!(process_part2(".??..??...?##. 1,1,3"), "16384")
    }
    #[test]
    fn test_9() {
        assert_eq!(process_part2("?#?#?#?#?#?#?#? 1,3,1,6"), "1")
    }
    #[test]
    fn test_10() {
        assert_eq!(process_part2("????.#...#... 4,1,1"), "16")
    }
    #[test]
    fn test_11() {
        assert_eq!(process_part2("????.######..#####. 1,6,5"), "2500")
    }
    #[test]
    fn test_12() {
        assert_eq!(process_part2("?###???????? 3,2,1"), "506250")
    }
}
