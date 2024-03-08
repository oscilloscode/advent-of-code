use aoc_23_01::{part1, part2};
use std::fs;

fn main() {
    // let input = fs::read_to_string("input/example.txt").expect("Cannot read file.");
    let input = fs::read_to_string("input/example2.txt").expect("Cannot read file.");
    // let input = fs::read_to_string("input/input.txt").expect("Cannot read file.");
    print!("{}", input);

    // let result_part1 = part1(&input);
    // println!("Result Part 1: {}", result_part1);

    let result_part2 = part2(&input);
    println!("Result Part 2: {}", result_part2);
}
