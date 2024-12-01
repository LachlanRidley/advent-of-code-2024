use std::fs;

use crate::util::split_on_new_lines;

pub fn solve() {
    let contents = fs::read_to_string("puzzle_inputs/day_1.txt")
        .expect("Should have been able to read the file");

    let lines = split_on_new_lines(contents.as_str());

    println!("--- Day 1 ---");
    println!("Part 1 answer: {}", "TODO");
    println!("Part 2 answer: {}", "TODO");
}

#[cfg(test)]
mod tests {
    use crate::util::split_on_new_lines;

    use super::*;

    #[test]
    fn part_1() {
        let puzzle = "";

        let lines = split_on_new_lines(puzzle);
        assert_eq!(true, true);
    }

    #[test]
    fn part_2() {
        let puzzle = "";

        let lines = split_on_new_lines(puzzle);
        assert_eq!(true, true)
    }
}
