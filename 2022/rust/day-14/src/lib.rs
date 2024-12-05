use std::{collections::HashMap, i32};

use nom::{
    bytes::complete::tag,
    character::complete::{self},
    multi::separated_list1,
    sequence::separated_pair,
    IResult,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Coord {
    x: i32,
    y: i32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Bounds {
    upper_left: Coord,
    lower_right: Coord,
}

#[derive(Debug, Clone, Copy)]
enum Slot {
    Rock,
    Air,
    MobileSand,
    RestingSand,
    Source,
}

type DrawRule = Vec<Coord>;

fn parse_draw_rule(input: &str) -> IResult<&str, DrawRule> {
    let (input, vec) = separated_list1(
        tag(" -> "),
        separated_pair(complete::i32, tag(","), complete::i32),
    )(input)?;
    Ok((
        input,
        vec.iter().map(|(x, y)| Coord { x: *x, y: *y }).collect(),
    ))
}

fn parse(input: &str) -> IResult<&str, Vec<DrawRule>> {
    separated_list1(tag("\n"), parse_draw_rule)(input)
}

fn draw_map(edges: Bounds, map: &HashMap<Coord, Slot>) {
    println!(
        "viewport top left corner is at {},{}",
        edges.upper_left.x - 1,
        edges.upper_left.y
    );
    for y in edges.upper_left.y..(edges.lower_right.y + 1) {
        for x in (edges.upper_left.x - 1)..=(edges.lower_right.x + 1) {
            match map.get(&Coord { x, y }) {
                Some(slot) => match slot {
                    Slot::Rock => print!("#"),
                    Slot::Air => print!("."),
                    Slot::RestingSand => print!("o"),
                    Slot::MobileSand => print!("*"),
                    Slot::Source => print!("+"),
                },
                None => print!("."),
            }
        }
        print!("\n")
    }
}

fn draw_rocks(rules: Vec<DrawRule>, source: Coord) -> (Bounds, HashMap<Coord, Slot>) {
    let mut map: HashMap<Coord, Slot> = HashMap::new();
    let mut leftmost: i32 = i32::MAX;
    let mut rightmost: i32 = 0;
    let mut uppermost: i32 = i32::MAX;
    let mut deepest: i32 = 0;
    for rule in rules {
        let mut last_coord_opt: Option<Coord> = None;
        for coord in rule {
            //update stats
            if coord.x > rightmost {
                rightmost = coord.x
            }
            if coord.x < leftmost {
                leftmost = coord.x
            }
            if coord.y > deepest {
                deepest = coord.y
            }
            if coord.y < uppermost {
                uppermost = coord.y
            }
            // draw rocks
            match last_coord_opt {
                None => {
                    last_coord_opt = Some(coord);
                }
                Some(last_coord) => {
                    if coord.x == last_coord.x {
                        //vertical line
                        if coord.y < last_coord.y {
                            for y in (coord.y)..=(last_coord.y) {
                                map.insert(Coord { x: coord.x, y: y }, Slot::Rock);
                            }
                        } else if last_coord.y < coord.y {
                            for y in (last_coord.y)..=(coord.y) {
                                map.insert(Coord { x: coord.x, y: y }, Slot::Rock);
                            }
                        } else {
                            panic!("should never get here, vertical line with same Ys")
                        }
                    } else if coord.y == last_coord.y {
                        // horizontal line
                        if coord.x < last_coord.x {
                            for x in (coord.x)..=(last_coord.x) {
                                map.insert(Coord { x: x, y: coord.y }, Slot::Rock);
                            }
                        } else if last_coord.x < coord.x {
                            for x in (last_coord.x)..=(coord.x) {
                                map.insert(Coord { x: x, y: coord.y }, Slot::Rock);
                            }
                        } else {
                            panic!("should never get here, horizontal line with same Xs")
                        }
                    } else {
                        panic!("should never get here, invalid rock draw rule")
                    }
                    last_coord_opt = Some(coord);
                }
            }
        }
    }
    map.insert(
        Coord {
            x: source.x,
            y: source.y,
        },
        Slot::Source,
    );
    if source.x > rightmost {
        rightmost = source.x;
    }
    if source.x < leftmost {
        leftmost = source.x;
    }
    if source.y > deepest {
        deepest = source.y
    }
    if source.y < uppermost {
        uppermost = source.y
    }
    let edges: Bounds = Bounds {
        upper_left: Coord {
            x: leftmost,
            y: uppermost,
        },
        lower_right: Coord {
            x: rightmost,
            y: deepest,
        },
    };
    //draw_map(edges, &map);
    (edges, map)
}

fn iterate_sand_once(
    location: Coord,
    map: &mut HashMap<Coord, Slot>,
    //edges: Bounds,
) -> (bool, Coord) {
    match map.get(&location) {
        Some(Slot::MobileSand) => {}
        _ => {
            panic!(
                "did not find mobile sand where the sand should be at {},{}",
                location.x, location.y
            )
        }
    }
    // iterate down once from current location
    let down_peek = Coord {
        x: location.x,
        y: location.y + 1,
    };
    let down_left_peek = Coord {
        x: location.x - 1,
        y: location.y + 1,
    };
    let down_right_peek = Coord {
        x: location.x + 1,
        y: location.y + 1,
    };
    match map.get(&down_peek) {
        Some(Slot::Rock) => {}
        Some(Slot::Air) => {
            //valid new location for sand
            map.insert(location, Slot::Air);
            map.insert(down_peek, Slot::MobileSand);
            //sand is still mobile so false is the return type
            return (false, down_peek);
        }
        Some(Slot::MobileSand) => {
            panic!("down: should not find moving sand while iterating another sand")
        }
        Some(Slot::RestingSand) => {}
        Some(Slot::Source) => {
            panic!("should not be the source, it shouldn't even be in the map")
        }
        None => {
            //tile is air;
            //valid new location for sand
            map.insert(location, Slot::Air);
            map.insert(down_peek, Slot::MobileSand);
            //sand is still mobile so false is the return type
            return (false, down_peek);
        }
    }
    match map.get(&down_left_peek) {
        Some(Slot::Rock) => {}
        Some(Slot::Air) => {
            //valid new location for sand
            map.insert(location, Slot::Air);
            map.insert(down_left_peek, Slot::MobileSand);
            //sand is still mobile so false is the return type
            return (false, down_left_peek);
        }
        Some(Slot::MobileSand) => {
            panic!("down_left: should not find moving sand while iterating another sand")
        }
        Some(Slot::RestingSand) => {}
        Some(Slot::Source) => {
            panic!("should not be the source, it shouldn't even be in the map")
        }
        None => {
            //tile is air;
            //valid new location for sand
            map.insert(location, Slot::Air);
            map.insert(down_left_peek, Slot::MobileSand);
            //sand is still mobile so false is the return type
            return (false, down_left_peek);
        }
    }
    match map.get(&down_right_peek) {
        Some(Slot::Rock) => {
            //last chance so the sand can't move anymore
            map.insert(location, Slot::RestingSand);
            return (true, down_right_peek);
        }
        Some(Slot::Air) => {
            //valid new location for sand
            map.insert(location, Slot::Air);
            map.insert(down_right_peek, Slot::MobileSand);
            //sand is still mobile so false is the return type
            return (false, down_right_peek);
        }
        Some(Slot::MobileSand) => {
            panic!("down_right: should not find moving sand while iterating another sand")
        }
        Some(Slot::RestingSand) => {
            //last chance so the sand can't move anymore
            map.insert(location, Slot::RestingSand);
            return (true, down_right_peek);
        }
        Some(Slot::Source) => {
            panic!("should not be the source, it shouldn't even be in the map")
        }
        None => {
            //tile is air;
            //valid new location for sand
            map.insert(location, Slot::Air);
            map.insert(down_right_peek, Slot::MobileSand);
            //sand is still mobile so false is the return type
            return (false, down_right_peek);
        }
    }
}

fn iterate_sand_until_rest(map: &mut HashMap<Coord, Slot>, edges: Bounds, source: Coord) -> bool {
    let max_height = (edges.lower_right.y - edges.upper_left.y) + 1;
    // create new mobile sand
    match map.get(&source) {
        Some(Slot::Source) => {
            map.insert(source, Slot::MobileSand);
        }
        _ => {
            panic!(
                "the source is not in the correct location, perhaps it wasn't replaced\
             after the sand came to rest or there was an incorrect parse"
            );
        }
    }
    let mut last_location = source;
    for _i in 0..=max_height {
        let sand_is_at_rest;
        (sand_is_at_rest, last_location) = iterate_sand_once(last_location, map); //edges);
        if sand_is_at_rest {
            // replace source
            map.insert(source, Slot::Source);
            return true;
        }
    }
    // replace source
    map.insert(source, Slot::Source);
    return false;
}

pub fn process_part1(input: &str) -> String {
    let source = Coord { x: 500, y: 0 };
    let (_, rules) = parse(input).expect("parse should succeed");
    let (edges, mut map) = draw_rocks(rules, source);
    let mut sand_count = 0;
    let sand_max = ((edges.lower_right.x - edges.upper_left.x) + 1)
        * ((edges.lower_right.y - edges.upper_left.y) + 1);
    for _i in 0..sand_max {
        if iterate_sand_until_rest(&mut map, edges, source) {
            sand_count += 1;
        } else {
            break;
        }
    }
    //draw_map(edges, &map);
    sand_count.to_string()
}

fn iterate_sand_once_with_floor(
    location: Coord,
    map: &mut HashMap<Coord, Slot>,
    floor_level: i32,
) -> (bool, Coord) {
    match map.get(&location) {
        Some(Slot::MobileSand) => {}
        _ => {
            panic!(
                "did not find mobile sand where the sand should be at {},{}",
                location.x, location.y
            )
        }
    }
    // iterate down once from current location
    let down_peek = Coord {
        x: location.x,
        y: location.y + 1,
    };
    let down_left_peek = Coord {
        x: location.x - 1,
        y: location.y + 1,
    };
    let down_right_peek = Coord {
        x: location.x + 1,
        y: location.y + 1,
    };
    match map.get(&down_peek) {
        Some(Slot::Rock) => {}
        Some(Slot::Air) => {
            //valid new location for sand
            map.insert(location, Slot::Air);
            map.insert(down_peek, Slot::MobileSand);
            //sand is still mobile so false is the return type
            return (false, down_peek);
        }
        Some(Slot::MobileSand) => {
            panic!("down: should not find moving sand while iterating another sand")
        }
        Some(Slot::RestingSand) => {}
        Some(Slot::Source) => {
            panic!("should not be the source, it shouldn't even be in the map")
        }
        None => {
            // first check if we are at floor level
            if down_peek.y == floor_level {
                //if so, do nothing
            } else {
                //tile is air;
                //valid new location for sand
                map.insert(location, Slot::Air);
                map.insert(down_peek, Slot::MobileSand);
                //sand is still mobile so false is the return type
                return (false, down_peek);
            }
        }
    }
    match map.get(&down_left_peek) {
        Some(Slot::Rock) => {}
        Some(Slot::Air) => {
            //valid new location for sand
            map.insert(location, Slot::Air);
            map.insert(down_left_peek, Slot::MobileSand);
            //sand is still mobile so false is the return type
            return (false, down_left_peek);
        }
        Some(Slot::MobileSand) => {
            panic!("down_left: should not find moving sand while iterating another sand")
        }
        Some(Slot::RestingSand) => {}
        Some(Slot::Source) => {
            panic!("should not be the source, it shouldn't even be in the map")
        }
        None => {
            // first check if we are at floor level
            if down_peek.y == floor_level {
                //if so, do nothing
            } else {
                //tile is air;
                //valid new location for sand
                map.insert(location, Slot::Air);
                map.insert(down_left_peek, Slot::MobileSand);
                //sand is still mobile so false is the return type
                return (false, down_left_peek);
            }
        }
    }
    match map.get(&down_right_peek) {
        Some(Slot::Rock) => {
            //last chance so the sand can't move anymore
            map.insert(location, Slot::RestingSand);
            return (true, down_right_peek);
        }
        Some(Slot::Air) => {
            //valid new location for sand
            map.insert(location, Slot::Air);
            map.insert(down_right_peek, Slot::MobileSand);
            //sand is still mobile so false is the return type
            return (false, down_right_peek);
        }
        Some(Slot::MobileSand) => {
            panic!("down_right: should not find moving sand while iterating another sand")
        }
        Some(Slot::RestingSand) => {
            //last chance so the sand can't move anymore
            map.insert(location, Slot::RestingSand);
            return (true, down_right_peek);
        }
        Some(Slot::Source) => {
            panic!("should not be the source, it shouldn't even be in the map")
        }
        None => {
            // first check if we are at floor level
            if down_peek.y == floor_level {
                //last chance so the sand can't move anymore
                map.insert(location, Slot::RestingSand);
                return (true, down_right_peek);
            } else {
                //tile is air;
                //valid new location for sand
                map.insert(location, Slot::Air);
                map.insert(down_right_peek, Slot::MobileSand);
                //sand is still mobile so false is the return type
                return (false, down_right_peek);
            }
        }
    }
}

fn iterate_sand_until_rest_with_floor(
    map: &mut HashMap<Coord, Slot>,
    edges: Bounds,
    source: Coord,
) -> bool {
    let max_height = (edges.lower_right.y - edges.upper_left.y) + 1;
    // create new mobile sand
    match map.get(&source) {
        Some(Slot::Source) => {
            map.insert(source, Slot::MobileSand);
        }
        _ => {
            panic!(
                "the source is not in the correct location, perhaps it wasn't replaced\
             after the sand came to rest or there was an incorrect parse"
            );
        }
    }
    let mut last_location = source;
    for _i in 0..=(max_height + 2) {
        let sand_is_at_rest;
        (sand_is_at_rest, last_location) =
            iterate_sand_once_with_floor(last_location, map, edges.lower_right.y + 2);
        if sand_is_at_rest {
            // replace source
            match map.insert(source, Slot::Source) {
                Some(Slot::Air) => return true,
                Some(Slot::RestingSand) => return false,
                Some(Slot::MobileSand) => panic!("should still not have mobile sand"),
                Some(Slot::Rock) => panic!("there should not be a rock"),
                Some(Slot::Source) => panic!("there should not already be a source"),
                None => panic!("there should be SOMETHING here"),
            }
        }
    }
    panic!("iter_sand_rest_w_floor: should not get here");
}

pub fn process_part2(input: &str) -> String {
    let source = Coord { x: 500, y: 0 };
    let (_, rules) = parse(input).expect("parse should succeed");
    let (edges, mut map) = draw_rocks(rules, source);
    let mut sand_count = 0;
    while iterate_sand_until_rest_with_floor(&mut map, edges, source) {
        sand_count += 1;
    }
    //// stuff only needed to draw the map
    //let mut leftmost: i32 = i32::MAX;
    //let mut rightmost: i32 = 0;
    //let mut uppermost: i32 = i32::MAX;
    //let mut deepest: i32 = 0;
    //for key in map.keys() {
    //    if key.x > rightmost {
    //        rightmost = key.x
    //    }
    //    if key.x < leftmost {
    //        leftmost = key.x
    //    }
    //    if key.y > deepest {
    //        deepest = key.y
    //    }
    //    if key.y < uppermost {
    //        uppermost = key.y
    //    }
    //}
    //let new_edges: Bounds = Bounds {
    //    upper_left: Coord {
    //        x: leftmost,
    //        y: uppermost,
    //    },
    //    lower_right: Coord {
    //        x: rightmost,
    //        y: deepest,
    //    },
    //};
    //draw_map(new_edges, &map);
    //// end of map draw code
    (sand_count + 1).to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input() {
        let file = include_str!("../test-input-1.txt");
        assert_eq!(process_part1(file), "24");
    }
    #[test]
    fn test_input2() {
        let file = include_str!("../test-input-1.txt");
        assert_eq!(process_part2(file), "93");
    }
}
