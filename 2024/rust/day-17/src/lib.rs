use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete,
    multi::many1,
    sequence::{preceded, terminated},
    IResult,
};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct State {
    ip: usize,
    a: i128,
    b: i128,
    c: i128,
    output: Vec<i8>,
}

impl State {
    // 0
    fn adv(&mut self, operand: i8) {
        let operand = match operand {
            7 => panic!("adv failed, operand is 7 (reserved)"),
            6 => self.c,
            5 => self.b,
            4 => self.a,
            0..=3 => i128::from(operand),
            operand => panic!("adv failed, operand is invalid: {operand}"),
        };
        //println!("operand calculated to be: {operand}");
        if operand < 0 {
            self.a *= 2i128.pow(operand.abs().try_into().unwrap())
        } else {
            self.a /= 2i128.pow(operand.try_into().unwrap());
        }
        //println!("a set to {}", self.a);
        self.ip += 2;
    }
    // 1
    fn bxl(&mut self, operand: i8) {
        if !(0..=7).contains(&operand) {
            panic!("bxl failed, operand is invalid: {operand}");
        }
        self.b ^= i128::from(operand);
        self.ip += 2;
    }
    // 2
    fn bst(&mut self, operand: i8) {
        let operand = match operand {
            7 => panic!("adv failed, operand is 7 (reserved)"),
            6 => self.c,
            5 => self.b,
            4 => self.a,
            0..=3 => i128::from(operand),
            operand => panic!("adv failed, operand is invalid: {operand}"),
        };
        let operand = operand % 8;
        self.b = operand;
        self.ip += 2;
    }
    // 3
    fn jnz(&mut self, operand: i8) {
        if !(0..=7).contains(&operand) {
            panic!("jnz failed, operand is invalid: {operand}");
        }
        if self.a == 0 {
            //println!("no jump");
            self.ip += 2;
        } else {
            //println!("jumping to {operand}");
            self.ip = operand.try_into().unwrap();
        }
    }
    // 4
    fn bxc(&mut self, _operand: i8) {
        self.b ^= self.c;
        self.ip += 2;
    }
    // 5
    fn out(&mut self, operand: i8) {
        let operand = match operand {
            7 => panic!("adv failed, operand is 7 (reserved)"),
            6 => self.c,
            5 => self.b,
            4 => self.a,
            0..=3 => i128::from(operand),
            operand => panic!("adv failed, operand is invalid: {operand}"),
        };
        let operand = operand % 8;
        //println!("outputting {operand}");
        self.output.push(operand.try_into().unwrap());
        self.ip += 2;
    }
    // 6
    fn bdv(&mut self, operand: i8) {
        let operand = match operand {
            7 => panic!("adv failed, operand is 7 (reserved)"),
            6 => self.c,
            5 => self.b,
            4 => self.a,
            0..=3 => i128::from(operand),
            operand => panic!("adv failed, operand is invalid: {operand}"),
        };
        if operand < 0 {
            self.b = self.a * 2i128.pow(operand.abs().try_into().unwrap());
        } else {
            self.b = self.a / 2i128.pow(operand.try_into().unwrap());
        }
        self.ip += 2;
    }
    // 7
    fn cdv(&mut self, operand: i8) {
        let operand = match operand {
            7 => panic!("adv failed, operand is 7 (reserved)"),
            6 => self.c,
            5 => self.b,
            4 => self.a,
            0..=3 => i128::from(operand),
            operand => panic!("adv failed, operand is invalid: {operand}"),
        };
        if operand < 0 {
            self.c = self.a * 2i128.pow(operand.abs().try_into().unwrap());
        } else {
            self.c = self.a / 2i128.pow(operand.try_into().unwrap());
        }
        self.ip += 2;
    }
    fn run_step(&mut self, instr_tape: &[i8]) {
        let opcode = instr_tape[self.ip];
        let operand = instr_tape[self.ip + 1];
        //println!("opcode {opcode}, operand: {operand}");

        match opcode {
            7 => {
                //println!("opcode decodes to cdv");
                self.cdv(operand);
            }
            6 => {
                //println!("opcode decodes to bdv");
                self.bdv(operand);
            }
            5 => {
                //println!("opcode decodes to out");
                self.out(operand);
            }
            4 => {
                //println!("opcode decodes to bxc");
                self.bxc(operand);
            }
            3 => {
                //println!("opcode decodes to jnz");
                self.jnz(operand);
            }
            2 => {
                //println!("opcode decodes to bst");
                self.bst(operand);
            }
            1 => {
                //println!("opcode decodes to bxl");
                self.bxl(operand);
            }
            0 => {
                //println!("opcode decodes to adv");
                self.adv(operand);
            }
            unknown => panic!("invalid opcode {unknown}"),
        }
    }
    fn run(&mut self, instr_tape: &[i8]) {
        if instr_tape.len() % 2 != 0 {
            panic!("invalid instruction tape, must have an even number of values")
        }
        while self.ip < instr_tape.len() {
            self.run_step(instr_tape);
        }
    }
    // fn run_known_output(&mut self, instr_tape: &Vec<i8>, known_output: &Vec<i8>) -> bool {
    //     if instr_tape.len() % 2 != 0 {
    //         panic!("invalid instruction tape, must have an even number of values")
    //     }
    //     while self.ip < instr_tape.len() {
    //         let check = if instr_tape[self.ip] == 5 {
    //             //println!("found an out instruction");
    //             true
    //         } else {
    //             false
    //         };
    //         self.run_step(instr_tape);
    //         if check {
    //             // println!(
    //             //     "checking output against known: \
    //             //     \ncurrent: {:?}\
    //             //     \nknown: {:?}",
    //             //     self.output, known_output
    //             // );
    //             if self.output.len() > known_output.len() {
    //                 return false;
    //             } else {
    //                 if self.output[self.output.len() - 1] == known_output[self.output.len() - 1] {
    //                     //println!("matches, continuing");
    //                     // keep going
    //                 } else {
    //                     //println!("doesn't match, ejecting");
    //                     return false;
    //                 }
    //             }
    //         }
    //     }
    //     // println!(
    //     //     "final check against known: \
    //     //     \ncurrent: {:?}\
    //     //     \nknown: {:?}",
    //     //     self.output, known_output
    //     // );
    //     if self.output.len() != known_output.len() {
    //         //println!("length does not match, ejecting");
    //         return false;
    //     } else {
    //         for (loc, val) in self.output.iter().enumerate() {
    //             if *val != known_output[loc] {
    //                 //println!("does not match, ejecting");
    //                 return false;
    //             }
    //         }
    //     }
    //     //println!("matches!");
    //     true
    // }
}

