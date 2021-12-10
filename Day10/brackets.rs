use std::fs;
use std::io::{self, BufRead};
use std::collections::HashMap;

fn main() {
    let input = fs::File::open("input.txt").expect("File not found");
    let lines = io::BufReader::new(input).lines().map(|l| l.unwrap());
    let part1_points: HashMap<char, usize> = [(')', 3), (']', 57), ('}', 1197), ('>', 25137)].iter().cloned().collect();
    let part2_points: HashMap<char, usize> = [('(', 1), ('[', 2), ('{', 3), ('<', 4)].iter().cloned().collect();
    let matching_bracket: HashMap<char, char> = [(')', '('), (']', '['), ('}', '{'), ('>', '<')].iter().cloned().collect();
    let mut part1_total = 0;
    let mut part2_total_vec = Vec::new();

    for line in lines {
        let mut open_bracket_vector = Vec::new();
        let mut corrupted = false;
        for c in line.chars() {
            match c {
                '(' => open_bracket_vector.push(c),
                '[' => open_bracket_vector.push(c),
                '{' => open_bracket_vector.push(c),
                '<' => open_bracket_vector.push(c),
                _ => {
                    if open_bracket_vector.last().unwrap() == matching_bracket.get(&c).unwrap() {
                            open_bracket_vector.pop();
                    } else {
                        part1_total += part1_points.get(&c).unwrap();
                        corrupted = true;
                        break;
                    }
                }
            }
        }

        if !corrupted {
            let mut part2_total: usize = 0;
            for _ in 0..open_bracket_vector.len() {
                part2_total *= 5;
                part2_total += part2_points.get(&open_bracket_vector.pop().unwrap()).unwrap();
            }
            part2_total_vec.push(part2_total);
        }
    }
    
    part2_total_vec.sort();
    println!("Part 1 Score: {}", part1_total);
    println!("Part 2 Score: {}", part2_total_vec[part2_total_vec.len() / 2]);
}