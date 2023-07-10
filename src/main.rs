use aoc_5;
use std::fs;

fn main() {
    part_1();
    part_2();
}

fn part_1() {
    let input = fs::read_to_string("input1.txt").unwrap();
    let result = aoc_5::process_part_1(&input);
    println!("Part 1: {}", result);
}

fn part_2() {
    let input = fs::read_to_string("input1.txt").unwrap();
    let result = aoc_5::process_part_2(&input);
    println!("Part 2: {}", result);
}
