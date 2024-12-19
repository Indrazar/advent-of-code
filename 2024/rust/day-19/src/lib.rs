use std::collections::HashMap;

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{alpha1, line_ending},
    multi::{many1, many_till},
    sequence::terminated,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Color {
    White,
    Blue,
    Black,
    Red,
    Green,
}

// impl Color {
//     fn iter() -> std::vec::IntoIter<Color> {
//         vec![
//             Color::White,
//             Color::Blue,
//             Color::Black,
//             Color::Red,
//             Color::Green,
//         ]
//         .into_iter()
//     }
// }

fn ch_to_color(input: char) -> Color {
    match input {
        'w' => Color::White,
        'u' => Color::Blue,
        'b' => Color::Black,
        'r' => Color::Red,
        'g' => Color::Green,
        unknown => panic!("unknown character in stripe list: {unknown}"),
    }
}

fn parse(input: &str) -> (Vec<Vec<Color>>, Vec<Vec<Color>>) {
    // get list of stripes
    let (input, (stripes, _)) = many_till(
        terminated(
            alpha1::<&str, nom::error::Error<_>>,
            alt((tag(", "), tag(""))),
        ),
        line_ending,
    )(input)
    .expect("should be a list of stripes");
    // eat the line ending between the lists
    let (input, _) = tag::<&str, &str, nom::error::Error<_>>("\n")(input)
        .expect("stripes and requests should be seporated by a newline");
    // get list of requests
    let (_, requests) = many1(terminated(
        alpha1::<&str, nom::error::Error<_>>,
        line_ending,
    ))(input)
    .expect("there should be requests");

    // process &str
    let mut vec_stripes: Vec<Vec<Color>> = Vec::with_capacity(stripes.len());
    for stripe in stripes {
        let mut vec_stripe: Vec<Color> = Vec::with_capacity(stripe.chars().count());
        for ch in stripe.chars() {
            vec_stripe.push(ch_to_color(ch));
        }
        if !vec_stripes.contains(&vec_stripe) {
            vec_stripes.push(vec_stripe);
        } else {
            panic!("found a duplicate stripe list")
        }
    }
    let mut vec_requests: Vec<Vec<Color>> = Vec::with_capacity(requests.len());
    for req in requests {
        let mut vec_req: Vec<Color> = Vec::with_capacity(req.chars().count());
        for ch in req.chars() {
            vec_req.push(ch_to_color(ch));
        }
        vec_requests.push(vec_req);
    }
    //println!("stripes: {vec_stripes:?}");
    //println!("requests: {vec_requests:?}");
    (vec_stripes, vec_requests)
}

fn create_map(stripes: &Vec<Vec<Color>>) -> HashMap<(Color, usize), Vec<Vec<Color>>> {
    let mut map: HashMap<(Color, usize), Vec<Vec<Color>>> = HashMap::new();
    for stripe_group in stripes {
        if stripe_group.is_empty() {
            panic!("striped objects need at least one stripe");
        }
        match map.get_mut(&(stripe_group[0], stripe_group.len())) {
            Some(listing) => {
                listing.push(stripe_group.clone());
            }
            None => {
                map.insert(
                    (stripe_group[0], stripe_group.len()),
                    vec![stripe_group.clone()],
                );
            }
        }
    }
    map
}

fn fit(req: &[Color], test: &[Color], req_indx: usize) -> bool {
    //println!("checking if {test:?} fits into {req:?} starting at {req_indx}");
    if req_indx + test.len() > req.len() {
        //println!("{test:?} is too long");
        return false;
    }
    //println!("{test:?} is not too long");
    for (test_idx, req_idx) in (req_indx..req_indx + test.len()).enumerate() {
        if test[test_idx] != req[req_idx] {
            //println!("does not match at {test_idx}");
            return false;
        } else {
            // continue
        }
    }
    //println!("{test:?} fits into {req:?} starting at {req_indx}");
    true
}

fn check_build(
    req: &Vec<Color>,
    map: &HashMap<(Color, usize), Vec<Vec<Color>>>,
    idx: usize,
    current: Vec<Vec<Color>>,
) -> (bool, Vec<Vec<Color>>) {
    if idx >= req.len() {
        //println!("match found!");
        // we're at the end and we matched everything
        return (true, current);
    }
    // biggest first mode
    //for i in (1..(req.len() - idx + 1)).rev() {
    // smallest first mode

    for i in 1..(req.len() - idx + 1) {
        //println!("checking for a len of {i} at {idx}");
        match map.get(&(req[idx], i)) {
            Some(listing) => {
                // we have a list of possible things that could fit
                for possible in listing {
                    if fit(req, possible, idx) {
                        let mut next: Vec<Vec<Color>> = current.clone();
                        next.push(possible.clone());
                        let (check, current) = check_build(req, map, idx + possible.len(), next);
                        if check {
                            return (true, current);
                        } else {
                            continue;
                        }
                    }
                }
            }
            None => {
                // nothing with this length at this color go shorter
                // println!(
                //     "no valid next starting {:?} at {idx} with len {i}",
                //     req[idx]
                // );
                continue;
            }
        }
    }
    (false, vec![])
}

