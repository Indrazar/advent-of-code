#[derive(Debug, Clone)]
struct Group {
    source: String,
    destination: String,
    remaps: Vec<RemapRange>,
}

#[derive(Debug, Clone, Copy)]
struct RemapRange {
    source_start: usize,
    destination_start: usize,
    range: usize,
}

#[derive(Debug, Clone, Copy)]
struct SeedRange {
    start: usize,
    range: usize,
}

fn parse_group(group: &str) -> Group {
    let source = group
        .lines()
        .next()
        .expect("group &str ended early")
        .split('-')
        .nth(0)
        .expect("should be a source")
        .to_string();
    let destination = group
        .lines()
        .next()
        .expect("group &str ended early")
        .split(&['-', ' '][..])
        .nth(2)
        .expect("should be a destination")
        .to_string();
    let mut group_iter = group.lines();
    group_iter.next(); //discard first line
    let remaps = group_iter
        .map(|remap| {
            let mut map_iter = remap.split_ascii_whitespace();
            let destination_start = map_iter
                .next()
                .expect("dest start should be present")
                .parse::<usize>()
                .expect("dest should be a number");
            let source_start = map_iter
                .next()
                .expect("source start should be present")
                .parse::<usize>()
                .expect("source should be a number");
            let range = map_iter
                .next()
                .expect("range should be present")
                .parse::<usize>()
                .expect("range should be a number");
            RemapRange {
                source_start,
                destination_start,
                range,
            }
        })
        .collect::<Vec<RemapRange>>();
    Group {
        source,
        destination,
        remaps,
    }
}

fn follow_remaps(seed: &usize, remap_groups: &Vec<Group>) -> usize {
    let mut current_id = seed.clone();
    let mut current_type: String = String::from("seed");
    for group in remap_groups {
        if group.source != current_type {
            panic!(
                "remap misaigned by source, expected {} but started on {current_type}",
                group.source
            )
        } else {
            for remap in group.remaps.clone() {
                if current_id == remap.source_start
                    || (current_id > remap.source_start
                        && current_id < remap.source_start + remap.range)
                {
                    let offset = current_id - remap.source_start;
                    //println!(
                    //    "updating {current_type} from {current_id} to {} with id {}",
                    //    group.destination,
                    //    remap.destination_start + offset
                    //);
                    current_id = remap.destination_start + offset;
                    break;
                } else {
                    continue; // do not change current_id, and check next remap range
                }
            }
            current_type = group.destination.clone();
        }
    }
    current_id
}

fn follow_remaps_input_ranges(seed: SeedRange, remap_groups: &Vec<Group>) -> usize {
    let mut min = usize::MAX;
    for i in seed.start..seed.start + seed.range - 1 {
        // this approach will take tens of minutes with inputs that are on the order of billions of seeds
        let val = follow_remaps(&i, remap_groups);
        if val < min {
            min = val;
        }
    }
    min
}

pub fn process_part1(input: &str) -> String {
    let mut group_iter = input.split("\n\n");
    let starting_seeds = group_iter
        .next()
        .expect("input is malformed")
        .split(":")
        .nth(1)
        .expect("input should contain values after :")
        .split_ascii_whitespace()
        .filter_map(|num| num.parse::<usize>().ok())
        .collect::<Vec<usize>>();
    let remap_groups = group_iter
        .map(|group| parse_group(group))
        .collect::<Vec<Group>>();
    let ending_locations = starting_seeds
        .iter()
        .map(|seed| follow_remaps(&seed, &remap_groups))
        .collect::<Vec<usize>>();
    ending_locations
        .iter()
        .min()
        .expect("should be a min")
        .to_string()
}

pub fn process_part2(input: &str) -> String {
    let mut group_iter = input.split("\n\n");
    let mut starting_seeds_iter = group_iter
        .next()
        .expect("input is malformed")
        .split(":")
        .nth(1)
        .expect("input should contain values after :")
        .split_ascii_whitespace();
    let mut seed_ranges: Vec<SeedRange> = Vec::default();
    loop {
        let start = match starting_seeds_iter.next() {
            Some(num) => num.parse::<usize>().expect("should be a number"),
            None => break,
        };
        let range = starting_seeds_iter
            .next()
            .expect("seed values must come in pairs")
            .parse::<usize>()
            .expect("should be a number");
        seed_ranges.push(SeedRange { start, range })
    }
    let remap_groups = group_iter
        .map(|group| parse_group(group))
        .collect::<Vec<Group>>();
    let ending_locations = seed_ranges
        .iter()
        .map(|seed| follow_remaps_input_ranges(seed.clone(), &remap_groups))
        .collect::<Vec<usize>>();
    ending_locations
        .iter()
        .min()
        .expect("should be a min")
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_input() {
        let file = fs::read_to_string("./test-input-1.txt").unwrap();
        assert_eq!(process_part1(file.as_str()), "35");
        assert_eq!(process_part2(file.as_str()), "46");
    }
}
