use std::fs;

use crate::util::split_on_new_lines;

pub fn solve() {
    let contents = fs::read_to_string("puzzle_inputs/day_1.txt")
        .expect("Should have been able to read the file");

    let lines = split_on_new_lines(contents.as_str());
    let (list_1, list_2) = parse_lists(&lines);

    println!("--- Day 1 ---");
    println!(
        "Part 1 answer: {}",
        calculate_total_distance(&list_1, &list_2)
    );
    println!(
        "Part 2 answer: {}",
        calculate_similarity_score(&list_1, &list_2)
    );
}

fn parse_lists(lines: &Vec<String>) -> (Vec<i32>, Vec<i32>) {
    let mut list_1: Vec<i32> = vec![];
    let mut list_2: Vec<i32> = vec![];

    for line in lines {
        let numbers = line.split("   ").collect::<Vec<_>>();

        list_1.push(numbers[0].parse::<i32>().unwrap());
        list_2.push(numbers[1].parse::<i32>().unwrap());
    }

    (list_1, list_2)
}

fn calculate_total_distance(list_1: &Vec<i32>, list_2: &Vec<i32>) -> i32 {
    let mut list_1_sorted = list_1.clone();
    let mut list_2_sorted = list_2.clone();

    list_1_sorted.sort();
    list_2_sorted.sort();

    let mut total_distance = 0;
    for i in 0..list_1_sorted.len() {
        let distance = (list_1_sorted[i] - list_2_sorted[i]).abs();
        total_distance += distance
    }

    total_distance
}

fn calculate_similarity_score(list_1: &Vec<i32>, list_2: &Vec<i32>) -> i32 {
    let l1 = list_1.clone();
    let l2 = list_2.clone();

    let mut total_similarity = 0;

    for num in l1 {
        let occurrences = l2.iter().filter(|&n| *n == num).count();
        let similarity = num * i32::try_from(occurrences).unwrap();
        total_similarity += similarity;
    }

    total_similarity
}

#[cfg(test)]
mod tests {
    use crate::util::split_on_new_lines;

    use super::*;

    #[test]
    fn test_parse_lists() {
        let puzzle = "3   4
4   3
2   5
1   3
3   9
3   3";

        let lines = split_on_new_lines(puzzle);
        let (list_1, list_2) = parse_lists(&lines);

        assert_eq!(list_1, vec![3, 4, 2, 1, 3, 3]);
        assert_eq!(list_2, vec![4, 3, 5, 3, 9, 3]);
    }

    #[test]
    fn part_1() {
        let puzzle = "3   4
4   3
2   5
1   3
3   9
3   3";

        let lines = split_on_new_lines(puzzle);
        let (list_1, list_2) = parse_lists(&lines);
        let total_distance = calculate_total_distance(&list_1, &list_2);

        assert_eq!(total_distance, 11);
    }

    #[test]
    fn part_2() {
        let puzzle = "3   4
4   3
2   5
1   3
3   9
3   3";

        let lines = split_on_new_lines(puzzle);
        let (list_1, list_2) = parse_lists(&lines);
        let similarity = calculate_similarity_score(&list_1, &list_2);

        assert_eq!(similarity, 31)
    }
}
