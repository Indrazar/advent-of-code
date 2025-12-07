use std::collections::BTreeMap;

#[derive(PartialEq, Eq, Clone, Copy)]
enum Tile {
    Start,
    Empty,
    Splitter,
    Beam,
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
struct Coord {
    x: usize,
    y: usize,
}

impl Coord {
    pub fn new(x: usize, y: usize) -> Coord {
        Coord { x, y }
    }
}

struct ActivatedTiles {
    beam: Vec<Coord>,
    splitter: Option<Coord>,
}

struct Data {
    map: Vec<Vec<Tile>>,
    activated_splitters: Vec<Coord>,
    active_beams: Vec<Coord>,
    active_splitters: Vec<Coord>,
}

impl Data {
    fn new(input: &str) -> Data {
        let mut map: Vec<Vec<Tile>> = Vec::new();
        let mut start_pos: Coord = Coord { x: 0, y: 0 };
        let mut start_found = false;
        for (y, line) in input.lines().enumerate() {
            let mut row: Vec<Tile> = Vec::new();
            for (x, char) in line.chars().enumerate() {
                match char {
                    '.' => row.push(Tile::Empty),
                    'S' => {
                        row.push(Tile::Start);
                        start_pos = Coord::new(x, y);
                        start_found = true;
                    }
                    '^' => row.push(Tile::Splitter),
                    _ => panic!("invalid character {char}"),
                }
            }
            map.push(row);
        }
        assert!(start_found);
        let active_beams: Vec<Coord> = vec![Coord {
            x: start_pos.x,
            y: start_pos.y,
        }];
        Data {
            map: map.clone(),
            active_beams,
            active_splitters: Vec::new(),
            activated_splitters: Vec::new(),
        }
    }

    fn propagate_beam(&self, beam: &Coord) -> ActivatedTiles {
        let y_max = self.map.len();
        if beam.y + 1 < y_max {
            match self.map[beam.y + 1][beam.x] {
                Tile::Start => panic!(
                    "beam iteration should not be at start: {},{}",
                    beam.x, beam.y
                ),
                Tile::Empty => {
                    //println!("beam propagated downward {},{}", beam.x, beam.y + 1);
                    ActivatedTiles {
                        beam: vec![Coord {
                            x: beam.x,
                            y: beam.y + 1,
                        }],
                        splitter: None,
                    }
                }
                Tile::Splitter => {
                    //println!("beam activated splitter {},{}", beam.x, beam.y + 1);
                    ActivatedTiles {
                        beam: vec![],
                        splitter: Some(Coord {
                            x: beam.x,
                            y: beam.y + 1,
                        }),
                    }
                }
                Tile::Beam => {
                    // println!(
                    //     "beam propagated downward to beam, subtracting a split {},{}",
                    //     beam.x,
                    //     beam.y + 1
                    // );
                    ActivatedTiles {
                        beam: vec![],
                        splitter: None,
                    }
                }
            }
        } else {
            //println!("beam exited manifold");
            ActivatedTiles {
                beam: vec![],
                splitter: None,
            }
        }
    }
    fn propagate_splitter(&self, splitter: &Coord) -> ActivatedTiles {
        let y_max = self.map.len();
        let x_max = self.map[0].len();
        let mut new_active_beams: Vec<Coord> = Vec::with_capacity(2);
        if splitter.y + 1 < y_max {
            //left
            if (splitter.x) > 0 {
                //print!("beam split left to: {},{}", splitter.x - 1, splitter.y + 1);
                new_active_beams.push(Coord {
                    x: splitter.x - 1,
                    y: splitter.y + 1,
                });
            } else {
                //println!("new beam would be outside manifold going left");
            }
            //right
            if (splitter.x + 1) < x_max {
                //print!("beam split right to: {},{}", splitter.x + 1, splitter.y + 1);
                new_active_beams.push(Coord {
                    x: splitter.x + 1,
                    y: splitter.y + 1,
                });
            } else {
                //println!("new beam would be outside manifold going right");
            }
            ActivatedTiles {
                beam: new_active_beams,
                splitter: None,
            }
        } else {
            //println!("new beam would start outside manifold");
            ActivatedTiles {
                beam: vec![],
                splitter: None,
            }
        }
    }

