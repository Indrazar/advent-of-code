use itertools::Itertools;
use nom::{bytes::complete::tag, character::complete, sequence::terminated};

// #[derive(Debug, Clone, Copy, PartialEq, Eq)]
// enum Dir {
//     U,
//     D,
//     L,
//     R,
// }

// +---+---+---+
// | 7 | 8 | 9 |
// +---+---+---+
// | 4 | 5 | 6 |
// +---+---+---+
// | 1 | 2 | 3 |
// +---+---+---+
//     | 0 | A |
//     +---+---+

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum NPad {
    Seven,
    Eight,
    Nine,
    Four,
    Five,
    Six,
    One,
    Two,
    Three,
    Zero,
    A,
    Invalid,
}

//     +---+---+
//     | ^ | A |
// +---+---+---+
// | < | v | > |
// +---+---+---+

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum DPad {
    U,
    A,
    L,
    D,
    R,
    Invalid,
}

type Seq = Vec<DPad>;

struct Coord {
    x: usize,
    y: usize,
}

const NMAP: [[NPad; 3]; 4] = [
    [NPad::Seven, NPad::Eight, NPad::Nine],
    [NPad::Four, NPad::Five, NPad::Six],
    [NPad::One, NPad::Two, NPad::Three],
    [NPad::Invalid, NPad::Zero, NPad::A],
];

const DMAP: [[DPad; 3]; 2] = [
    [DPad::Invalid, DPad::U, DPad::A],
    [DPad::L, DPad::D, DPad::R],
];

fn validate_dpad_path(start: DPad, end: DPad, inputs: &Vec<DPad>) -> bool {
    let mut up = false;
    let mut down = false;
    let mut left = false;
    let mut right = false;
    let mut last_dir: Option<DPad> = None;
    let mut current_coord = match start {
        DPad::U => Coord { x: 1, y: 0 },
        DPad::A => Coord { x: 2, y: 0 },
        DPad::L => Coord { x: 0, y: 1 },
        DPad::D => Coord { x: 1, y: 1 },
        DPad::R => Coord { x: 2, y: 1 },
        DPad::Invalid => todo!(),
    };
    for input in inputs {
        match (last_dir, input) {
            (None, _) => {}
            (_, DPad::Invalid) => panic!("should never get here"),
            (Some(DPad::Invalid), _) => panic!("should never get here"),
            (_, DPad::A) => panic!("should never get here"),
            (Some(DPad::A), _) => panic!("should never get here"),
            (Some(DPad::U), DPad::U) => {}
            (Some(DPad::L), DPad::L) => {}
            (Some(DPad::D), DPad::D) => {}
            (Some(DPad::R), DPad::R) => {}
            (Some(DPad::U), DPad::D) => {
                return false;
            }
            (Some(DPad::D), DPad::U) => {
                return false;
            }
            (Some(DPad::L), DPad::R) => {
                return false;
            }
            (Some(DPad::R), DPad::L) => {
                return false;
            }
            (Some(DPad::U), DPad::L) => {
                if left {
                    return false;
                } else {
                    left = true;
                    up = true
                }
            }
            (Some(DPad::U), DPad::R) => {
                if right {
                    return false;
                } else {
                    up = true;
                    right = true;
                }
            }
            (Some(DPad::L), DPad::U) => {
                if up {
                    return false;
                } else {
                    left = true;
                    up = true
                }
            }
            (Some(DPad::L), DPad::D) => {
                if down {
                    return false;
                } else {
                    left = true;
                    down = true;
                }
            }
            (Some(DPad::D), DPad::L) => {
                if left {
                    return false;
                } else {
                    down = true;
                    left = true;
                }
            }
            (Some(DPad::D), DPad::R) => {
                if right {
                    return false;
                } else {
                    down = true;
                    right = true
                }
            }
            (Some(DPad::R), DPad::U) => {
                if up {
                    return false;
                } else {
                    up = true;
                    right = true;
                }
            }
            (Some(DPad::R), DPad::D) => {
                if down {
                    return false;
                } else {
                    down = true;
                    right = true;
                }
            }
        }
        last_dir = Some(*input);
        match input {
            DPad::U => current_coord.y -= 1,
            DPad::A => {}
            DPad::L => current_coord.x -= 1,
            DPad::D => current_coord.y += 1,
            DPad::R => current_coord.x += 1,
            DPad::Invalid => panic!("should not get here"),
        }
        if DMAP[current_coord.y][current_coord.x] == DPad::Invalid {
            //println!("got to an invalid square");
            return false;
        }
    }
    if DMAP[current_coord.y][current_coord.x] == end {
        true
    } else {
        panic!("pathfinding is wrong");
    }
}

