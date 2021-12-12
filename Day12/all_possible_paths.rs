use std::fs;
use std::io::{self, BufRead};
use std::fmt::Debug;

fn main() {
    let caves = read_input();
    let mut path_count_part1: u32 = 0;
    let mut path_count_part2: u32 = 0;

    count_paths(&caves, vec!["start".to_string()], &mut path_count_part1, false);
    count_paths(&caves, vec!["start".to_string()], &mut path_count_part2, true);

    println!("Part 1 Paths {}", path_count_part1);
    println!("Part 2 Paths {}", path_count_part2);
}

fn count_paths(caves: &Vec<Cave>, curr_path: Vec<String>, path_count: &mut u32, multiple_visits: bool) {
    if *curr_path.last().unwrap() == "end".to_string() {
        *path_count += 1;
    } else {
        let current_cave = caves.iter().find(|cave| cave.name == *curr_path.last().unwrap()).unwrap();
        for path in &current_cave.paths {
            let mut new_path = curr_path.clone();
            if path == "start" {continue;}
            else if path.to_uppercase() == *path {
                new_path.push(path.clone());
                count_paths(caves, new_path, path_count, multiple_visits);
            } else {
                if (multiple_visits && get_small_multiple_visits(&curr_path)) || curr_path.iter().filter(|c| *c == path).count() < 1 {
                    new_path.push(path.clone());
                    count_paths(caves, new_path, path_count, multiple_visits);
                }
            }
            
        }
    }
}

fn get_small_multiple_visits(curr_path: &Vec<String>) -> bool {
    let small_caves = curr_path.iter().filter(|c| c.to_lowercase() == **c).collect::<Vec<&String>>();
    for cave in small_caves {
        if curr_path.iter().filter(|&c| c == cave).count() > 1 {
            return false;
        }
    }
    true
}

fn read_input() -> Vec<Cave> {
    let input = fs::File::open("input.txt").expect("File not found");
    let reader = io::BufReader::new(input);
    let lines: Vec<String> = reader.lines().map(|l| l.unwrap()).collect();
    let mut caves: Vec<Cave> = Vec::new();
    for line in &lines {
        for cave in line.split("-") {
            if caves.iter().find(|c| c.name == cave).is_none() {
                caves.push(Cave::from(cave));
            }
        }  
    }

    for line in &lines {
        let mut both_caves = line.split('-');
        let cave1 = both_caves.next().unwrap();
        let cave2 = both_caves.next().unwrap();
        caves.iter_mut().filter(|c| c.name == cave1).for_each(|c| c.paths.push(cave2.to_string()));
        caves.iter_mut().filter(|c| c.name == cave2).for_each(|c| c.paths.push(cave1.to_string()));
    }
    caves
}

struct Cave {
    name: String,
    paths: Vec<String>,
}

impl From<&str> for Cave {
    fn from(s: &str) -> Self {
        Cave {
            name: s.to_string(),
            paths: Vec::new(),
        }
    }
}

impl Default for Cave {
    fn default() -> Self {
        Cave {
            name: String::new(),
            paths: Vec::new(),
        }
    }
}

impl Debug for Cave {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)
    }
}