// used to avoid the complier not understanding the error type and also avoids lifetimes!

#[inline]
fn parse_i128(input: &str) -> IResult<&str, i128> {
    complete::i128(input)
}

#[inline]
fn parse_i8(input: &str) -> IResult<&str, i8> {
    complete::i8(input)
}

fn parse(input: &str) -> (State, Vec<i8>) {
    let (input, a) = preceded(tag("Register A: "), parse_i128)(input)
        .expect("there should be a Register A value");
    let (input, b) = preceded(tag("\nRegister B: "), parse_i128)(input)
        .expect("there should be a Register B value");
    let (input, c) = preceded(tag("\nRegister C: "), parse_i128)(input)
        .expect("there should be a register C value");
    let (_, tape) = preceded(
        tag("\n\nProgram: "),
        many1(terminated(parse_i8, alt((tag(","), tag(""))))),
    )(input)
    .expect("there should be a program listing");
    (
        State {
            ip: 0,
            a,
            b,
            c,
            output: vec![],
        },
        tape,
    )
}

pub fn process_part1(input: &str) -> String {
    let (mut state, tape) = parse(input);
    //println!("state: {state:?}, tape: {tape:?}");
    state.run(&tape);
    let mut res = String::new();
    for val in state.output {
        if !(0..=9).contains(&val) {
            panic!("can't fit {val} into a char");
        }
        res.push(
            val.to_string()
                .chars()
                .next()
                .expect("should be a single char"),
        );
        res.push(',');
    }
    res.pop();
    res
}

