use std::fs;
use std::io::{self, BufRead};
use std::collections::HashSet;
use std::iter::FromIterator;

fn main() {
    let input = fs::File::open("input.txt").expect("File not found");
    let lines = io::BufReader::new(input).lines().map(|l| l.unwrap());
    let mut part1_count = 0;
    let mut part2_count = 0;

    for line in lines {
        let split = line.split(" | ").collect::<Vec<&str>>();
        let mut signal_patterns = split[0].split(" ").collect::<Vec<&str>>();
        let disp_output = split[1].split(" ").collect::<Vec<&str>>();
        let mut num_str: Vec<HashSet<char>> = vec![HashSet::new(); 10];
        let mut part2_num = String::from("");

        signal_patterns.sort_by(|a, b| a.chars().count().cmp(&b.chars().count()));

        for pattern in signal_patterns {
            let pattern_set = HashSet::from_iter(pattern.chars());
            match pattern_set.len() {
                2 => num_str[1] = pattern_set,
                3 => num_str[7] = pattern_set,
                4 => num_str[4] = pattern_set,
                5 => {
                    if pattern_set.intersection(&num_str[1]).count() == 2 {
                        num_str[3] = pattern_set;
                    } else {
                        if pattern_set.intersection(&num_str[4]).count() == 3 {
                            num_str[5] = pattern_set;
                        } else if pattern_set.intersection(&num_str[4]).count() == 2 {
                            num_str[2] = pattern_set;
                        }
                    }
                }
                6 => {
                    if pattern_set.intersection(&num_str[4]).count() == 4 {
                        num_str[9] = pattern_set;
                    } else {
                        if pattern_set.intersection(&num_str[1]).count() == 1 {
                            num_str[6] = pattern_set;
                        } else if pattern_set.intersection(&num_str[1]).count() == 2 {
                            num_str[0] = pattern_set;
                        }
                    }
                }
                7 => {
                    num_str[8] = pattern_set;
                }
                _ => {
                    println!("Error: {}", pattern);
                }
            }
        }

        for output in disp_output {
            if output.chars().count() < 5 || output.chars().count() == 7 {
                part1_count += 1;
            }

            let output_chars = HashSet::from_iter(output.chars());
            for i in 0..num_str.len() {
                if output_chars == num_str[i] {
                    part2_num.push_str(&i.to_string())
                }
            }
        }

        part2_count += part2_num.parse::<i32>().unwrap();
    }

    println!("Part 1: {}", part1_count);
    println!("Part 2: {}", part2_count);
}