    fn iterate_active_beams(&mut self) {
        let mut new_active_beams: Vec<Coord> = Vec::new();
        let active_beams = self.active_beams.clone();
        for beam in active_beams {
            let result = self.propagate_beam(&beam);
            match (result.beam.len() > 0, result.splitter) {
                (false, None) => {
                    //exited
                }
                (false, Some(split)) => {
                    if !self.activated_splitters.contains(&split) {
                        self.activated_splitters.push(split);
                        self.active_splitters.push(split);
                    }
                }
                (true, None) => {
                    for res_beam in &result.beam {
                        self.map[res_beam.y][res_beam.x] = Tile::Beam;
                    }
                    new_active_beams.extend(result.beam)
                }
                (true, Some(_splitter)) => panic!("shouldn't be possible"),
            }
        }
        self.active_beams = new_active_beams;
    }
    fn iterate_active_splitters(&mut self) {
        let mut new_active_beams: Vec<Coord> = Vec::new();
        for splitter in &self.active_splitters {
            let result = self.propagate_splitter(splitter);
            match (result.beam.len() > 0, result.splitter) {
                (false, None) => {
                    //exited
                }
                (false, Some(_split)) => {
                    panic!("should not be possible");
                }
                (true, None) => {
                    for beam in &result.beam {
                        match self.map[beam.y][beam.x] {
                            Tile::Start => panic!("shouldn't go to the start"),
                            Tile::Empty => {
                                self.map[beam.y][beam.x] = Tile::Beam;
                            }
                            Tile::Splitter => panic!("shouldn't hit another splitter"),
                            Tile::Beam => {
                                //println!("we already have a beam here")
                                // cannot split here
                            }
                        };
                    }
                    new_active_beams.extend(result.beam)
                }
                (true, Some(_splitter)) => panic!("shouldn't be possible"),
            }
        }
        self.active_beams = new_active_beams;
        self.active_splitters.clear();
    }
}

pub fn process_part1(input: &str) -> String {
    let mut data = Data::new(input);
    while data.active_beams.len() > 0 || data.active_splitters.len() > 0 {
        while data.active_beams.len() > 0 {
            data.iterate_active_beams();
        }
        while data.active_splitters.len() > 0 {
            data.iterate_active_splitters();
        }
    }
    // for row in data.map {
    //     for col in row {
    //         match col {
    //             Tile::Start => print!("S"),
    //             Tile::Empty => print!("."),
    //             Tile::Splitter => print!("^"),
    //             Tile::Beam => print!("|"),
    //         }
    //     }
    //     println!()
    // }
    data.activated_splitters.len().to_string()
}

struct TraceData {
    ancestors: Vec<Coord>,
    current_location: Coord,
}

struct QData {
    map: Vec<Vec<Tile>>,
    active_traces: Vec<TraceData>,
    resolved_traces: BTreeMap<Coord, usize>,
    start_location: Coord,
}

impl QData {
    fn new(input: &str) -> QData {
        let mut map: Vec<Vec<Tile>> = Vec::new();
        let mut start_pos: Coord = Coord { x: 0, y: 0 };
        let mut start_found = false;
        for (y, line) in input.lines().enumerate() {
            let mut row: Vec<Tile> = Vec::new();
            for (x, char) in line.chars().enumerate() {
                match char {
                    '.' => row.push(Tile::Empty),
                    'S' => {
                        row.push(Tile::Start);
                        start_pos = Coord::new(x, y);
                        start_found = true;
                    }
                    '^' => row.push(Tile::Splitter),
                    _ => panic!("invalid character {char}"),
                }
            }
            map.push(row);
        }
        assert!(start_found);
        QData {
            map,
            active_traces: vec![TraceData {
                ancestors: vec![start_pos],
                current_location: start_pos,
            }],
            resolved_traces: BTreeMap::new(),
            start_location: start_pos,
        }
    }
    fn iterate_trace(&mut self, input: TraceData) {
        let y_max = self.map.len();
        let x_max = self.map[0].len();
        if input.current_location.y + 1 < y_max {
            match self.map[input.current_location.y + 1][input.current_location.x] {
                Tile::Start => panic!("can't be the start"),
                Tile::Empty => {
                    // before we make a new active trace validate we haven't walked it already
                    let new_location = Coord {
                        x: input.current_location.x,
                        y: input.current_location.y + 1,
                    };
                    if let Some(route_count) = self.resolved_traces.get(&new_location).copied() {
                        // we have an answer
                        for ancestor in input.ancestors {
                            self.resolved_traces
                                .entry(ancestor)
                                .and_modify(|x| {
                                    *x += route_count;
                                })
                                .or_insert(route_count);
                        }
                    } else {
                        self.active_traces.push(TraceData {
                            ancestors: input.ancestors.clone(),
                            current_location: new_location,
                        })
                    }
                }

                Tile::Splitter => {
                    //left
                    if input.current_location.x > 0 {
                        // before we make a new active trace validate we haven't walked it already
                        let new_location = Coord {
                            x: input.current_location.x - 1,
                            y: input.current_location.y + 1,
                        };
                        if let Some(route_count) = self.resolved_traces.get(&new_location).copied()
                        {
                            // we have an answer
                            for ancestor in &input.ancestors {
                                self.resolved_traces
                                    .entry(*ancestor)
                                    .and_modify(|x| {
                                        *x += route_count;
                                    })
                                    .or_insert(route_count);
                            }
                        } else {
                            let mut new_ancestors = input.ancestors.clone();
                            new_ancestors.push(Coord {
                                x: input.current_location.x,
                                y: input.current_location.y,
                            });
                            self.active_traces.push(TraceData {
                                ancestors: new_ancestors,
                                current_location: new_location,
                            })
                        }
                    }
                    //right
                    if input.current_location.x + 1 < x_max {
                        // before we make a new active trace validate we haven't walked it already
                        let new_location = Coord {
                            x: input.current_location.x + 1,
                            y: input.current_location.y + 1,
                        };
                        if let Some(route_count) = self.resolved_traces.get(&new_location).copied()
                        {
                            // we have an answer
                            for ancestor in &input.ancestors {
                                self.resolved_traces
                                    .entry(*ancestor)
                                    .and_modify(|x| {
                                        *x += route_count;
                                    })
                                    .or_insert(route_count);
                            }
                        } else {
                            let mut new_ancestors = input.ancestors.clone();
                            new_ancestors.push(Coord {
                                x: input.current_location.x,
                                y: input.current_location.y,
                            });
                            self.active_traces.push(TraceData {
                                ancestors: new_ancestors,
                                current_location: new_location,
                            })
                        }
                    }
                }
                Tile::Beam => panic!("can't be a beam"),
            }
        } else {
            // exited manifold
            // print!("exiting manifold, ancestors: ");
            for ancestor in input.ancestors {
                let _count = self
                    .resolved_traces
                    .entry(ancestor)
                    .and_modify(|x| *x += 1)
                    .or_insert(1);
                // print!("{},{} : {}; ", ancestor.x, ancestor.y, *count)
            }
            // println!();
        }
    }
    fn solve_one_trace(&mut self) {
        while self.active_traces.len() > 0 {
            let trace = self.active_traces.pop().unwrap();
            // have we solved this one?
            match self.resolved_traces.get(&trace.current_location).copied() {
                Some(route_count) => {
                    // we have an answer
                    for ancestor in trace.ancestors {
                        self.resolved_traces
                            .entry(ancestor)
                            .and_modify(|x| {
                                *x += route_count;
                            })
                            .or_insert(route_count);
                    }
                }
                None => {
                    // print!(
                    //     "iterating trace: {},{} with ancestors: ",
                    //     trace.current_location.x, trace.current_location.y
                    // );
                    // for ancestor in &trace.ancestors {
                    //     print!("{},{}:", ancestor.x, ancestor.y);
                    //     print!("{}; ", self.resolved_traces.get(ancestor).unwrap_or(&0));
                    // }
                    // println!();
                    self.iterate_trace(trace);
                }
            }
        }
    }
}

pub fn process_part2(input: &str) -> String {
    let mut data = QData::new(input);
    while data.active_traces.len() > 0 {
        data.solve_one_trace();
    }
    data.resolved_traces
        .get(&data.start_location)
        .expect("there should be a resolved start")
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input() {
        let file = include_str!("../test-input-1.txt");
        assert_eq!(process_part1(file), "21");
        assert_eq!(process_part2(file), "40");
    }
}
