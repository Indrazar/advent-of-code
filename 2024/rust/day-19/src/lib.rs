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

// not fast
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

//not fast
fn is_possible<'a>(
    request: &'a [Color],
    stripes: &'a [Vec<Color>],
    map: &mut HashMap<(&'a [Color], &'a [Vec<Color>]), bool>,
) -> bool {
    if let Some(v) = map.get(&(request, stripes)) {
        return *v;
    }
    let res = stripes
        .iter()
        .map(|stripe| {
            if request.starts_with(stripe) {
                let new_request = &request[stripe.len()..];
                if new_request.is_empty() {
                    return true;
                }
                is_possible(new_request, stripes, map)
            } else {
                false
            }
        })
        .any(|x| x);
    map.insert((request, stripes), res);
    res
}

pub fn process_part1(input: &str) -> String {
    let (stripes, requests) = parse(input);
    let count: usize = requests
        .iter()
        .filter(|req| is_possible(req, &stripes, &mut HashMap::new()))
        .count();
    count.to_string()
}

fn count_possible<'a>(
    request: &'a [Color],
    stripes: &'a [Vec<Color>],
    map: &mut HashMap<(&'a [Color], &'a [Vec<Color>]), usize>,
) -> usize {
    if let Some(v) = map.get(&(request, stripes)) {
        return *v;
    }
    let res = stripes
        .iter()
        .filter_map(|stripe| {
            if request.starts_with(stripe) {
                let new_request = &request[stripe.len()..];
                if new_request.is_empty() {
                    return Some(1);
                }
                Some(count_possible(new_request, stripes, map))
            } else {
                None
            }
        })
        .sum();
    map.insert((request, stripes), res);
    res
}

pub fn process_part2(input: &str) -> String {
    let (stripes, requests) = parse(input);
    let count: usize = requests
        .iter()
        .map(|req| count_possible(req, &stripes, &mut HashMap::new()))
        .sum();
    count.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input() {
        let file = include_str!("../test-input-1.txt");
        assert_eq!(process_part1(file), "6");
        assert_eq!(process_part2(file), "16");
    }
}