// fn collision(blocked: &Vec<(usize, usize)>, new: (usize, usize)) -> bool {
//     for blocked_region in blocked {
//         for i in blocked_region.0..=blocked_region.1 {
//             if (new.0..=new.1).contains(&i) {
//                 return true;
//             }
//         }
//     }
//     false
// }

// fn fit_around(
//     req: &Vec<Color>,
//     test: &Vec<Color>,
//     req_indx: usize,
//     constraint: Color,
//     blocked: &Vec<(usize, usize)>,
// ) -> (bool, usize, usize) {
//     //println!("checking if {test:?} fits in {req:?} starting near {req_indx}");
//     //find where test could fit based on the constraining color
//     let mut rel_pos = usize::MAX;
//     for (i, color) in test.iter().enumerate() {
//         if *color == constraint {
//             rel_pos = i;
//         }
//     }
//     if rel_pos == usize::MAX {
//         panic!("invalid test value {test:?} in fit_around, does not contain {constraint:?}");
//     }
//     if req_indx < rel_pos {
//         println!("first {constraint:?} would put it outside bounds: {req_indx} < {rel_pos}");
//         return (false, 0, 0);
//     }
//     let start_pos = req_indx - rel_pos;
//     let end_pos = req_indx + test.len() - rel_pos - 1;
//     if end_pos >= req.len() {
//         println!("{test:?} is too long to fit");
//         return (false, 0, 0);
//     }
//     // check do we collide with any blocked regions
//     if collision(blocked, (start_pos, end_pos)) {
//         println!("collision with blocked region");
//         return (false, 0, 0);
//     }
//     // does it actually match the literal text?
//     for (test_idx, req_idx) in (start_pos..=end_pos).enumerate() {
//         if test[test_idx] != req[req_idx] {
//             println!("does not match at {test_idx}");
//             return (false, 0, 0);
//         } else {
//             // continue
//         }
//     }
//     //println!("{test:?} fits into {req:?} starting at {req_indx}");
//     (true, start_pos, end_pos)
// }

// fn check_build_with_constraint(
//     req: &Vec<Color>,
//     //start_map: &HashMap<(Color, usize), Vec<Vec<Color>>>,
//     idx: usize,
//     constraint: Color,
//     constraint_list: &Vec<Vec<Color>>,
//     mut blocked: Vec<(usize, usize)>,
// ) -> bool {
//     println!("checking {req:?} starting at {idx} with blocked: {blocked:?}");
//     if idx >= req.len() {
//         // we're at the end and we matched everything
//         println!("already done");
//         return true;
//     }
//     // check if there are any constraining colors even left
//     let mut no_constraint = true;
//     for i in idx..req.len() {
//         if req[i] == constraint {
//             no_constraint = false;
//             break;
//         }
//     }
//     if no_constraint {
//         println!("no constraints remaining");
//         return true;
//     }
//     // find first color we're constrained by
//     for i in idx..req.len() {
//         if req[i] == constraint {
//             for possible in constraint_list {
//                 print!("trying: {possible:?}: ");
//                 let (check, start, end) = fit_around(req, possible, i, constraint, &blocked);
//                 if check {
//                     // mark section as blocked
//                     println!("yes! new blocked section: ({start},{end})");
//                     blocked.push((start, end));
//                     // recursively call with updated idx
//                     return check_build_with_constraint(
//                         req,
//                         end + 1,
//                         constraint,
//                         constraint_list,
//                         blocked,
//                     );
//                 } else {
//                     //try next possible
//                     println!("nope");
//                 }
//             }
//         }
//     }
//     false
// }

// fn find_constraint(map: &HashMap<(Color, usize), Vec<Vec<Color>>>) -> Vec<Color> {
//     let mut res: Vec<Color> = Vec::new();
//     for color in Color::iter() {
//         match map.get(&(color, 1)) {
//             Some(_) => {}
//             None => res.push(color),
//         }
//     }
//     res
// }

pub fn process_part1(input: &str) -> String {
    let (stripes, requests) = parse(input);
    let map = create_map(&stripes);
    // for (key, value) in map.iter() {
    //     println!("{key:?}: {}", value.len());
    // }
    let mut reduced_list: Vec<Vec<Color>> = Vec::new();
    for stripe in stripes.iter() {
        let (check, current) = check_build(stripe, &map, 0, vec![]);
        if check {
            if current.len() > 1 {
                // don't need something that can be constructed from smaller
            } else {
                // can only be built by itself
                reduced_list.push(stripe.clone());
            }
        } else {
            panic!("shouldn't get here")
        }
    }
    let reduced_map = create_map(&reduced_list);
    //println!("reduced list: {reduced_list:?}");
    let mut total = 0;
    for req in requests.iter() {
        let (check, _current) = check_build(req, &reduced_map, 0, vec![]);
        if check {
            //println!("{i}: for {req:?} found {current:?}");
            total += 1;
        } else {
            //println!("{i}: no valid result");
        }
    }
    total.to_string()
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
        assert_eq!(process_part1(file), "6");
        //assert_eq!(process_part2(file), "16");
    }
}