fn dpad_list_from_dpad(start: DPad, end: DPad) -> Vec<Seq> {
    let mut seq = match (start, end) {
        (DPad::U, DPad::U) => vec![],
        (DPad::U, DPad::A) => vec![DPad::R],
        (DPad::U, DPad::L) => vec![DPad::D, DPad::L],
        (DPad::U, DPad::D) => vec![DPad::D],
        (DPad::U, DPad::R) => vec![DPad::D, DPad::R],
        (DPad::A, DPad::U) => vec![DPad::L],
        (DPad::A, DPad::A) => vec![],
        (DPad::A, DPad::L) => vec![DPad::D, DPad::L, DPad::L],
        (DPad::A, DPad::D) => vec![DPad::D, DPad::L],
        (DPad::A, DPad::R) => vec![DPad::D],
        (DPad::L, DPad::U) => vec![DPad::R, DPad::U],
        (DPad::L, DPad::A) => vec![DPad::R, DPad::R, DPad::U],
        (DPad::L, DPad::L) => vec![],
        (DPad::L, DPad::D) => vec![DPad::R],
        (DPad::L, DPad::R) => vec![DPad::R, DPad::R],
        (DPad::D, DPad::U) => vec![DPad::U],
        (DPad::D, DPad::A) => vec![DPad::U, DPad::R],
        (DPad::D, DPad::L) => vec![DPad::L],
        (DPad::D, DPad::D) => vec![],
        (DPad::D, DPad::R) => vec![DPad::R],
        (DPad::R, DPad::U) => vec![DPad::U, DPad::L],
        (DPad::R, DPad::A) => vec![DPad::U],
        (DPad::R, DPad::L) => vec![DPad::L, DPad::L],
        (DPad::R, DPad::D) => vec![DPad::L],
        (DPad::R, DPad::R) => vec![],
        _ => panic!("dpad_list_from_dpad found invalid state"),
    };
    if seq.len() > 1 {
        let size: usize = seq.len();
        //println!("seq: {seq:?}");
        let mut res: Vec<Seq> = seq
            .into_iter()
            .permutations(size)
            .unique()
            .filter(|x| validate_dpad_path(start, end, x))
            .collect();
        //println!("res: {res:?}");
        for s in res.iter_mut() {
            s.push(DPad::A);
        }
        res
    } else {
        if !validate_dpad_path(start, end, &seq) {
            panic!("invalid route")
        }
        seq.push(DPad::A);
        vec![seq]
    }
}

fn validate_npad_path(start: NPad, end: NPad, inputs: &Vec<DPad>) -> bool {
    let mut current_coord = match start {
        NPad::Seven => Coord { x: 0, y: 0 },
        NPad::Eight => Coord { x: 1, y: 0 },
        NPad::Nine => Coord { x: 2, y: 0 },
        NPad::Four => Coord { x: 0, y: 1 },
        NPad::Five => Coord { x: 1, y: 1 },
        NPad::Six => Coord { x: 2, y: 1 },
        NPad::One => Coord { x: 0, y: 2 },
        NPad::Two => Coord { x: 1, y: 2 },
        NPad::Three => Coord { x: 2, y: 2 },
        NPad::Zero => Coord { x: 1, y: 3 },
        NPad::A => Coord { x: 2, y: 3 },
        NPad::Invalid => Coord { x: 0, y: 3 },
    };
    for input in inputs {
        match input {
            DPad::U => current_coord.y -= 1,
            DPad::A => {}
            DPad::L => current_coord.x -= 1,
            DPad::D => current_coord.y += 1,
            DPad::R => current_coord.x += 1,
            DPad::Invalid => todo!(),
        }
        if NMAP[current_coord.y][current_coord.x] == NPad::Invalid {
            return false;
        }
    }
    NMAP[current_coord.y][current_coord.x] == end
}

