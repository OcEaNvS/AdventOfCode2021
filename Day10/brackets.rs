use std::fs;
use std::io::{self, BufRead};

fn main() {
    let input = fs::File::open("input.txt").expect("File not found");
    let lines = io::BufReader::new(input).lines().map(|l| l.unwrap()).collect::<Vec<_>>();
    let mut corrupted_line = vec![false; lines.len()];
    let mut part1_total = 0;
    let mut part2_total_vec = Vec::new();
    let mut open_bracket_vector = Vec::new();

    for i in 0..lines.len() {
        for c in lines[i].chars() {
            match c {
                '(' => open_bracket_vector.push(c),
                '[' => open_bracket_vector.push(c),
                '{' => open_bracket_vector.push(c),
                '<' => open_bracket_vector.push(c),
                _ => {
                    if let Some(bracket) = open_bracket_vector.last() {
                        if *bracket == return_matching_bracket(c) {
                            open_bracket_vector.pop();
                        } else {
                            part1_total += calculate_points_p1(c);
                            corrupted_line[i] = true;
                            open_bracket_vector.clear();
                            break;
                        }
                    } else {
                        part1_total += calculate_points_p1(c);
                        corrupted_line[i] = true;
                        open_bracket_vector.clear();
                        break;
                    }
                }
            }
        }
    }

    for i in 0..lines.len() {
        if !corrupted_line[i] {
            for c in lines[i].chars() {
                match c {
                    '(' => open_bracket_vector.push(c),
                    '[' => open_bracket_vector.push(c),
                    '{' => open_bracket_vector.push(c),
                    '<' => open_bracket_vector.push(c),
                    _ => {
                        if *open_bracket_vector.last().unwrap() == return_matching_bracket(c) {
                            open_bracket_vector.pop();
                        }
                    }
                }
            }

            let mut part2_total: usize = 0;

            for _ in 0..open_bracket_vector.len() {
                part2_total *= 5;
                part2_total += calculate_points_p2(open_bracket_vector.pop().unwrap());
            }

            part2_total_vec.push(part2_total);
        }
    }

    part2_total_vec.sort();


    println!("Part 1 Score: {}", part1_total);
    println!("Part 2 Score: {}", part2_total_vec[part2_total_vec.len() / 2]);
}

fn calculate_points_p1(bracket: char) -> i32 {
    match bracket {
        ')' => 3,
        ']' => 57,
        '}' => 1197,
        '>' => 25137,
        _ => panic!("Invalid bracket"),
    }
}

fn calculate_points_p2(bracket: char) -> usize {
    match bracket {
        '(' => 1,
        '[' => 2,
        '{' => 3,
        '<' => 4,
        _ => panic!("Invalid bracket"),
    }
}

fn return_matching_bracket(bracket: char) -> char {
    match bracket {
        ')' => '(',
        ']' => '[',
        '}' => '{',
        '>' => '<',
        _ => panic!("Invalid bracket"),
    }
}
