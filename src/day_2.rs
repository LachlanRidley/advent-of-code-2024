use std::fs;

use crate::util::split_on_new_lines;

pub fn solve() {
    let contents = fs::read_to_string("puzzle_inputs/day_2.txt")
        .expect("Should have been able to read the file");

    let reports = parse_reports(&contents);

    println!("--- Day 2 ---");
    println!("Part 1 answer: {}", count_safe_reports(&reports, false));
    println!("Part 2 answer: {}", count_safe_reports(&reports, true));
}

#[derive(Debug, PartialEq)]
struct Report {
    levels: Vec<i32>,
}

fn parse_reports(puzzle: &str) -> Vec<Report> {
    split_on_new_lines(puzzle)
        .iter()
        .map(|line| Report {
            levels: line
                .split(' ')
                .map(|level| level.parse::<i32>().unwrap())
                .collect::<Vec<i32>>(),
        })
        .collect()
}

fn count_safe_reports(reports: &Vec<Report>, enable_problem_dampener: bool) -> usize {
    reports
        .iter()
        .filter(|report| is_safe_report(report, enable_problem_dampener))
        .count()
}
fn is_safe_report(report: &Report, enable_problem_dampener: bool) -> bool {
    if check(&report.levels) {
        return true;
    }

    if !enable_problem_dampener {
        return false;
    }

    for i in 0..report.levels.len() {
        let mut altered_level = report.levels.clone();
        altered_level.remove(i);

        if check(&altered_level) {
            return true;
        }
    }

    return false;
}

fn is_vec_sorted<T: Ord + Clone>(v: &Vec<T>) -> bool {
    let mut sorted_vec = v.clone();
    sorted_vec.sort();

    let mut reverse_sorted_vec = v.clone();
    reverse_sorted_vec.sort();
    reverse_sorted_vec.reverse();

    return v == &sorted_vec || v == &reverse_sorted_vec;
}

fn check(levels: &Vec<i32>) -> bool {
    if !is_vec_sorted(&levels) {
        return false;
    }

    for window in levels.windows(2) {
        let [prev, next] = window else {
            return true; // there's only 0 or 1 elements, in which case it's always valid
        };
        let delta = (next - prev).abs();

        if delta < 1 || delta > 3 {
            return false;
        }
    }

    return true;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_reports() {
        let puzzle = "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9";

        let reports = parse_reports(puzzle);

        assert_eq!(
            reports,
            vec![
                Report {
                    levels: vec![7, 6, 4, 2, 1]
                },
                Report {
                    levels: vec![1, 2, 7, 8, 9]
                },
                Report {
                    levels: vec![9, 7, 6, 2, 1]
                },
                Report {
                    levels: vec![1, 3, 2, 4, 5]
                },
                Report {
                    levels: vec![8, 6, 4, 4, 1]
                },
                Report {
                    levels: vec![1, 3, 6, 7, 9]
                },
            ]
        );
    }

    #[test]
    fn part_1() {
        let puzzle = "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9";

        let reports = parse_reports(puzzle);

        assert_eq!(count_safe_reports(&reports, false), 2);
    }

    #[test]
    fn part_2() {
        let puzzle = "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9";

        let reports = parse_reports(puzzle);

        assert_eq!(count_safe_reports(&reports, true), 4);
    }
}