fn dpad_list_from_npad(start: NPad, end: NPad) -> Vec<Seq> {
    let mut seq = match (start, end) {
        (NPad::Seven, NPad::Seven) => vec![],
        (NPad::Seven, NPad::Eight) => vec![DPad::R],
        (NPad::Seven, NPad::Nine) => vec![DPad::R, DPad::R],
        (NPad::Seven, NPad::Four) => vec![DPad::D],
        (NPad::Seven, NPad::Five) => vec![DPad::D, DPad::R],
        (NPad::Seven, NPad::Six) => vec![DPad::D, DPad::R, DPad::R],
        (NPad::Seven, NPad::One) => vec![DPad::D, DPad::D],
        (NPad::Seven, NPad::Two) => vec![DPad::D, DPad::D, DPad::R],
        (NPad::Seven, NPad::Three) => vec![DPad::D, DPad::D, DPad::R, DPad::R],
        (NPad::Seven, NPad::Zero) => vec![DPad::R, DPad::D, DPad::D, DPad::D],
        (NPad::Seven, NPad::A) => vec![DPad::R, DPad::R, DPad::D, DPad::D, DPad::D],
        (NPad::Eight, NPad::Seven) => vec![DPad::L],
        (NPad::Eight, NPad::Eight) => vec![],
        (NPad::Eight, NPad::Nine) => vec![DPad::R],
        (NPad::Eight, NPad::Four) => vec![DPad::D, DPad::L],
        (NPad::Eight, NPad::Five) => vec![DPad::D],
        (NPad::Eight, NPad::Six) => vec![DPad::D, DPad::R],
        (NPad::Eight, NPad::One) => vec![DPad::D, DPad::D, DPad::L],
        (NPad::Eight, NPad::Two) => vec![DPad::D, DPad::D],
        (NPad::Eight, NPad::Three) => vec![DPad::D, DPad::D, DPad::R],
        (NPad::Eight, NPad::Zero) => vec![DPad::D, DPad::D, DPad::D],
        (NPad::Eight, NPad::A) => vec![DPad::D, DPad::D, DPad::D, DPad::R],
        (NPad::Nine, NPad::Seven) => vec![DPad::L, DPad::L],
        (NPad::Nine, NPad::Eight) => vec![DPad::L],
        (NPad::Nine, NPad::Nine) => vec![],
        (NPad::Nine, NPad::Four) => vec![DPad::D, DPad::L, DPad::L],
        (NPad::Nine, NPad::Five) => vec![DPad::D, DPad::L],
        (NPad::Nine, NPad::Six) => vec![DPad::D],
        (NPad::Nine, NPad::One) => vec![DPad::D, DPad::D, DPad::L, DPad::L],
        (NPad::Nine, NPad::Two) => vec![DPad::D, DPad::D, DPad::L],
        (NPad::Nine, NPad::Three) => vec![DPad::D, DPad::D],
        (NPad::Nine, NPad::Zero) => vec![DPad::D, DPad::D, DPad::D, DPad::L],
        (NPad::Nine, NPad::A) => vec![DPad::D, DPad::D, DPad::D],
        (NPad::Four, NPad::Seven) => vec![DPad::U],
        (NPad::Four, NPad::Eight) => vec![DPad::U, DPad::R],
        (NPad::Four, NPad::Nine) => vec![DPad::U, DPad::R, DPad::R],
        (NPad::Four, NPad::Four) => vec![],
        (NPad::Four, NPad::Five) => vec![DPad::R],
        (NPad::Four, NPad::Six) => vec![DPad::R, DPad::R],
        (NPad::Four, NPad::One) => vec![DPad::D],
        (NPad::Four, NPad::Two) => vec![DPad::D, DPad::R],
        (NPad::Four, NPad::Three) => vec![DPad::D, DPad::R, DPad::R],
        (NPad::Four, NPad::Zero) => vec![DPad::R, DPad::D, DPad::D],
        (NPad::Four, NPad::A) => vec![DPad::R, DPad::R, DPad::D, DPad::D],
        (NPad::Five, NPad::Seven) => vec![DPad::U, DPad::L],
        (NPad::Five, NPad::Eight) => vec![DPad::U],
        (NPad::Five, NPad::Nine) => vec![DPad::U, DPad::R],
        (NPad::Five, NPad::Four) => vec![DPad::L],
        (NPad::Five, NPad::Five) => vec![],
        (NPad::Five, NPad::Six) => vec![DPad::R],
        (NPad::Five, NPad::One) => vec![DPad::D, DPad::L],
        (NPad::Five, NPad::Two) => vec![DPad::D],
        (NPad::Five, NPad::Three) => vec![DPad::D, DPad::R],
        (NPad::Five, NPad::Zero) => vec![DPad::D, DPad::D],
        (NPad::Five, NPad::A) => vec![DPad::D, DPad::D, DPad::R],
        (NPad::Six, NPad::Seven) => vec![DPad::U, DPad::L, DPad::L],
        (NPad::Six, NPad::Eight) => vec![DPad::L, DPad::U],
        (NPad::Six, NPad::Nine) => vec![DPad::U],
        (NPad::Six, NPad::Four) => vec![DPad::L, DPad::L],
        (NPad::Six, NPad::Five) => vec![DPad::L],
        (NPad::Six, NPad::Six) => vec![],
        (NPad::Six, NPad::One) => vec![DPad::D, DPad::L, DPad::L],
        (NPad::Six, NPad::Two) => vec![DPad::D, DPad::L],
        (NPad::Six, NPad::Three) => vec![DPad::D],
        (NPad::Six, NPad::Zero) => vec![DPad::D, DPad::D, DPad::L],
        (NPad::Six, NPad::A) => vec![DPad::D, DPad::D],
        (NPad::One, NPad::Seven) => vec![DPad::U, DPad::U],
        (NPad::One, NPad::Eight) => vec![DPad::U, DPad::U, DPad::R],
        (NPad::One, NPad::Nine) => vec![DPad::U, DPad::U, DPad::R, DPad::R],
        (NPad::One, NPad::Four) => vec![DPad::U],
        (NPad::One, NPad::Five) => vec![DPad::U, DPad::R],
        (NPad::One, NPad::Six) => vec![DPad::U, DPad::R, DPad::R],
        (NPad::One, NPad::One) => vec![],
        (NPad::One, NPad::Two) => vec![DPad::R],
        (NPad::One, NPad::Three) => vec![DPad::R, DPad::R],
        (NPad::One, NPad::Zero) => vec![DPad::R, DPad::D],
        (NPad::One, NPad::A) => vec![DPad::R, DPad::R, DPad::D],
        (NPad::Two, NPad::Seven) => vec![DPad::U, DPad::U, DPad::L],
        (NPad::Two, NPad::Eight) => vec![DPad::U, DPad::U],
        (NPad::Two, NPad::Nine) => vec![DPad::R, DPad::U, DPad::U],
        (NPad::Two, NPad::Four) => vec![DPad::U, DPad::L],
        (NPad::Two, NPad::Five) => vec![DPad::U],
        (NPad::Two, NPad::Six) => vec![DPad::U, DPad::R],
        (NPad::Two, NPad::One) => vec![DPad::L],
        (NPad::Two, NPad::Two) => vec![],
        (NPad::Two, NPad::Three) => vec![DPad::R],
        (NPad::Two, NPad::Zero) => vec![DPad::D],
        (NPad::Two, NPad::A) => vec![DPad::D, DPad::R],
        (NPad::Three, NPad::Seven) => vec![DPad::U, DPad::U, DPad::L, DPad::L],
        (NPad::Three, NPad::Eight) => vec![DPad::U, DPad::U, DPad::L],
        (NPad::Three, NPad::Nine) => vec![DPad::U, DPad::U],
        (NPad::Three, NPad::Four) => vec![DPad::U, DPad::L, DPad::L],
        (NPad::Three, NPad::Five) => vec![DPad::U, DPad::L],
        (NPad::Three, NPad::Six) => vec![DPad::U],
        (NPad::Three, NPad::One) => vec![DPad::L, DPad::L],
        (NPad::Three, NPad::Two) => vec![DPad::L],
        (NPad::Three, NPad::Three) => vec![],
        (NPad::Three, NPad::Zero) => vec![DPad::D, DPad::L],
        (NPad::Three, NPad::A) => vec![DPad::D],
        (NPad::Zero, NPad::Seven) => vec![DPad::U, DPad::U, DPad::U, DPad::L],
        (NPad::Zero, NPad::Eight) => vec![DPad::U, DPad::U, DPad::U],
        (NPad::Zero, NPad::Nine) => vec![DPad::U, DPad::U, DPad::U, DPad::R],
        (NPad::Zero, NPad::Four) => vec![DPad::U, DPad::U, DPad::L],
        (NPad::Zero, NPad::Five) => vec![DPad::U, DPad::U],
        (NPad::Zero, NPad::Six) => vec![DPad::U, DPad::U, DPad::R],
        (NPad::Zero, NPad::One) => vec![DPad::U, DPad::L],
        (NPad::Zero, NPad::Two) => vec![DPad::U],
        (NPad::Zero, NPad::Three) => vec![DPad::U, DPad::R],
        (NPad::Zero, NPad::Zero) => vec![],
        (NPad::Zero, NPad::A) => vec![DPad::R],
        (NPad::A, NPad::Seven) => vec![DPad::U, DPad::U, DPad::U, DPad::L, DPad::L],
        (NPad::A, NPad::Eight) => vec![DPad::U, DPad::U, DPad::U, DPad::L],
        (NPad::A, NPad::Nine) => vec![DPad::U, DPad::U, DPad::U],
        (NPad::A, NPad::Four) => vec![DPad::U, DPad::U, DPad::L, DPad::L],
        (NPad::A, NPad::Five) => vec![DPad::U, DPad::U, DPad::L],
        (NPad::A, NPad::Six) => vec![DPad::U, DPad::U],
        (NPad::A, NPad::One) => vec![DPad::U, DPad::L, DPad::L],
        (NPad::A, NPad::Two) => vec![DPad::U, DPad::L],
        (NPad::A, NPad::Three) => vec![DPad::U],
        (NPad::A, NPad::Zero) => vec![DPad::L],
        (NPad::A, NPad::A) => vec![],
        _ => panic!("dpad_list_from_npad found invalid state"),
    };
    if seq.len() > 1 {
        let size: usize = seq.len();
        //println!("seq: {seq:?}");
        let mut res: Vec<Seq> = seq
            .into_iter()
            .permutations(size)
            .unique()
            .filter(|x| validate_npad_path(start, end, x))
            .collect();
        //println!("res: {res:?}");
        for s in res.iter_mut() {
            s.push(DPad::A);
        }
        res
    } else {
        if !validate_npad_path(start, end, &seq) {
            panic!("invalid route")
        }
        seq.push(DPad::A);
        vec![seq]
    }
}

