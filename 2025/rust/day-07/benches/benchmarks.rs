use day_07::*;

fn main() {
    divan::main();
}

#[divan::bench]
fn part1() {
    process_part1(divan::black_box(include_str!("../input.txt")));
}
#[divan::bench]
fn part2() {
    process_part2(divan::black_box(include_str!("../input.txt")));
}