fn go_backwards(state: &State, rev_iteration: usize, tape: &Vec<i8>) -> Option<i128> {
    //println!("\niteration: {}, passed state: {:?}", rev_iteration, state);
    for rem in 0..=0b111 {
        let mult = 8i128.pow(rev_iteration as u32);

        if state.a + mult * rem < 8i128.pow((tape.len() as u32) - 1) {
            continue;
        }

        let mut temp_state = state.clone();
        //let a_print = state.a + mult * rem;
        temp_state.a = state.a + mult * rem;
        temp_state.run(tape);
        let result = temp_state.output.clone();
        //println!("a in binary: {a_print:048b}\na: {a_print}, result: {result:?}");
        //for _digit in 0..a_print.to_string().chars().count() {
        //    print!(" ");
        //}
        //print!("       want: {:?}\n", tape);

        //println!(
        //    "result[rev_iteration]: {} =? tape[rev_iteration]: {}",
        //    result[rev_iteration], tape[rev_iteration]
        //);
        if result[rev_iteration] == tape[rev_iteration] {
            return if rev_iteration == 0 {
                // println!(
                //     "a: {}, b: {}, c: {}",
                //     temp_state.a, temp_state.b, temp_state.c
                // );
                //println!("final result:");
                Some(state.a + mult * rem)
            } else if let Some(a) = {
                //println!("iterating again to find next correct output");
                go_backwards(
                    &State {
                        ip: 0,
                        a: state.a + mult * rem,
                        b: state.b,
                        c: state.c,
                        output: vec![],
                    },
                    rev_iteration - 1,
                    tape,
                )
            } {
                Some(a)
            } else {
                continue;
            };
        }
        //println!("iteration {iteration}, a {temp} could not find a valid output")
    }
    None
}

pub fn process_part2(input: &str) -> String {
    let (start_state, tape) = parse(input);
    let mut state = start_state.clone();
    // start with 0 and iterate from highest 3 bits of a (tape.len()*3) bit number
    state.a = 0;
    go_backwards(&state, tape.len() - 1, &tape)
        .expect("should find something")
        .to_string()
}

pub fn process_part2_slow(input: &str) -> String {
    let (start_state, tape) = parse(input);
    let mut state = start_state.clone();
    for a_lower in 0..=0b111 {
        state.a = (949978046398464 << 3) | a_lower;
        //let temp = state.a;
        state.run(&tape);
        //println!("{temp}: {:?}", state.output);
    }
    "works".to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_c_9() {
        let mut state = State {
            ip: 0,
            a: 0,
            b: 0,
            c: 9,
            output: vec![],
        };
        state.run(&vec![2, 6]);
        assert_eq!(state.b, 1);
    }
    #[test]
    fn test_a_10() {
        let mut state = State {
            ip: 0,
            a: 10,
            b: 0,
            c: 9,
            output: vec![],
        };
        state.run(&vec![5, 0, 5, 1, 5, 4]);
        assert_eq!(state.output, vec![0, 1, 2]);
    }
    #[test]
    fn test_a_2024() {
        let mut state = State {
            ip: 0,
            a: 2024,
            b: 0,
            c: 0,
            output: vec![],
        };
        state.run(&vec![0, 1, 5, 4, 3, 0]);
        assert_eq!(state.output, vec![4, 2, 5, 6, 7, 7, 7, 7, 3, 1, 0]);
        assert_eq!(state.a, 0);
    }
    #[test]
    fn test_b_29() {
        let mut state = State {
            ip: 0,
            a: 0,
            b: 29,
            c: 0,
            output: vec![],
        };
        state.run(&vec![1, 7]);
        assert_eq!(state.b, 26);
    }
    #[test]
    fn test_b_2024_c_43690() {
        let mut state = State {
            ip: 0,
            a: 0,
            b: 2024,
            c: 43690,
            output: vec![],
        };
        state.run(&vec![4, 0]);
        assert_eq!(state.b, 44354);
    }
    #[test]
    fn test_117440() {
        let mut state = State {
            ip: 0,
            a: 117440,
            b: 0,
            c: 0,
            output: vec![],
        };
        state.run(&vec![0, 3, 5, 4, 3, 0]);
        assert_eq!(state.output, vec![0, 3, 5, 4, 3, 0]);
    }
    #[test]
    fn test_input() {
        let file = include_str!("../test-input-1.txt");
        assert_eq!(process_part1(file), "4,6,3,5,6,3,5,2,1,0");
        let file2 = include_str!("../test-input-2.txt");
        assert_eq!(process_part2(file2), "117440");
    }
}