fn parse(input: &str) -> Vec<(u32, Vec<NPad>)> {
    let mut res: Vec<(u32, Vec<NPad>)> = Vec::new();
    for line in input.lines() {
        let (_, number) = terminated(complete::u32::<&str, nom::error::Error<_>>, tag("A"))(line)
            .expect("there should be a number");
        let mut code: Vec<NPad> = Vec::new();
        for ch in line.chars() {
            let key = match ch {
                '0' => NPad::Zero,
                '1' => NPad::One,
                '2' => NPad::Two,
                '3' => NPad::Three,
                '4' => NPad::Four,
                '5' => NPad::Five,
                '6' => NPad::Six,
                '7' => NPad::Seven,
                '8' => NPad::Eight,
                '9' => NPad::Nine,
                'A' => NPad::A,
                unknown => panic!("unknown character in parse: {unknown}"),
            };
            code.push(key);
        }
        res.push((number, code));
    }
    res
}

pub fn process_part1(input: &str) -> String {
    let codes = parse(input);
    //println!("{codes:?}");
    let mut total: usize = 0;
    for (val, code) in codes {
        //innermost robot
        //println!("code: {}: {code:?}", code.len());
        // start on A go to first key
        let mut robot1_task: Vec<Seq> = dpad_list_from_npad(NPad::A, code[0]);
        for pair in code.windows(2) {
            let start = pair[0];
            let end = pair[1];
            let mut new_task_list: Vec<Seq> = Vec::new();
            let new_step_list = dpad_list_from_npad(start, end);
            for old_steps in robot1_task.iter() {
                //println!("robot 1: {new_step_list:?}");
                for new_steps in new_step_list.iter() {
                    let mut result_steps = old_steps.clone();
                    result_steps.extend(new_steps);
                    new_task_list.push(result_steps);
                }
            }
            robot1_task = new_task_list;
        }
        // println!("robot1: {}: {robot1_task:?}", robot1_task.len());
        let mut shortest1 = usize::MAX;
        for seq1 in robot1_task.iter() {
            //println!("{}: {:?}", seq1.len(), seq1);
            if seq1.len() < shortest1 {
                shortest1 = seq1.len();
            }
        }
        // println!("robot1 shortest: {shortest1}");
        let mut robot2_total: Vec<Seq> = Vec::new();
        let mut robot2_task;
        for seq1 in robot1_task.iter() {
            // start on A go to first key
            //println!("start: A, end: {:?}", seq1[0]);
            robot2_task = dpad_list_from_dpad(DPad::A, seq1[0]);
            //println!("robot 2: {robot2_task:?}");
            for pair in seq1.windows(2) {
                let start = pair[0];
                let end = pair[1];
                //println!("robot2_task so far: {robot2_task:?}");
                //println!("start: {start:?}, end: {end:?}");
                let mut new_task_list: Vec<Seq> = Vec::new();
                let new_step_list = dpad_list_from_dpad(start, end);
                //println!("robot 2: {new_step_list:?}");
                for old_steps in robot2_task.iter() {
                    for new_steps in new_step_list.iter() {
                        let mut result_steps = old_steps.clone();
                        result_steps.extend(new_steps);
                        new_task_list.push(result_steps);
                    }
                }
                robot2_task = new_task_list;
            }
            robot2_total.extend(robot2_task);
        }
        let mut shortest2 = usize::MAX;
        for seq2 in robot2_total.iter() {
            //println!("{}: {:?}", seq2.len(), seq2);
            if seq2.len() < shortest2 {
                shortest2 = seq2.len();
            }
        }
        // println!("robot2 shortest: {shortest2}");
        // println!("robot2: {}: {robot2_task:?}", robot2_task.len());
        let mut robot3_total: Vec<Seq> = Vec::new();
        let mut robot3_task;
        for seq1 in robot2_total.iter() {
            // start on A go to first key
            //println!("start: A, end: {:?}", seq1[0]);
            robot3_task = dpad_list_from_dpad(DPad::A, seq1[0]);
            //println!("robot 3: {robot3_task:?}");
            for pair in seq1.windows(2) {
                let start = pair[0];
                let end = pair[1];
                //println!("robot3_task so far: {robot3_task:?}");
                //println!("start: {start:?}, end: {end:?}");
                let mut new_task_list: Vec<Seq> = Vec::new();
                let new_step_list = dpad_list_from_dpad(start, end);
                //println!("robot 3: {new_step_list:?}");
                for old_steps in robot3_task.iter() {
                    for new_steps in new_step_list.iter() {
                        let mut result_steps = old_steps.clone();
                        result_steps.extend(new_steps);
                        new_task_list.push(result_steps);
                    }
                }
                robot3_task = new_task_list;
            }
            robot3_total.extend(robot3_task);
        }
        let mut shortest3 = usize::MAX;
        //print!("lengths: ");
        for seq3 in robot3_total.iter() {
            //print!("{}, ", seq3.len());
            if seq3.len() < shortest3 {
                shortest3 = seq3.len();
            }
        }
        //println!("robot3 shortest: {shortest3}");
        // println!(
        //     "robot3 routes: {}:\nroute0: {}: {:?}",
        //     robot3_total.len(),
        //     robot3_total[0].len(),
        //     robot3_total[0]
        // );
        //println!("complexity: {}*{}", shortest3, val);
        total += val as usize * shortest3;
    }
    total.to_string()
}

pub fn process_part2(input: &str) -> String {
    "works".to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn find_npad_endpoint(start: NPad, inputs: Vec<DPad>) -> NPad {
        let mut current_coord = match start {
            NPad::Seven => Coord { x: 0, y: 0 },
            NPad::Eight => Coord { x: 1, y: 0 },
            NPad::Nine => Coord { x: 2, y: 0 },
            NPad::Four => Coord { x: 0, y: 1 },
            NPad::Five => Coord { x: 1, y: 1 },
            NPad::Six => Coord { x: 2, y: 1 },
            NPad::One => Coord { x: 0, y: 2 },
            NPad::Two => Coord { x: 1, y: 2 },
            NPad::Three => Coord { x: 2, y: 2 },
            NPad::Zero => Coord { x: 1, y: 3 },
            NPad::A => Coord { x: 2, y: 3 },
            NPad::Invalid => Coord { x: 0, y: 3 },
        };
        for input in inputs {
            match input {
                DPad::U => current_coord.y -= 1,
                DPad::A => {}
                DPad::L => current_coord.x -= 1,
                DPad::D => current_coord.y += 1,
                DPad::R => current_coord.x += 1,
                DPad::Invalid => todo!(),
            }
            assert_ne!(NMAP[current_coord.y][current_coord.x], NPad::Invalid)
        }
        NMAP[current_coord.y][current_coord.x]
    }

    fn find_dpad_endpoint(start: DPad, inputs: Vec<DPad>) -> DPad {
        let mut current_coord = match start {
            DPad::U => Coord { x: 1, y: 0 },
            DPad::A => Coord { x: 2, y: 0 },
            DPad::L => Coord { x: 0, y: 1 },
            DPad::D => Coord { x: 1, y: 1 },
            DPad::R => Coord { x: 2, y: 1 },
            DPad::Invalid => todo!(),
        };
        for input in inputs {
            match input {
                DPad::U => current_coord.y -= 1,
                DPad::A => {}
                DPad::L => current_coord.x -= 1,
                DPad::D => current_coord.y += 1,
                DPad::R => current_coord.x += 1,
                DPad::Invalid => todo!(),
            }
            assert_ne!(DMAP[current_coord.y][current_coord.x], DPad::Invalid)
        }
        DMAP[current_coord.y][current_coord.x]
    }

    fn validate_npad_route_count(start: NPad, end: NPad, seq_count: usize) -> bool {
        println!("recieved {seq_count} sequences");
        seq_count
            == match (start, end) {
                (NPad::Invalid, _) => panic!("shouldn't get here"),
                (_, NPad::Invalid) => panic!("shouldn't get here"),
                (NPad::Seven, NPad::Seven) => 1,
                (NPad::Seven, NPad::Eight) => 1,
                (NPad::Seven, NPad::Nine) => 1,
                (NPad::Seven, NPad::Four) => 1,
                (NPad::Seven, NPad::Five) => 2,
                (NPad::Seven, NPad::Six) => 3,
                (NPad::Seven, NPad::One) => 1,
                (NPad::Seven, NPad::Two) => 3,
                (NPad::Seven, NPad::Three) => 6,
                (NPad::Seven, NPad::Zero) => 3,
                (NPad::Seven, NPad::A) => 9,
                (NPad::Eight, NPad::Seven) => 1,
                (NPad::Eight, NPad::Eight) => 1,
                (NPad::Eight, NPad::Nine) => 1,
                (NPad::Eight, NPad::Four) => 2,
                (NPad::Eight, NPad::Five) => 1,
                (NPad::Eight, NPad::Six) => 2,
                (NPad::Eight, NPad::One) => 3,
                (NPad::Eight, NPad::Two) => 1,
                (NPad::Eight, NPad::Three) => 3,
                (NPad::Eight, NPad::Zero) => 1,
                (NPad::Eight, NPad::A) => 4,
                (NPad::Nine, NPad::Seven) => 1,
                (NPad::Nine, NPad::Eight) => 1,
                (NPad::Nine, NPad::Nine) => 1,
                (NPad::Nine, NPad::Four) => 3,
                (NPad::Nine, NPad::Five) => 2,
                (NPad::Nine, NPad::Six) => 1,
                (NPad::Nine, NPad::One) => 6,
                (NPad::Nine, NPad::Two) => 3,
                (NPad::Nine, NPad::Three) => 1,
                (NPad::Nine, NPad::Zero) => 4,
                (NPad::Nine, NPad::A) => 1,
                (NPad::Four, NPad::Seven) => 1,
                (NPad::Four, NPad::Eight) => 2,
                (NPad::Four, NPad::Nine) => 3,
                (NPad::Four, NPad::Four) => 1,
                (NPad::Four, NPad::Five) => 1,
                (NPad::Four, NPad::Six) => 1,
                (NPad::Four, NPad::One) => 1,
                (NPad::Four, NPad::Two) => 2,
                (NPad::Four, NPad::Three) => 3,
                (NPad::Four, NPad::Zero) => 2,
                (NPad::Four, NPad::A) => 5,
                (NPad::Five, NPad::Seven) => 2,
                (NPad::Five, NPad::Eight) => 1,
                (NPad::Five, NPad::Nine) => 2,
                (NPad::Five, NPad::Four) => 1,
                (NPad::Five, NPad::Five) => 1,
                (NPad::Five, NPad::Six) => 1,
                (NPad::Five, NPad::One) => 2,
                (NPad::Five, NPad::Two) => 1,
                (NPad::Five, NPad::Three) => 2,
                (NPad::Five, NPad::Zero) => 1,
                (NPad::Five, NPad::A) => 3,
                (NPad::Six, NPad::Seven) => 3,
                (NPad::Six, NPad::Eight) => 2,
                (NPad::Six, NPad::Nine) => 1,
                (NPad::Six, NPad::Four) => 1,
                (NPad::Six, NPad::Five) => 1,
                (NPad::Six, NPad::Six) => 1,
                (NPad::Six, NPad::One) => 3,
                (NPad::Six, NPad::Two) => 2,
                (NPad::Six, NPad::Three) => 1,
                (NPad::Six, NPad::Zero) => 3,
                (NPad::Six, NPad::A) => 1,
                (NPad::One, NPad::Seven) => 1,
                (NPad::One, NPad::Eight) => 3,
                (NPad::One, NPad::Nine) => 6,
                (NPad::One, NPad::Four) => 1,
                (NPad::One, NPad::Five) => 2,
                (NPad::One, NPad::Six) => 3,
                (NPad::One, NPad::One) => 1,
                (NPad::One, NPad::Two) => 1,
                (NPad::One, NPad::Three) => 1,
                (NPad::One, NPad::Zero) => 1,
                (NPad::One, NPad::A) => 2,
                (NPad::Two, NPad::Seven) => 3,
                (NPad::Two, NPad::Eight) => 1,
                (NPad::Two, NPad::Nine) => 3,
                (NPad::Two, NPad::Four) => 2,
                (NPad::Two, NPad::Five) => 1,
                (NPad::Two, NPad::Six) => 2,
                (NPad::Two, NPad::One) => 1,
                (NPad::Two, NPad::Two) => 1,
                (NPad::Two, NPad::Three) => 1,
                (NPad::Two, NPad::Zero) => 1,
                (NPad::Two, NPad::A) => 2,
                (NPad::Three, NPad::Seven) => 6,
                (NPad::Three, NPad::Eight) => 3,
                (NPad::Three, NPad::Nine) => 1,
                (NPad::Three, NPad::Four) => 3,
                (NPad::Three, NPad::Five) => 2,
                (NPad::Three, NPad::Six) => 1,
                (NPad::Three, NPad::One) => 1,
                (NPad::Three, NPad::Two) => 1,
                (NPad::Three, NPad::Three) => 1,
                (NPad::Three, NPad::Zero) => 2,
                (NPad::Three, NPad::A) => 1,
                (NPad::Zero, NPad::Seven) => 3,
                (NPad::Zero, NPad::Eight) => 1,
                (NPad::Zero, NPad::Nine) => 4,
                (NPad::Zero, NPad::Four) => 2,
                (NPad::Zero, NPad::Five) => 1,
                (NPad::Zero, NPad::Six) => 3,
                (NPad::Zero, NPad::One) => 1,
                (NPad::Zero, NPad::Two) => 1,
                (NPad::Zero, NPad::Three) => 2,
                (NPad::Zero, NPad::Zero) => 1,
                (NPad::Zero, NPad::A) => 1,
                (NPad::A, NPad::Seven) => 9,
                (NPad::A, NPad::Eight) => 4,
                (NPad::A, NPad::Nine) => 1,
                (NPad::A, NPad::Four) => 5,
                (NPad::A, NPad::Five) => 3,
                (NPad::A, NPad::Six) => 1,
                (NPad::A, NPad::One) => 2,
                (NPad::A, NPad::Two) => 2,
                (NPad::A, NPad::Three) => 1,
                (NPad::A, NPad::Zero) => 1,
                (NPad::A, NPad::A) => 1,
            }
    }

    fn validate_dpad_route_count(start: DPad, end: DPad, seq_count: usize) -> bool {
        println!("recieved {seq_count} sequences");
        seq_count
            == match (start, end) {
                (DPad::Invalid, _) => panic!("shouldn't get here"),
                (_, DPad::Invalid) => panic!("shouldn't get here"),
                (DPad::U, DPad::U) => 1,
                (DPad::U, DPad::A) => 1,
                (DPad::U, DPad::L) => 1,
                (DPad::U, DPad::D) => 1,
                (DPad::U, DPad::R) => 2,
                (DPad::A, DPad::U) => 1,
                (DPad::A, DPad::A) => 1,
                (DPad::A, DPad::L) => 1,
                (DPad::A, DPad::D) => 2,
                (DPad::A, DPad::R) => 1,
                (DPad::L, DPad::U) => 1,
                (DPad::L, DPad::A) => 1,
                (DPad::L, DPad::L) => 1,
                (DPad::L, DPad::D) => 1,
                (DPad::L, DPad::R) => 1,
                (DPad::D, DPad::U) => 1,
                (DPad::D, DPad::A) => 2,
                (DPad::D, DPad::L) => 1,
                (DPad::D, DPad::D) => 1,
                (DPad::D, DPad::R) => 1,
                (DPad::R, DPad::U) => 2,
                (DPad::R, DPad::A) => 1,
                (DPad::R, DPad::L) => 1,
                (DPad::R, DPad::D) => 1,
                (DPad::R, DPad::R) => 1,
            }
    }

    #[test]
    fn test_dpad() {
        assert_eq!(
            dpad_list_from_dpad(DPad::A, DPad::R),
            vec![vec![DPad::D, DPad::A]]
        );

        let robot1_task = vec![
            vec![
                DPad::L,
                DPad::A,
                DPad::U,
                DPad::A,
                DPad::R,
                DPad::U,
                DPad::U,
                DPad::A,
                DPad::D,
                DPad::D,
                DPad::D,
                DPad::A,
            ],
            vec![
                DPad::L,
                DPad::A,
                DPad::U,
                DPad::A,
                DPad::U,
                DPad::R,
                DPad::U,
                DPad::A,
                DPad::D,
                DPad::D,
                DPad::D,
                DPad::A,
            ],
        ];
        let mut robot2_total: Vec<Seq> = Vec::new();
        let mut robot2_task;
        for seq1 in robot1_task.iter() {
            // start on A go to first key
            println!("start: A, end: {:?}", seq1[0]);
            robot2_task = dpad_list_from_dpad(DPad::A, seq1[0]);
            println!("robot 2: {robot2_task:?}");
            for pair in seq1.windows(2) {
                let start = pair[0];
                let end = pair[1];
                println!("robot2_task so far: {robot2_task:?}");
                println!("start: {start:?}, end: {end:?}");
                let mut new_task_list: Vec<Seq> = Vec::new();
                let new_step_list = dpad_list_from_dpad(start, end);
                println!("robot 2: {new_step_list:?}");
                for old_steps in robot2_task.iter() {
                    for new_steps in new_step_list.iter() {
                        let mut result_steps = old_steps.clone();
                        result_steps.extend(new_steps);
                        new_task_list.push(result_steps);
                    }
                }
                robot2_task = new_task_list;
            }
            robot2_total.extend(robot2_task);
        }
        assert_eq!(
            robot2_total.contains(&vec![
                DPad::D,
                DPad::L,
                DPad::L,
                DPad::A,
                DPad::R,
                DPad::R,
                DPad::U,
                DPad::A,
                DPad::L,
                DPad::A,
                DPad::R,
                DPad::A,
                DPad::D,
                DPad::A,
                DPad::L,
                DPad::U,
                DPad::A,
                DPad::A,
                DPad::R,
                DPad::A,
                DPad::L,
                DPad::D,
                DPad::A,
                DPad::A,
                DPad::A,
                DPad::R,
                DPad::U,
                DPad::A
            ]),
            true
        )
    }

    #[test]
    fn test_keypad() {
        let npad_list = [
            NPad::Nine,
            NPad::Eight,
            NPad::Seven,
            NPad::Six,
            NPad::Five,
            NPad::Four,
            NPad::Three,
            NPad::Two,
            NPad::One,
            NPad::Zero,
            NPad::A,
        ];
        let dpad_list = [DPad::U, DPad::D, DPad::L, DPad::R, DPad::A];
        for start in npad_list {
            for end in npad_list {
                println!("starting {start:?}, ending {end:?}");
                let sequence_list = dpad_list_from_npad(start, end);
                assert_eq!(
                    validate_npad_route_count(start, end, sequence_list.len()),
                    true
                );
                for instruction_list in sequence_list {
                    println!("start: {start:?}, end: {end:?} checking {instruction_list:?}");
                    assert_eq!(find_npad_endpoint(start, instruction_list), end)
                }
            }
        }
        for start in dpad_list {
            for end in dpad_list {
                println!("starting {start:?}, ending {end:?}");
                let sequence_list = dpad_list_from_dpad(start, end);
                assert_eq!(
                    validate_dpad_route_count(start, end, sequence_list.len()),
                    true
                );
                for instruction_list in sequence_list {
                    println!("start: {start:?}, end: {end:?} checking {instruction_list:?}");
                    assert_eq!(find_dpad_endpoint(start, instruction_list), end)
                }
            }
        }
    }

    #[test]
    fn test_input() {
        let file = include_str!("../test-input-1.txt");
        assert_eq!(process_part1(file), "126384");
    }
}
